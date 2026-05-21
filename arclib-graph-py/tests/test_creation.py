# Copyright (c) 2026 ARC (Applied Research & Computation)
# SPDX-License-Identifier: LGPL-2.1-or-later

from arclib_graph import BaseGraph, CustomNode


def test_node_creation():
    graph = BaseGraph()

    s_node_f16 = graph.add_base_node(16.0, dtype="f16")
    s_node_f32 = graph.add_base_node(32.0, dtype="f32")
    s_node_f64 = graph.add_base_node(64.0)
    s_node_symbol = graph.add_base_node("x", kind="symbol", dtype="string")

    # print(s_node_symbol)

    assert s_node_f16.dtype == "f16"
    assert s_node_f32.dtype == "f32"
    assert s_node_f64.dtype == "f64"
    assert s_node_symbol.dtype == "String"

    assert s_node_f16.value == 16.0
    assert s_node_f32.value == 32.0
    assert s_node_f64.value == 64.0
    assert s_node_symbol.value == "x"


def test_custom_node_creation():
    graph = BaseGraph()

    class CounterNode(CustomNode):
        def __init__(self):
            super().__init__()
            self.count = 0

    my_counter = CounterNode()

    graph.add_custom_node(my_counter)
