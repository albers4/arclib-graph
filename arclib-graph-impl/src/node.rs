// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use crate::{DType, base::BaseContextValue, utils::fnv1a_hash};
use arclib_graph_spec::{GraphContext, Node, NodeId};
use half::f16;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub enum BaseNodeKind {
    Scalar,
    Symbol,
}

#[derive(Clone)]
pub enum Payload {
    ScalarF16(f16),
    ScalarF32(f32),
    ScalarF64(f64),
    Symbol(Box<String>),
}

impl Payload {
    pub fn as_f32(&self) -> Option<f32> {
        match &self {
            Payload::ScalarF16(v) => Some(v.to_f32()),
            Payload::ScalarF32(v) => Some(*v),
            Payload::ScalarF64(v) => Some(*v as f32),
            Payload::Symbol(_) => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match &self {
            Payload::ScalarF16(v) => Some(v.to_f64()),
            Payload::ScalarF32(v) => Some(*v as f64),
            Payload::ScalarF64(v) => Some(*v),
            Payload::Symbol(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct BaseNode {
    pub id: Uuid,
    pub kind: BaseNodeKind,
    pub dtype: DType,
    pub payload: Payload,
}

impl BaseNode {
    pub fn new(kind: BaseNodeKind, dtype: DType, payload: Payload) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            kind,
            dtype,
            payload,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn dtype(&self) -> DType {
        self.dtype
    }

    pub fn value(&self) -> &Payload {
        &self.payload
    }
}

impl Node<BaseContextValue> for BaseNode {
    fn type_id_static() -> u64
    where
        Self: Sized,
    {
        fnv1a_hash("BaseNode")
    }

    fn id(&self) -> &Uuid {
        self.id()
    }

    fn as_node(&self) -> &dyn Node<BaseContextValue> {
        self
    }

    fn as_node_mut(&mut self) -> &mut dyn Node<BaseContextValue> {
        self
    }

    fn compute(&mut self, ctx: &mut GraphContext<'_, BaseContextValue>) {
        match &self.payload {
            Payload::ScalarF16(v) => ctx
                .values
                .insert(*self.id(), BaseContextValue::ScalarF16(*v)),
            Payload::ScalarF32(v) => ctx
                .values
                .insert(*self.id(), BaseContextValue::ScalarF32(*v)),
            Payload::ScalarF64(v) => ctx
                .values
                .insert(*self.id(), BaseContextValue::ScalarF64(*v)),
            Payload::Symbol(v) => ctx
                .values
                .insert(*self.id(), BaseContextValue::Symbol(v.clone())),
        };
    }

    fn dependencies(&self) -> Vec<NodeId> {
        Vec::new()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &dyn std::any::Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Node<BaseContextValue>> {
        Box::new(self.clone())
    }
}
