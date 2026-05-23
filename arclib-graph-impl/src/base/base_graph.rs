// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::collections::HashMap;

use arclib_graph_spec::NodeId;

use crate::{
    base::{BaseContextValue, base_storage::BaseGraphStorage},
    schedule::Schedule,
};

pub struct BaseGraph {
    pub storage: BaseGraphStorage,
    pub schedule: Option<Schedule>,
    pub values_map: HashMap<NodeId, BaseContextValue>,
}

impl BaseGraph {
    pub fn new() -> Self {
        Self {
            storage: BaseGraphStorage {
                index_map: HashMap::new(),
                pools: HashMap::new(),
                outgoing: HashMap::new(),
                incoming: HashMap::new(),
                executors: HashMap::new(),
                depdendency_collectors: HashMap::new(),
            },
            schedule: None,
            values_map: HashMap::new(),
        }
    }

    pub fn length(&self) -> usize {
        self.storage.index_map.len()
    }
}

impl Default for BaseGraph {
    fn default() -> Self {
        Self::new()
    }
}
