// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::{
    any::Any,
    collections::{HashMap, hash_map::Entry},
};

use arclib_graph_spec::{
    GraphContext, GraphStorageLike, Node, NodeId, PoolDepCollectorFn, PoolExecuteFn,
};

use crate::{
    base::BaseContextValue,
    utils::{collect_deps_wrapper, execute_wrapper},
};

#[derive(Default)]
pub struct BaseGraphStorage {
    pub index_map: HashMap<NodeId, (u64, usize)>,
    pub pools: HashMap<u64, Box<dyn Any + Send + Sync>>,

    pub executors: HashMap<u64, PoolExecuteFn<BaseContextValue>>,
    pub depdendency_collectors: HashMap<u64, PoolDepCollectorFn>,

    pub outgoing: HashMap<NodeId, Vec<NodeId>>,
    pub incoming: HashMap<NodeId, Vec<NodeId>>,
}

impl BaseGraphStorage {
    pub fn new() -> Self {
        Self::default()
    }

    #[track_caller]
    pub fn register_pool<T: Node<BaseContextValue>>(&mut self) {
        let type_id = T::type_id_static();
        if let Entry::Vacant(e) = self.pools.entry(type_id) {
            e.insert(Box::new(Vec::<T>::new()));
            self.executors
                .insert(type_id, execute_wrapper::<BaseContextValue, T>);
            self.depdendency_collectors
                .insert(type_id, collect_deps_wrapper::<BaseContextValue, T>);
        }
    }

    pub fn execute_node(
        &mut self,
        type_id: u64,
        index: usize,
        ctx: &mut GraphContext<'_, BaseContextValue>,
    ) {
        let pool = self
            .pools
            .get_mut(&type_id)
            .expect("Pool not registered for execution");
        let executor = self
            .executors
            .get(&type_id)
            .expect("Executor not registered for type_id");

        executor(pool, index, ctx);
    }

    pub fn add_node<T: Node<BaseContextValue>>(&mut self, node: T) -> NodeId {
        let id = *node.id();
        let type_id = T::type_id_static();

        let pool = self
            .pools
            .get_mut(&T::type_id_static())
            .expect("Pool not registered. Call register_pool::<T>() first.");
        let vec = pool
            .downcast_mut::<Vec<T>>()
            .expect("Type mismatch in pool");

        let index = vec.len();
        vec.push(node);
        self.index_map.insert(id, (type_id, index));

        id
    }

    pub fn connect(&mut self, source: NodeId, target: NodeId) {
        self.outgoing.entry(source).or_default().push(target);
        self.incoming.entry(target).or_default().push(source);
    }
}

impl GraphStorageLike<BaseContextValue> for BaseGraphStorage {
    fn index_map(&self) -> &HashMap<NodeId, (u64, usize)> {
        &self.index_map
    }

    fn pools(&self) -> &HashMap<u64, Box<dyn Any + Send + Sync>> {
        &self.pools
    }

    fn executors(&self) -> &HashMap<u64, arclib_graph_spec::PoolExecuteFn<BaseContextValue>> {
        &self.executors
    }

    fn dependency_collectors(&self) -> &HashMap<u64, arclib_graph_spec::PoolDepCollectorFn> {
        &self.depdendency_collectors
    }

    fn outgoing(&self) -> &HashMap<NodeId, Vec<NodeId>> {
        &self.outgoing
    }

    fn incoming(&self) -> &HashMap<NodeId, Vec<NodeId>> {
        &self.incoming
    }
}
