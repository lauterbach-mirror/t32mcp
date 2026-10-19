// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! Tool handlers of t32mcp and the infrastructure they share: the context,
//! the tool entry, and the helpers that talk to the worker thread.
//! Each tool handler is expected to be implemented in a separate file that
//! provides an `entry` function (parameterizable)

pub mod trace32_get_status;
pub mod trace32_manage_instances;
pub mod trace32_run_command;
pub mod trace32_run_script;

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::mpsc;

use crate::execution_policy;
use crate::mcp;
use crate::t32;

pub type ToolHandler = fn(Option<&serde_json::Value>, &mut Context) -> mcp::CallToolResult;

pub struct ToolEntry {
    pub tool: mcp::Tool,
    pub handler: ToolHandler,
}

pub struct Context {
    pub event_queue: VecDeque<serde_json::Value>,
    pub instructions: String,
    pub policy: execution_policy::ExecutionPolicy,
    /// --skills: if not empty, skill scripts must lie below one of them.
    pub skill_roots: Vec<PathBuf>,
    pub tools: Vec<ToolEntry>,
    pub worker_tx: mpsc::Sender<t32::WorkerRequest>,
}

pub fn error_result(text: &str) -> mcp::CallToolResult {
    call_result(text.to_string(), Some(true), None)
}

pub fn text_result(text: &str) -> mcp::CallToolResult {
    call_result(text.to_string(), None, None)
}

fn call_result(
    text: String,
    is_error: Option<bool>,
    structured_content: Option<serde_json::Value>,
) -> mcp::CallToolResult {
    mcp::CallToolResult {
        content: vec![mcp::ContentBlock::TextContent(mcp::TextContent {
            annotations: None,
            meta: None,
            text,
            type_: "text".to_string(),
        })],
        is_error,
        meta: None,
        result_type: "complete".to_string(),
        structured_content,
    }
}

/// Sends a request to the worker thread and waits for its response.
fn worker_call<R>(
    ctx: &Context,
    make_request: impl FnOnce(mpsc::Sender<R>) -> t32::WorkerRequest,
) -> Result<R, String> {
    let (tx, rx) = mpsc::channel();
    let request = make_request(tx);
    ctx.worker_tx
        .send(request)
        .map_err(|_| "Worker thread is not available".to_string())?;
    rx.recv()
        .map_err(|_| "Worker thread dropped the response channel".to_string())
}
