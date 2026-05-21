// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_impl::fnv1a_hash;
use arclib_graph_spec::{ContextValue, GraphContext, Node, NodeId};
use pyo3::prelude::*;
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

impl Node for PyNodeWrapper {
    fn type_id_static() -> u64
    where
        Self: Sized,
    {
        fnv1a_hash("PyNodeWrapper")
    }

    fn id(&self) -> &NodeId {
        &self.id
    }

    fn compute(&mut self, ctx: &mut GraphContext) {
        Python::attach(|py| {
            let result = self.py_instance.call_method0(py, "compute");

            match result {
                Ok(output_py) => {
                    let output_bound = output_py.bind(py);
                    if let Some(rust_value) = convert_py_to_value(output_bound) {
                        ctx.values.insert(self.id, rust_value);
                    }
                }
                Err(e) => {
                    e.print(py);
                }
            }
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn as_node_mut(&mut self) -> &mut dyn Node {
        self
    }
}

fn convert_py_to_value(obj: &Bound<'_, PyAny>) -> Option<ContextValue> {
    if let Ok(f) = obj.extract::<f64>() {
        return Some(ContextValue::ScalarF64(f));
    }
    if let Ok(f) = obj.extract::<f32>() {
        return Some(ContextValue::ScalarF32(f));
    }
    None
}
