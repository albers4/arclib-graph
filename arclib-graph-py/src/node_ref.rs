// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_spec::ContextValue;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::base_graph::PyBaseGraph;

#[pyclass]
pub struct PyNodeRef {
    pub id: uuid::Uuid,
    pub type_name: String,
    pub dtype: Option<String>,
    pub graph: Py<PyBaseGraph>,
}

#[pymethods]
impl PyNodeRef {
    #[getter]
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn __repr__(&self) -> PyResult<String> {
        let dtype_str = self.dtype.as_deref().unwrap_or("N/A");
        Ok(format!(
            "PyNodeRef(id={}, type={}, dtype={:?})",
            self.id, self.type_name, dtype_str
        ))
    }

    #[getter]
    fn dtype(&self) -> String {
        let dtype_str = self.dtype.as_deref().unwrap_or("N/A");
        dtype_str.to_string()
    }

    #[getter]
    fn value<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let graph_ref = self.graph.borrow(py);

        match graph_ref.inner.values_map.get(&self.id) {
            Some(ContextValue::ScalarF16(v)) => Ok(v.to_f32().into_pyobject(py)?.into_any()),
            Some(ContextValue::ScalarF32(v)) => Ok(v.into_pyobject(py)?.into_any()),
            Some(ContextValue::ScalarF64(v)) => Ok(v.into_pyobject(py)?.into_any()),
            Some(ContextValue::Symbol(s)) => Ok(s.as_ref().into_pyobject(py)?.into_any()),
            Some(ContextValue::Empty) | None => Err(PyValueError::new_err(
                "Node has no output value yet. Ensure graph.compile() and graph.step() have been called.",
            )),
        }
    }
}
