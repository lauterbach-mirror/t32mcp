// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

// The tool schemas are large JSON literals; their macro expansion needs room.
#![recursion_limit = "256"]

pub mod docs;
pub mod execution_policy;
#[allow(clippy::large_enum_variant)]
#[rustfmt::skip]
pub mod mcp;
pub mod t32;
pub mod t32api;
pub mod tools;
