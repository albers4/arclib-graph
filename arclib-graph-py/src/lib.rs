// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

mod base_graph;
mod node_ref;
mod node_wrapper;

use base_graph::PyBaseGraph;
use node_ref::PyNodeRef;

use pyo3::prelude::*;
use pyo3::types::PyModule;

#[pymodule]
fn arclib_graph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBaseGraph>()?;
    m.add_class::<PyNodeRef>()?;
    Ok(())
}
