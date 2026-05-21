// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::collections::{HashMap, VecDeque};

use arclib_graph_spec::NodeId;

use crate::BaseGraphStorage;

pub struct Schedule {
    pub execution_queue: Vec<(u64, usize)>,
}

impl Schedule {
    pub fn new(execution_queue: Vec<(u64, usize)>) -> Self {
        Self { execution_queue }
    }
}

pub fn topological_sort(storage: &BaseGraphStorage) -> Result<Vec<NodeId>, String> {
    let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
    let mut queue: VecDeque<NodeId> = VecDeque::new();
    let mut sorted: Vec<NodeId> = Vec::new();

    for id in storage.index_map.keys() {
        let deg = storage.incoming.get(id).map(|v| v.len()).unwrap_or(0);
        in_degree.insert(*id, deg);
        if deg == 0 {
            queue.push_back(*id);
        }
    }

    while let Some(node_id) = queue.pop_front() {
        sorted.push(node_id);

        if let Some(targets) = storage.outgoing.get(&node_id) {
            for target_id in targets {
                if let Some(deg) = in_degree.get_mut(target_id) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(*target_id);
                    }
                }
            }
        }
    }

    if sorted.len() != storage.index_map.len() {
        return Err("Cycle detected in graph".to_string());
    }

    Ok(sorted)
}
