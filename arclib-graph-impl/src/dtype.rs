// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum DType {
    F16,
    F32,
    F64,
    String,
}

impl Display for DType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DType::F16 => write!(f, "f16"),
            DType::F32 => write!(f, "f32"),
            DType::F64 => write!(f, "f64"),
            DType::String => write!(f, "String"),
        }
    }
}

impl From<DType> for String {
    fn from(value: DType) -> Self {
        format!("{}", value)
    }
}
