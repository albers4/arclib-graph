// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use crate::{DType, utils::fnv1a_hash};
use arclib_graph_spec::Node;
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

    pub fn as_f32(&self) -> Option<f32> {
        match &self.payload {
            Payload::ScalarF16(v) => Some(v.to_f32()),
            Payload::ScalarF32(v) => Some(*v),
            Payload::ScalarF64(v) => Some(*v as f32),
            Payload::Symbol(_) => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match &self.payload {
            Payload::ScalarF16(v) => Some(v.to_f64()),
            Payload::ScalarF32(v) => Some(*v as f64),
            Payload::ScalarF64(v) => Some(*v),
            Payload::Symbol(_) => None,
        }
    }
}

impl Node for BaseNode {
    const TYPE_ID: u64 = fnv1a_hash("BaseNode");

    fn id(&self) -> &Uuid {
        self.id()
    }
}
