// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::{
    any::Any,
    collections::{HashMap, hash_map::Entry},
};

use arclib_graph_spec::{ContextValue, GraphContext, Node, NodeId};

use crate::{schedule::Schedule, topological_sort};

pub type PoolExecuteFn = fn(&mut Box<dyn Any + Send + Sync>, usize, &mut GraphContext);

#[derive(Default)]
pub struct BaseGraphStorage {
    pub index_map: HashMap<NodeId, (u64, usize)>,
    pub pools: HashMap<u64, Box<dyn Any + Send + Sync>>,

    pub executors: HashMap<u64, PoolExecuteFn>,

    pub outgoing: HashMap<NodeId, Vec<NodeId>>,
    pub incoming: HashMap<NodeId, Vec<NodeId>>,
}

impl BaseGraphStorage {
    pub fn new() -> Self {
        Self::default()
    }

    #[track_caller]
    fn register_pool<T: Node>(&mut self) {
        let type_id = T::type_id_static();
        if let Entry::Vacant(e) = self.pools.entry(type_id) {
            e.insert(Box::new(Vec::<T>::new()));
            self.executors.insert(type_id, execute_wrapper::<T>);
        }
    }

    pub fn execute_node(&mut self, type_id: u64, index: usize, ctx: &mut GraphContext) {
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

    fn add_node<T: Node>(&mut self, node: T) -> NodeId {
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
}

fn execute_wrapper<T: Node>(
    pool: &mut Box<dyn Any + Send + Sync>,
    index: usize,
    ctx: &mut GraphContext,
) {
    let vec = pool
        .downcast_mut::<Vec<T>>()
        .expect("Executor type mismatch");
    vec[index].compute(ctx);
}

pub struct BaseGraph {
    pub storage: BaseGraphStorage,
    pub schedule: Option<Schedule>,
    pub values_map: HashMap<NodeId, ContextValue>,
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
            },
            schedule: None,
            values_map: HashMap::new(),
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

    pub fn register_pool<T: Node>(&mut self) {
        self.storage.register_pool::<T>();
    }

    pub fn add_node<T: Node>(&mut self, node: T) -> NodeId {
        self.storage.add_node(node)
    }

    pub fn compile(&mut self) -> Result<(), String> {
        let order = topological_sort(&self.storage)?;
        let mut queue = Vec::with_capacity(order.len());

        for id in &order {
            let &(type_id, index) = self
                .storage
                .index_map
                .get(id)
                .ok_or(format!("Node {} not found", id))?;
            queue.push((type_id, index));
        }

        self.schedule = Some(Schedule::new(queue));
        self.values_map.clear();
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), String> {
        let schedule = self
            .schedule
            .as_ref()
            .ok_or("Graph not compiled".to_string())?;
        let mut ctx = GraphContext::new(&mut self.values_map);

        for &(type_id, index) in &schedule.execution_queue {
            self.storage.execute_node(type_id, index, &mut ctx);
        }

        Ok(())
    }
}

impl Default for BaseGraph {
    fn default() -> Self {
        Self::new()
    }
}
