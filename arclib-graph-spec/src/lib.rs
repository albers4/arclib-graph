// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

/// Specification: v0.1.0
mod compiler;
mod context;
mod graph;
mod knowledge;
mod runtime;

pub use context::{ContextValueLike, GraphContext};
pub use graph::{Shape, GraphLike, GraphStorageLike, Node, NodeId, PoolDepCollectorFn, PoolExecuteFn};
