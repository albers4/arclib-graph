// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_spec::{GraphLike, Node, NodeId};

use crate::{BaseGraph, base::BaseContextValue};

impl GraphLike<BaseContextValue> for BaseGraph {
    fn get_node<T: Node<BaseContextValue>>(&self, id: &NodeId) -> Option<&T> {
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

    fn get_node_mut<T: Node<BaseContextValue>>(&mut self, id: &NodeId) -> Option<&mut T> {
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

    fn iter<T: Node<BaseContextValue>>(&self) -> impl Iterator<Item = &T> + '_ {
        let pool = self
            .storage
            .pools
            .get(&T::type_id_static())
            .expect("Pool not registered");
        pool.downcast_ref::<Vec<T>>().expect("Type mismatch").iter()
    }

    fn iter_mut<T: Node<BaseContextValue>>(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        let pool = self
            .storage
            .pools
            .get_mut(&T::type_id_static())
            .expect("Pool not registered");
        pool.downcast_mut::<Vec<T>>()
            .expect("Type mismatch")
            .iter_mut()
    }
}
