// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_impl::{BaseGraph, BaseNode, BaseNodeKind, DType, Payload};
use arclib_graph_spec::Graph;
use half::f16;

#[test]
fn graph_creation() {
    let mut graph = BaseGraph::new();
    graph.register_pool::<BaseNode>();

    let base_node = BaseNode::new(
        BaseNodeKind::Scalar,
        DType::F16,
        Payload::ScalarF16(f16::from_f32(16.0)),
    );
    let base_node_id = base_node.id().clone();
    graph.add_node(base_node);

    let res: &BaseNode = graph.get_node(&base_node_id).unwrap();

    assert_eq!(res.as_f32(), Some(16.0));
}
