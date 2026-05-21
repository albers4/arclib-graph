// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::{any::Any, collections::HashMap};

use arclib_graph_spec::NodeId;

use crate::schedule::Schedule;

pub struct BaseGraphStorage {
    pub index_map: HashMap<NodeId, (u64, usize)>,
    pub pools: HashMap<u64, Box<dyn Any + Send + Sync>>,
    pub outgoing: HashMap<NodeId, Vec<NodeId>>,
    pub incoming: HashMap<NodeId, Vec<NodeId>>,
}

pub struct BaseGraph {
    pub storage: BaseGraphStorage,
    pub schedule: Option<Schedule>,
}

impl BaseGraph {
    pub fn new() -> Self {
        Self {
            storage: BaseGraphStorage {
                index_map: HashMap::new(),
                pools: HashMap::new(),
                outgoing: HashMap::new(),
                incoming: HashMap::new(),
            },
            schedule: None,
        }
    }

    pub fn connect(&mut self, source: NodeId, target: NodeId) {
        self.storage
            .outgoing
            .entry(source)
            .or_default()
            .push(target);
        self.storage
            .incoming
            .entry(target)
            .or_default()
            .push(source);
    }
}

impl Default for BaseGraph {
    fn default() -> Self {
        Self::new()
    }
}
