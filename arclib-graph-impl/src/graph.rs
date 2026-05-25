// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::{collections::HashMap, fmt::Debug};

use arclib_graph_spec::{ContextValueLike, NodeId};

use crate::{schedule::Schedule, storage::GraphStorage};

pub struct Graph<V: ContextValueLike> {
    pub storage: GraphStorage<V>,
    pub schedule: Option<Schedule>,
    pub values_map: HashMap<NodeId, V>,
    pub node_names: HashMap<NodeId, String>,
}

impl<V: ContextValueLike> Graph<V> {
    pub fn new() -> Self {
        Self {
            storage: GraphStorage {
                index_map: HashMap::new(),
                pools: HashMap::new(),
                outgoing: HashMap::new(),
                incoming: HashMap::new(),
                executors: HashMap::new(),
                dependency_collectors: HashMap::new(),
            },
            schedule: None,
            values_map: HashMap::new(),
            node_names: HashMap::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.storage.index_map.len()
    }
}

impl<V: ContextValueLike> Default for Graph<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: ContextValueLike> Debug for Graph<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        out.push_str("--- Graph ---\n");
        if let Some(schedule) = &self.schedule {
            for (i, (tid, idx)) in schedule.execution_queue.iter().enumerate() {
                let name = self
                    .storage
                    .index_map
                    .iter()
                    .find_map(|(id, &(t, i))| {
                        if t == *tid && i == *idx {
                            Some(id)
                        } else {
                            None
                        }
                    })
                    .and_then(|id| self.node_names.get(id))
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                out.push_str(&format!(
                    "[{:04}] {} (TypeID: {}, PoolIdx: {})\n",
                    i, name, tid, idx
                ));
            }
        } else {
            out.push_str("(Not compiled)\n");
        }

        out.push_str("\n--- Connections ---\n");
        for (src, targets) in &self.storage.outgoing {
            let src_name = self
                .node_names
                .get(src)
                .map(|s| s.as_str())
                .unwrap_or("Unknown");
            let tgt_names: Vec<&str> = targets
                .iter()
                .map(|id| {
                    self.node_names
                        .get(id)
                        .map(|s| s.as_str())
                        .unwrap_or("Unknown")
                })
                .collect();
            out.push_str(&format!("{} -> {:?}\n", src_name, tgt_names));
        }

        write!(f, "{}", out)
    }
}
