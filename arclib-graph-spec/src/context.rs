// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::collections::HashMap;

use crate::NodeId;

pub trait ContextValueLike: std::fmt::Display + Clone {}

pub struct GraphContext<'a, V: ContextValueLike> {
    pub temp: &'a mut HashMap<NodeId, V>,
    pub state: &'a HashMap<NodeId, V>,
    pub next_state: &'a mut HashMap<NodeId, V>,
}

impl<'a, V: ContextValueLike> GraphContext<'a, V> {
    pub fn new(
        temp: &'a mut HashMap<NodeId, V>,
        state: &'a HashMap<NodeId, V>,
        next_state: &'a mut HashMap<NodeId, V>,
    ) -> Self {
        Self {
            temp,
            state,
            next_state,
        }
    }

    pub fn get(&self, id: &NodeId) -> Option<&V> {
        self.state.get(id)
    }
}
