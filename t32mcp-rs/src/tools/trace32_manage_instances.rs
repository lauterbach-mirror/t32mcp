// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! The trace32_manage_instances tool: add and remove PowerView instances at
//! run time. Not available under the strict execution policy.

use super::{Context, ToolEntry, call_result, error_result, worker_call};
use crate::{mcp, t32};

pub fn trace32_manage_instances(
    args: Option<&serde_json::Value>,
    ctx: &mut Context,
) -> mcp::CallToolResult {
    let action = args.and_then(|a| a.get("action")).and_then(|v| v.as_str());
    let port = args
        .and_then(|a| a.get("port"))
        .and_then(|v| v.as_u64())
        .map(u16::try_from);
    let action = match (action, port) {
        (Some("inspect"), _) => t32::InstanceAction::Inspect,
        (Some("add" | "remove"), None) => {
            return error_result("action \"add\" and \"remove\" require a port");
        }
        (Some("add" | "remove"), Some(Err(_))) => {
            return error_result("port must be between 1 and 65535");
        }
        (Some("add"), Some(Ok(port))) => t32::InstanceAction::Add(port),
        (Some("remove"), Some(Ok(port))) => t32::InstanceAction::Remove(port),
        _ => return error_result("action must be \"inspect\", \"add\" or \"remove\""),
    };

    match worker_call(ctx, |tx| t32::WorkerRequest::ManageInstances {
        action,
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
            description: Some(include_str!("../docs/manage_instances.md").to_string()),
            icons: Vec::new(),
            input_schema: mcp::ToolInputSchema {
                type_: "object".to_string(),
                extra: serde_json::json!({
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["inspect", "add", "remove"],
                            "description": "inspect: list the instances. add, remove: add or remove the instance with 'port'."
                        },
                        "port": {
                            "type": "integer",
                            "description": "Remote API TCP port of the instance. Required for add and remove, ignored for inspect."
                        }
                    },
                    "required": ["action"]
                })
                .as_object()
                .unwrap()
                .clone(),
                schema: None,
            },
            meta: None,
            name: "trace32_manage_instances".to_string(),
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
                                    "port": { "type": "integer", "description": "Remote API TCP port" },
                                    "state": { "type": "string", "enum": ["connected", "not reachable"] },
                                    "error": { "type": "string", "description": "Why the instance is not reachable" }
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
        handler: trace32_manage_instances,
    }
}
