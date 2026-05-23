// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::collections::HashMap;

use arclib_graph_spec::{ContextValueLike, NodeId};

use crate::{
    schedule::Schedule,
    storage::GraphStorage,
};

pub struct Graph<V: ContextValueLike> {
    pub storage: GraphStorage<V>,
    pub schedule: Option<Schedule>,
    pub values_map: HashMap<NodeId, V>,
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
