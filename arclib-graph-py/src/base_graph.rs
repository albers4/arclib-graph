// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_impl::{BaseGraph, BaseNode, BaseNodeKind, DType, Payload};
use arclib_graph_spec::GraphLike;
use half::f16;
use pyo3::{
    exceptions::{PyRuntimeError, PyTypeError, PyValueError},
    prelude::*,
};

use crate::{node_ref::PyNodeRef, node_wrapper::PyNodeWrapper};

#[pyclass]
pub struct PyBaseGraph {
    pub inner: BaseGraph,
}

#[pymethods]
impl PyBaseGraph {
    #[new]
    fn new() -> Self {
        let mut g = BaseGraph::new();
        g.register_pool::<BaseNode>();
        //g.register_pool::<ComputeNode>();
        g.register_pool::<PyNodeWrapper>();

        Self { inner: g }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("PyBaseGraph(nodes={})", self.inner.length()))
    }

    fn add_custom_node(
        mut slf: PyRefMut<'_, Self>,
        node_obj: &Bound<'_, PyAny>,
    ) -> PyResult<PyNodeRef> {
        if !node_obj.hasattr("id")? {
            return Err(PyTypeError::new_err("Object must have an 'id' attribute"));
        }

        let id_str: String = node_obj.getattr("id")?.extract()?;
        let id = uuid::Uuid::parse_str(&id_str)
            .map_err(|e| PyValueError::new_err(format!("Invalid UUID: {}", e)))?;

        let type_name = node_obj
            .getattr("__class__")?
            .getattr("__name__")?
            .extract::<String>()?;

        let wrapper = PyNodeWrapper {
            id,
            py_instance: node_obj.clone().unbind(),
        };

        slf.inner.add_node(wrapper);

        let graph_handle: Py<PyBaseGraph> = slf.into();
        Ok(PyNodeRef {
            id,
            type_name,
            dtype: None,
            graph: graph_handle,
        })
    }

    #[pyo3(signature = (value, kind="scalar", dtype="f64"))]
    fn add_base_node(
        mut slf: PyRefMut<'_, Self>,
        value: &Bound<'_, PyAny>,
        kind: &str,
        dtype: &str,
    ) -> PyResult<PyNodeRef> {
        let parsed_kind = parse_kind(kind)?;
        let parsed_dtype = parse_dtype(dtype)?;

        if parsed_kind == BaseNodeKind::Symbol && parsed_dtype != DType::String {
            return Err(PyValueError::new_err("Symbol kind requires 'String' dtype"));
        }
        if parsed_kind == BaseNodeKind::Scalar && parsed_dtype == DType::String {
            return Err(PyValueError::new_err(
                "Scalar kind cannot use 'String' dtype",
            ));
        }

        let payload = extract_payload(value, &parsed_dtype)?;

        let node = BaseNode::new(parsed_kind, parsed_dtype, payload);
        let id = *node.id();

        slf.inner.add_node(node);

        let graph_handle: Py<PyBaseGraph> = slf.into();
        Ok(PyNodeRef {
            id,
            type_name: "BaseNode".to_string(),
            dtype: Some(dtype.to_string()),
            graph: graph_handle,
        })
    }

    fn compile(&mut self) -> PyResult<()> {
        self.inner
            .compile()
            .map_err(|e| PyRuntimeError::new_err(format!("Compilation failed: {}", e)))
    }

    fn step(&mut self) -> PyResult<()> {
        self.inner
            .step()
            .map_err(|e| PyRuntimeError::new_err(format!("Step failed: {}", e)))
    }
}

fn parse_kind(kind: &str) -> PyResult<BaseNodeKind> {
    match kind {
        "scalar" => Ok(BaseNodeKind::Scalar),
        "symbol" => Ok(BaseNodeKind::Symbol),
        _ => Err(PyValueError::new_err(format!(
            "kind: {} doesnt exist.",
            kind
        ))),
    }
}

fn parse_dtype(dtype: &str) -> PyResult<DType> {
    match dtype {
        "f16" => Ok(DType::F16),
        "f32" => Ok(DType::F32),
        "f64" => Ok(DType::F64),
        "string" => Ok(DType::String),
        _ => Err(PyValueError::new_err(format!(
            "dtype: {} doesnt exist.",
            dtype
        ))),
    }
}

fn extract_payload(value: &Bound<'_, PyAny>, dtype: &DType) -> PyResult<Payload> {
    match dtype {
        DType::F16 => {
            let val_f64: f64 = value.extract()?;
            Ok(Payload::ScalarF16(f16::from_f64(val_f64)))
        }
        DType::F32 => {
            let val_f64: f64 = value.extract()?;
            Ok(Payload::ScalarF32(val_f64 as f32))
        }
        DType::F64 => {
            let val_f64: f64 = value.extract()?;
            Ok(Payload::ScalarF64(val_f64))
        }
        DType::String => {
            let s: String = value.extract()?;
            Ok(Payload::Symbol(Box::new(s)))
        }
    }
}
