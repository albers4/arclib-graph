// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::{
    any::Any,
    collections::{HashMap, VecDeque},
};

use arclib_graph_spec::{ContextValueLike, GraphContext, GraphStorageLike, Node, NodeId};

pub const fn fnv1a_hash(s: &str) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x1000000000000003;

    let mut hash = FNV_OFFSET;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }
    hash
}

pub fn execute_wrapper<V: ContextValueLike, T: Node<V>>(
    pool: &mut Box<dyn Any + Send + Sync>,
    index: usize,
    ctx: &mut GraphContext<'_, V>,
) {
    let vec = pool
        .downcast_mut::<Vec<T>>()
        .expect("Executor type mismatch");
    vec[index].compute(ctx);
}

pub fn collect_deps_wrapper<V: ContextValueLike, T: Node<V>>(
    pool: &Box<dyn Any + Send + Sync>,
    index: usize,
    out: &mut Vec<(NodeId, NodeId)>,
) {
    if let Some(vec) = pool.downcast_ref::<Vec<T>>()
        && let Some(node) = vec.get(index)
    {
        let self_id = *node.id();
        for dep_id in node.dependencies() {
            out.push((dep_id, self_id));
        }
    }
}

pub fn as_node_wrapper<V: ContextValueLike, T: Node<V>>(
    pool: &Box<dyn Any + Send + Sync>,
    index: usize,
) -> &dyn Node<V> {
    let vec = pool
        .downcast_ref::<Vec<T>>()
        .expect("Pool downcast failed in as_node_wrapper");
    &vec[index]
}

pub fn as_node_mut_wrapper<V: ContextValueLike, T: Node<V>>(
    pool: &mut Box<dyn Any + Send + Sync>,
    index: usize,
) -> &mut dyn Node<V> {
    let vec = pool
        .downcast_mut::<Vec<T>>()
        .expect("Pool downcast failed in as_node_mut_wrapper");
    &mut vec[index]
}

pub fn topological_sort<V: ContextValueLike>(
    storage: &impl GraphStorageLike<V>,
) -> Result<Vec<NodeId>, String> {
    let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
    let mut queue: VecDeque<NodeId> = VecDeque::new();
    let mut sorted: Vec<NodeId> = Vec::new();

    for id in storage.index_map().keys() {
        let deg = storage.incoming().get(id).map(|v| v.len()).unwrap_or(0);
        in_degree.insert(*id, deg);
        if deg == 0 {
            queue.push_back(*id);
        }
    }

    while let Some(node_id) = queue.pop_front() {
        sorted.push(node_id);

        if let Some(targets) = storage.outgoing().get(&node_id) {
            for target_id in targets {
                if let Some(deg) = in_degree.get_mut(target_id) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(*target_id);
                    }
                }
            }
        }
    }

    if sorted.len() != storage.index_map().len() {
        return Err("Cycle detected in graph".to_string());
    }

    Ok(sorted)
}
