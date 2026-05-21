// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use half::f16;
use std::collections::HashMap;

use crate::NodeId;

#[derive(Clone)]
pub enum ContextValue {
    ScalarF16(f16),
    ScalarF32(f32),
    ScalarF64(f64),
    Symbol(Box<String>),
    Empty,
}

impl ContextValue {
    pub fn as_f32(&self) -> Option<f32> {
        match &self {
            ContextValue::ScalarF16(v) => Some(v.to_f32()),
            ContextValue::ScalarF32(v) => Some(*v),
            ContextValue::ScalarF64(v) => Some(*v as f32),
            ContextValue::Symbol(_) => None,
            ContextValue::Empty => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match &self {
            ContextValue::ScalarF16(v) => Some(v.to_f64()),
            ContextValue::ScalarF32(v) => Some(*v as f64),
            ContextValue::ScalarF64(v) => Some(*v),
            ContextValue::Symbol(_) => None,
            ContextValue::Empty => None,
        }
    }
}

impl std::fmt::Display for ContextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextValue::ScalarF16(v) => write!(f, "{}", v),
            ContextValue::ScalarF32(v) => write!(f, "{}", v),
            ContextValue::ScalarF64(v) => write!(f, "{}", v),
            ContextValue::Symbol(s) => write!(f, "{}", s),
            ContextValue::Empty => write!(f, "Empty"),
        }
    }
}

pub struct GraphContext<'a> {
    pub values: &'a mut HashMap<NodeId, ContextValue>,
}

impl<'a> GraphContext<'a> {
    pub fn new(values: &'a mut HashMap<NodeId, ContextValue>) -> Self {
        Self { values }
    }

    pub fn get(&self, id: &NodeId) -> Option<&ContextValue> {
        self.values.get(id)
    }
}
