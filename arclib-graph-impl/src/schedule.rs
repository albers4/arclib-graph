// Copyright (c) 2026 ARC (Applied Research & Computation)
// SPDX-License-Identifier: LGPL-2.1-or-later

pub struct Schedule {
    pub execution_queue: Vec<(u64, usize)>,
}

impl Schedule {
    pub fn new(execution_queue: Vec<(u64, usize)>) -> Self {
        Self { execution_queue }
    }
}
