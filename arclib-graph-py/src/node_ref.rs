// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_impl::{BaseNode, Payload};
use arclib_graph_spec::Graph;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::base_graph::PyBaseGraph;

#[pyclass]
pub struct PyNodeRef {
    pub id: uuid::Uuid,
    pub graph: Py<PyBaseGraph>,
}

#[pymethods]
impl PyNodeRef {
    #[getter]
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let graph_ref = self.graph.borrow(py);
        let node: &BaseNode = graph_ref
            .inner
            .get_node(&self.id)
            .ok_or_else(|| PyValueError::new_err("Node not found in graph"))?;

        Ok(format!("PyNodeRef(id={}, dtype={})", self.id, node.dtype()))
    }

    #[getter]
    fn dtype(&self, py: Python) -> PyResult<String> {
        let graph_ref = self.graph.borrow(py);
        let node: &BaseNode = graph_ref
            .inner
            .get_node(&self.id)
            .ok_or_else(|| PyValueError::new_err("Node not found in graph"))?;

        Ok(node.dtype().to_string())
    }

    #[getter]
    fn value<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let graph_ref = self.graph.borrow(py);
        let node_ref: &BaseNode = graph_ref
            .inner
            .get_node(&self.id)
            .ok_or_else(|| PyValueError::new_err("Node not found in graph"))?;

        if let Some(_node) = graph_ref.inner.get_node::<BaseNode>(&self.id) {
            match &node_ref.payload {
                Payload::ScalarF16(v) => Ok(v.to_f32().into_pyobject(py)?.into_any()),
                Payload::ScalarF32(v) => Ok(v.into_pyobject(py)?.into_any()),
                Payload::ScalarF64(v) => Ok(v.into_pyobject(py)?.into_any()),
                Payload::Symbol(s) => Ok(s.as_str().into_pyobject(py)?.into_any()),
            }
        } else {
            Err(PyValueError::new_err(
                "Cannot get value: Not a BaseNode or Node not found",
            ))
        }
    }
}
