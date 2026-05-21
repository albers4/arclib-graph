// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_impl::fnv1a_hash;
use arclib_graph_spec::{Node, NodeId};
use pyo3::{Py, PyAny, Python};

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
    const TYPE_ID: u64 = fnv1a_hash("PyNodeWrapper");

    fn id(&self) -> &NodeId {
        &self.id
    }
}
