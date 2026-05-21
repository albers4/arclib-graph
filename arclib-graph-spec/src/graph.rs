// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use uuid::Uuid;

pub type NodeId = Uuid;

pub trait Node: 'static + Send + Sync {
    const TYPE_ID: u64;

    fn id(&self) -> &NodeId;
}

pub trait Graph {
    fn get_node<T: Node>(&self, id: &NodeId) -> Option<&T>;
    fn get_node_mut<T: Node>(&mut self, id: &NodeId) -> Option<&mut T>;

    fn iter<T: Node>(&self) -> impl Iterator<Item = &T> + '_;
    fn iter_mut<T: Node>(&mut self) -> impl Iterator<Item = &mut T> + '_;

    fn register_pool<T: Node>(&mut self);
    fn add_node<T: Node>(&mut self, node: T) -> NodeId;
    fn contains(&self, id: &NodeId) -> bool;
}
