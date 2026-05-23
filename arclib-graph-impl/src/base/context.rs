// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use arclib_graph_spec::ContextValueLike;
use half::f16;

#[derive(Clone)]
pub enum BaseContextValue {
    ScalarF16(f16),
    ScalarF32(f32),
    ScalarF64(f64),
    Symbol(Box<String>),
    Empty,
}

impl BaseContextValue {
    pub fn as_f32(&self) -> Option<f32> {
        match &self {
            BaseContextValue::ScalarF16(v) => Some(v.to_f32()),
            BaseContextValue::ScalarF32(v) => Some(*v),
            BaseContextValue::ScalarF64(v) => Some(*v as f32),
            BaseContextValue::Symbol(_) => None,
            BaseContextValue::Empty => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match &self {
            BaseContextValue::ScalarF16(v) => Some(v.to_f64()),
            BaseContextValue::ScalarF32(v) => Some(*v as f64),
            BaseContextValue::ScalarF64(v) => Some(*v),
            BaseContextValue::Symbol(_) => None,
            BaseContextValue::Empty => None,
        }
    }
}

impl std::fmt::Display for BaseContextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseContextValue::ScalarF16(v) => write!(f, "{}", v),
            BaseContextValue::ScalarF32(v) => write!(f, "{}", v),
            BaseContextValue::ScalarF64(v) => write!(f, "{}", v),
            BaseContextValue::Symbol(s) => write!(f, "{}", s),
            BaseContextValue::Empty => write!(f, "Empty"),
        }
    }
}

impl ContextValueLike for BaseContextValue {}
