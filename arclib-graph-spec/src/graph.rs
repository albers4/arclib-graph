// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::any::Any;

use uuid::Uuid;

use crate::context::GraphContext;

pub type NodeId = Uuid;

pub trait Node: 'static + Send + Sync {
    fn type_id_static() -> u64
    where
        Self: Sized;
    fn id(&self) -> &NodeId;
    fn compute(&mut self, ctx: &mut GraphContext);

    fn as_node_mut(&mut self) -> &mut dyn Node;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &dyn Any;

    fn clone_box(&self) -> Box<dyn Node>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait Graph {
    fn get_node<T: Node>(&self, id: &NodeId) -> Option<&T>;
    fn get_node_mut<T: Node>(&mut self, id: &NodeId) -> Option<&mut T>;

    fn iter<T: Node>(&self) -> impl Iterator<Item = &T> + '_;
    fn iter_mut<T: Node>(&mut self) -> impl Iterator<Item = &mut T> + '_;
}
