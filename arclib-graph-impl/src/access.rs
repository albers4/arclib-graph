// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_spec::{ContextValueLike, GraphContext, GraphLike, Node, NodeId};

use crate::{graph::Graph, schedule::Schedule, topological_sort};

impl<V: ContextValueLike> GraphLike<V> for Graph<V> {
    fn get_node<T: Node<V>>(&self, id: &NodeId) -> Option<&T> {
        let &(type_id, index) = self.storage.index_map.get(id)?;
        if type_id != T::type_id_static() {
            return None;
        }
        self.storage
            .pools
            .get(&type_id)
            .and_then(|p| p.downcast_ref::<Vec<T>>())
            .and_then(|v| v.get(index))
    }

    fn get_node_mut<T: Node<V>>(&mut self, id: &NodeId) -> Option<&mut T> {
        let &(type_id, index) = self.storage.index_map.get(id)?;
        if type_id != T::type_id_static() {
            return None;
        }
        self.storage
            .pools
            .get_mut(&type_id)
            .and_then(|p| p.downcast_mut::<Vec<T>>())
            .and_then(|v| v.get_mut(index))
    }

    fn iter<T: Node<V>>(&self) -> impl Iterator<Item = &T> + '_ {
        let pool = self
            .storage
            .pools
            .get(&T::type_id_static())
            .expect("Pool not registered");
        pool.downcast_ref::<Vec<T>>().expect("Type mismatch").iter()
    }

    fn iter_mut<T: Node<V>>(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        let pool = self
            .storage
            .pools
            .get_mut(&T::type_id_static())
            .expect("Pool not registered");
        pool.downcast_mut::<Vec<T>>()
            .expect("Type mismatch")
            .iter_mut()
    }

    fn compile(&mut self) -> Result<(), String> {
        self.validate_inputs()?;
        self.storage.build_dependency_edges();
        let order = topological_sort::<V>(&self.storage)?;

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

    fn connect(&mut self, source: NodeId, target: NodeId) -> Result<(), String> {
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

    fn register_pool<T: Node<V>>(&mut self) {
        self.storage.register_pool::<T>();
    }

    fn add_node<T: Node<V>>(&mut self, node: T) -> NodeId {
        self.node_names.insert(*node.id(), node.name().to_string());
        self.storage.add_node(node)
    }

    fn validate_inputs(&self) -> Result<(), String> {
        let mut edges = Vec::new();
        let mut missing = Vec::new();

        for (&_, &(type_id, index)) in &self.storage.index_map {
            if let Some(collector) = self.storage.dependency_collectors.get(&type_id)
                && let Some(pool) = self.storage.pools.get(&type_id)
            {
                collector(pool, index, &mut edges);
            }

            for (src_id, _) in edges.drain(..) {
                if !self.storage.index_map.contains_key(&src_id) {
                    missing.push(src_id);
                }
            }
        }

        if missing.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "Validation failed: {} missing input node(s): {:?}",
                missing.len(),
                missing
            ))
        }
    }

    fn step(&mut self) -> Result<(), String> {
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
