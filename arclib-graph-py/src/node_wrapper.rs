// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_impl::{BaseContextValue, fnv1a_hash};
use arclib_graph_spec::{GraphContext, Node, NodeId};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::{Bound, Py, PyAny, Python};

pub struct PyNodeWrapper {
    pub id: NodeId,
    pub py_instance: Py<PyAny>,
}

impl Clone for PyNodeWrapper {
    fn clone(&self) -> Self {
        Python::attach(|py| Self {
            id: self.id,
            py_instance: self.py_instance.clone_ref(py),
        })
    }
}

impl Node<BaseContextValue> for PyNodeWrapper {
    fn type_id_static() -> u64
    where
        Self: Sized,
    {
        fnv1a_hash("PyNodeWrapper")
    }

    fn id(&self) -> &NodeId {
        &self.id
    }

    fn compute(&mut self, ctx: &mut GraphContext<'_, BaseContextValue>) {
        Python::attach(|py| match self.py_instance.call_method0(py, "compute") {
            Ok(py_obj) => {
                let bound = py_obj.bind(py);
                if let Some(value) = py_extract_compute(bound) {
                    ctx.values.insert(self.id, value);
                }
            }
            Err(e) => {
                e.print(py);
            }
        });
    }

    fn dependencies(&self) -> Vec<NodeId> {
        Python::attach(
            |py| match self.py_instance.call_method0(py, "dependencies") {
                Ok(py_obj) => {
                    let bound = py_obj.bind(py);

                    match py_extract_dependencies(bound) {
                        Ok(deps) => deps,
                        Err(e) => {
                            e.print(py);
                            Vec::new()
                        }
                    }
                }
                Err(e) => {
                    e.print(py);
                    Vec::new()
                }
            },
        )
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Node<BaseContextValue>> {
        Box::new(self.clone())
    }

    fn as_node_mut(&mut self) -> &mut dyn Node<BaseContextValue> {
        self
    }
}

fn py_extract_compute(obj: &Bound<'_, PyAny>) -> Option<BaseContextValue> {
    if let Ok(f) = obj.extract::<f64>() {
        return Some(BaseContextValue::ScalarF64(f));
    }
    if let Ok(f) = obj.extract::<f32>() {
        return Some(BaseContextValue::ScalarF32(f));
    }
    None
}

fn py_extract_dependencies(obj: &Bound<'_, PyAny>) -> PyResult<Vec<NodeId>> {
    if let Ok(uuid_str) = obj.extract::<String>() {
        let id = NodeId::parse_str(&uuid_str)
            .map_err(|e| PyValueError::new_err(format!("Invalid UUID: {}", e)))?;
        return Ok(vec![id]);
    }

    if let Ok(list) = obj.cast::<PyList>() {
        let mut deps = Vec::with_capacity(list.len());
        for item in list.iter() {
            let uuid_str: String = item.extract::<String>()?;
            let id = NodeId::parse_str(&uuid_str)
                .map_err(|e| PyValueError::new_err(format!("Invalid UUID in list: {}", e)))?;
            deps.push(id);
        }
        return Ok(deps);
    }

    Err(PyTypeError::new_err(
        "dependencies() must return a string or list of UUID strings",
    ))
}
