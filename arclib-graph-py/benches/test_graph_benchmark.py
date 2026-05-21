# Copyright (c) 2026 ARC (Applied Research & Computation)
# SPDX-License-Identifier: LGPL-2.1-or-later

from arclib_graph import BaseGraph


def test_graph_benchmark(benchmark):
    def create():
        graph = BaseGraph()
        graph.add_base_node(64.0)
        return 0

    result = benchmark(create)
    assert result == 0
