// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::collections::HashMap;

use crate::NodeId;

pub trait ContextValueLike: std::fmt::Display + Clone {}

pub struct GraphContext<'a, V: ContextValueLike> {
    pub values: &'a mut HashMap<NodeId, V>,
}

impl<'a, V: ContextValueLike> GraphContext<'a, V> {
    pub fn new(values: &'a mut HashMap<NodeId, V>) -> Self {
        Self { values }
    }

    pub fn get(&self, id: &NodeId) -> Option<&V> {
        self.values.get(id)
    }
}
