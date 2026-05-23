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
    schedule::Schedule,
    topological_sort,
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
}

impl GraphStorageLike<BaseContextValue> for BaseGraphStorage {
    #[track_caller]
    fn register_pool<T: Node<BaseContextValue>>(&mut self) {
        let type_id = T::type_id_static();
        if let Entry::Vacant(e) = self.pools.entry(type_id) {
            e.insert(Box::new(Vec::<T>::new()));
            self.executors
                .insert(type_id, execute_wrapper::<BaseContextValue, T>);
            self.depdendency_collectors
                .insert(type_id, collect_deps_wrapper::<BaseContextValue, T>);
        }
    }

    fn execute_node(
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

    fn add_node<T: Node<BaseContextValue>>(&mut self, node: T) -> NodeId {
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

    fn connect(&mut self, source: NodeId, target: NodeId) {
        self.outgoing.entry(source).or_default().push(target);
        self.incoming.entry(target).or_default().push(source);
    }

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

    pub fn connect(&mut self, source: NodeId, target: NodeId) -> Result<(), String> {
        if !self.storage.index_map.contains_key(&source) {
            return Err(format!("Source node {} not found", source));
        }
        if !self.storage.index_map.contains_key(&target) {
            return Err(format!("Target node {} not found", target));
        }

        self.storage.connect(source, target);
        self.schedule = None;

        Ok(())
    }

    pub fn register_pool<T: Node<BaseContextValue>>(&mut self) {
        self.storage.register_pool::<T>();
    }

    pub fn add_node<T: Node<BaseContextValue>>(&mut self, node: T) -> NodeId {
        self.storage.add_node(node)
    }

    pub fn compile(&mut self) -> Result<(), String> {
        self.validate_inputs()?;

        let order = topological_sort::<BaseContextValue>(&self.storage)?;

        let mut queue = Vec::with_capacity(order.len());
        for id in &order {
            let &(type_id, index) = self
                .storage
                .index_map
                .get(id)
                .ok_or(format!("Node {} missing from storage", id))?;
            queue.push((type_id, index));
        }

        self.schedule = Some(Schedule::new(queue));
        self.values_map.clear();
        Ok(())
    }

    fn validate_inputs(&self) -> Result<(), String> {
        let mut all_deps = Vec::new();

        for (&type_id, pool) in &self.storage.pools {
            if let Some(collector) = self.storage.depdendency_collectors.get(&type_id) {
                collector(pool, &mut all_deps);
            }
        }

        let missing: Vec<NodeId> = all_deps
            .into_iter()
            .filter(|id| !self.storage.index_map.contains_key(id))
            .collect();

        if !missing.is_empty() {
            return Err(format!(
                "Validation failed: {} missing input node(s): {:?}",
                missing.len(),
                missing
            ));
        }

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
