// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::{any::Any, collections::HashMap, fmt::Debug};

use uuid::Uuid;

use crate::{ContextValueLike, context::GraphContext};

pub type NodeId = Uuid;

pub type PoolExecuteFn<V> = fn(&mut Box<dyn Any + Send + Sync>, usize, &mut GraphContext<'_, V>);
pub type PoolDepCollectorFn = fn(&Box<dyn Any + Send + Sync>, usize, &mut Vec<(NodeId, NodeId)>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape(pub Vec<usize>);

impl Shape {
    pub fn flat_size(&self) -> usize {
        self.0.iter().product()
    }
}

pub trait Node<V: ContextValueLike>: 'static + Send + Sync {
    fn type_id_static() -> u64
    where
        Self: Sized;
    fn id(&self) -> &NodeId;
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("Unknown")
    }

    fn compute(&mut self, ctx: &mut GraphContext<'_, V>);
    fn dependencies(&self) -> Vec<NodeId>;

    fn infer_shape(&self, _inputs: &[Shape]) -> Result<Shape, String> {
        Err("Shape inference not implemented for this node type".to_string())
    }

    fn as_node(&self) -> &dyn Node<V>;
    fn as_node_mut(&mut self) -> &mut dyn Node<V>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &dyn Any;

    fn clone_box(&self) -> Box<dyn Node<V>>;
}

impl<V: 'static + ContextValueLike> Clone for Box<dyn Node<V>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait GraphStorageLike<V: ContextValueLike> {
    fn index_map(&self) -> &HashMap<NodeId, (u64, usize)>;
    fn pools(&self) -> &HashMap<u64, Box<dyn Any + Send + Sync>>;
    fn executors(&self) -> &HashMap<u64, PoolExecuteFn<V>>;
    fn dependency_collectors(&self) -> &HashMap<u64, PoolDepCollectorFn>;
    fn outgoing(&self) -> &HashMap<NodeId, Vec<NodeId>>;
    fn incoming(&self) -> &HashMap<NodeId, Vec<NodeId>>;
}

pub trait GraphLike<V: ContextValueLike>: Debug {
    fn get_node<T: Node<V>>(&self, id: &NodeId) -> Option<&T>;
    fn get_node_mut<T: Node<V>>(&mut self, id: &NodeId) -> Option<&mut T>;

    fn iter<T: Node<V>>(&self) -> impl Iterator<Item = &T> + '_;
    fn iter_mut<T: Node<V>>(&mut self) -> impl Iterator<Item = &mut T> + '_;

    fn register_pool<T: Node<V>>(&mut self);
    fn add_node<T: Node<V>>(&mut self, node: T) -> NodeId;
    fn connect(&mut self, source: NodeId, target: NodeId) -> Result<(), String>;

    fn compile(&mut self) -> Result<(), String>;
    fn validate_inputs(&self) -> Result<(), String>;
    fn step(&mut self) -> Result<(), String>;
}
