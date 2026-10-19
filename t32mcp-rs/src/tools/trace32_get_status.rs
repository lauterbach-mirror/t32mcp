// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! The trace32_get_status tool: the PowerView instances, their cores, and
//! the target state. Available in every execution policy.

use std::time::Duration;

use super::{Context, ToolEntry, call_result, error_result, worker_call};
use crate::{mcp, t32};

pub fn trace32_get_status(
    args: Option<&serde_json::Value>,
    ctx: &mut Context,
) -> mcp::CallToolResult {
    let timeout_secs = args
        .and_then(|a| a.get("timeout"))
        .and_then(|v| v.as_u64())
        .unwrap_or(5);

    match worker_call(ctx, |tx| t32::WorkerRequest::QueryTargetSystem {
        timeout: Duration::from_secs(timeout_secs),
        response_tx: tx,
    }) {
        Ok(Ok(json)) => call_result(
            serde_json::to_string_pretty(&json).unwrap_or_default(),
            None,
            Some(json),
        ),
        Ok(Err(msg)) | Err(msg) => error_result(&msg),
    }
}

/// The complete tool: schema, description, and handler.
pub fn entry() -> ToolEntry {
    ToolEntry {
        tool: mcp::Tool {
            annotations: None,
            description: Some(include_str!("../docs/get_status.md").to_string()),
            icons: Vec::new(),
            input_schema: mcp::ToolInputSchema {
                type_: "object".to_string(),
                extra: serde_json::json!({
                    "properties": {
                        "timeout": {
                            "type": "integer",
                            "description": "Maximum time in seconds to wait for the query to finish (default: 5)",
                            "default": 5
                        }
                    },
                    "required": []
                })
                .as_object()
                .unwrap()
                .clone(),
                schema: None,
            },
            meta: None,
            name: "trace32_get_status".to_string(),
            output_schema: Some(mcp::ToolOutputSchema {
                schema: Some("https://json-schema.org/draft/2020-12/schema".to_string()),
                extra: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "instances": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "port": {
                                        "type": "integer",
                                        "description": "Remote API port of the PowerView instance"
                                    },
                                    "state": {
                                        "type": "string",
                                        "enum": ["connected", "not reachable"]
                                    },
                                    "error": {
                                        "type": "string",
                                        "description": "Why the instance is not reachable, or what could not be queried"
                                    },
                                    "practice": {
                                        "type": "object",
                                        "description": "PRACTICE interpreter: state, stack, windows"
                                    },
                                    "cores": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "core": {
                                                    "type": "integer",
                                                    "description": "TRACE32 core number within the instance (CORE())"
                                                },
                                                "selected": {
                                                    "type": "boolean",
                                                    "description": "Whether this is the currently selected core"
                                                },
                                                "cpu_family": {
                                                    "type": "string",
                                                    "description": "CPU family"
                                                },
                                                "cpu": {
                                                    "type": "string",
                                                    "description": "CPU name"
                                                },
                                                "state": {
                                                    "type": "string",
                                                    "description": "Target state line (STATE.TARGET)"
                                                },
                                                "running": {
                                                    "type": "boolean",
                                                    "description": "Whether the core is running"
                                                },
                                                "registers": {
                                                    "type": "array",
                                                    "description": "CPU registers (only present for halted cores)",
                                                    "items": {
                                                        "type": "object",
                                                        "properties": {
                                                            "name": {
                                                                "type": "string",
                                                                "description": "Register name"
                                                            },
                                                            "value": {
                                                                "type": "string",
                                                                "description": "Register value (hex string, or array of hex strings for vector registers)",
                                                                "oneOf": [
                                                                    { "type": "string" },
                                                                    { "type": "array", "items": { "type": "string" } }
                                                                ]
                                                            }
                                                        },
                                                        "required": ["name"]
                                                    }
                                                },
                                                "error": {
                                                    "type": "string",
                                                    "description": "Error message if this core could not be queried"
                                                }
                                            },
                                            "required": ["core"]
                                        }
                                    }
                                },
                                "required": ["port", "state"]
                            }
                        }
                    },
                    "required": ["instances"]
                })
                .as_object()
                .unwrap()
                .clone(),
            }),
            title: None,
        },
        handler: trace32_get_status,
    }
}
