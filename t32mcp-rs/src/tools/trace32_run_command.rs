// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! The trace32_run_command tool: runs a single PRACTICE command. Not
//! available under the scripts and strict execution policies.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use super::{Context, ToolEntry, error_result, text_result, worker_call};
use crate::{mcp, t32};

/// The optional `device_prompt` of a call, e.g. "B::" (debugger and
/// simulator) or "::" (TRACE32 OS level). Only an identifier and "::": the
/// value is sent as a command line.
fn parse_device_prompt(args: &serde_json::Value) -> Result<String, &'static str> {
    match args.get("device_prompt") {
        None | Some(serde_json::Value::Null) => Ok("B::".to_string()),
        Some(v) => match v.as_str() {
            Some(p)
                if p.len() <= 10
                    && p.ends_with("::")
                    && p[..p.len() - 2].chars().all(|c| c.is_ascii_alphanumeric()) =>
            {
                Ok(p.to_string())
            }
            _ => Err("device_prompt must be letters or digits followed by \"::\", e.g. \"B::\""),
        },
    }
}

/// The optional `port` (instance) and `core` (TRACE32 core number) of a call.
fn parse_target(args: &serde_json::Value) -> Result<(Option<u16>, Option<u32>), &'static str> {
    let port = match args.get("port") {
        None | Some(serde_json::Value::Null) => None,
        Some(v) => match v.as_u64().and_then(|p| u16::try_from(p).ok()) {
            Some(p) if p > 0 => Some(p),
            _ => return Err("port must be an integer between 1 and 65535"),
        },
    };
    let core = match args.get("core") {
        None | Some(serde_json::Value::Null) => None,
        Some(v) => match v.as_u64().and_then(|c| u32::try_from(c).ok()) {
            Some(c) => Some(c),
            None => return Err("core must be a non-negative integer"),
        },
    };
    Ok((port, core))
}

pub fn trace32_run_command(
    args: Option<&serde_json::Value>,
    ctx: &mut Context,
) -> mcp::CallToolResult {
    let args = match args {
        Some(v) => v,
        None => return error_result("missing arguments"),
    };

    let command = match args.get("command").and_then(|v| v.as_str()) {
        Some(command) => command,
        None => return error_result("missing command"),
    };

    let collect_output = args
        .get("collect_output")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let timeout_secs = args.get("timeout").and_then(|v| v.as_u64()).unwrap_or(5);
    let timeout = Duration::from_secs(timeout_secs);

    let if_busy = match args.get("if_busy") {
        None | Some(serde_json::Value::Null) => t32::IfBusy::Fail,
        Some(v) => match v.as_str().and_then(t32::IfBusy::parse) {
            Some(if_busy) => if_busy,
            None => return error_result("if_busy must be \"fail\", \"terminate\" or \"nest\""),
        },
    };

    let (port, core) = match parse_target(args) {
        Ok(target) => target,
        Err(msg) => return error_result(msg),
    };
    let device_prompt = match parse_device_prompt(args) {
        Ok(prompt) => prompt,
        Err(msg) => return error_result(msg),
    };

    if !collect_output {
        // Fire-and-forget: a single command line submitted directly via the
        // Remote API, without waiting for its execution.
        if command.contains('\n') {
            return error_result(
                "collect_output=false only supports single-line commands: retry with collect_output=true for multi-line command sequences",
            );
        }

        return match worker_call(ctx, |tx| t32::WorkerRequest::RunCommand {
            command: command.to_string(),
            port,
            core,
            device_prompt,
            response_tx: tx,
        }) {
            Ok(Ok(())) => text_result("Command submitted."),
            Ok(Err(msg)) | Err(msg) => error_result(&msg),
        };
    }

    // PRACTICE commands passed via the Remote API are single-line only, so
    // the command is written to a temporary script and executed with DO.
    // PRinTer.Area redirects WinPrint output into the selected AREA so that
    // it is captured by the pipe.
    static SCRIPT_COUNT: AtomicU64 = AtomicU64::new(0);
    let path = std::env::temp_dir().join(format!(
        "t32mcp_{}_{}.cmm",
        std::process::id(),
        SCRIPT_COUNT.fetch_add(1, Ordering::Relaxed)
    ));

    let mut script = String::from("PRinTer.Area\n");
    script.push_str(command);
    script.push('\n');

    if let Err(e) = std::fs::write(&path, &script) {
        return error_result(&format!("Temp script creation failed: {e}"));
    }

    let script_path = path.to_string_lossy().to_string();

    let result = worker_call(ctx, |tx| t32::WorkerRequest::ExecuteScript {
        script_path,
        args: String::new(),
        port,
        core,
        timeout,
        if_busy,
        device_prompt,
        response_tx: tx,
    });
    let _ = std::fs::remove_file(&path);

    match result {
        Ok(Ok(text)) if text.trim().is_empty() => text_result(
            "Done. The command printed nothing. Only PRINT and WinPrint.<command> output is returned: a window command like Register.view only opens a window in PowerView.",
        ),
        Ok(Ok(text)) => text_result(&text),
        Ok(Err(msg)) | Err(msg) => error_result(&msg),
    }
}

/// The complete tool: schema, description, and handler.
pub fn entry() -> ToolEntry {
    ToolEntry {
        tool: mcp::Tool {
            annotations: None,
            description: Some(include_str!("../docs/run_command.md").to_string()),
            icons: Vec::new(),
            input_schema: mcp::ToolInputSchema {
                type_: "object".to_string(),
                extra: serde_json::json!({
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "PRACTICE command or multi-line command sequence to execute"
                        },
                        "collect_output": {
                            "type": "boolean",
                            "description": "Wait for the command to finish and return its AREA output (default: true). If false, the command is submitted without waiting (fire-and-forget) and must be a single command line.",
                            "default": true
                        },
                        "port": {
                            "type": "integer",
                            "description": "Remote API port of the PowerView instance, see trace32_get_status. If omitted, the first reachable instance is used."
                        },
                        "core": {
                            "type": "integer",
                            "description": "TRACE32 core number within the instance ('core' in trace32_get_status). Selected (CORE.Select) before the command runs. If omitted, the selected core is used."
                        },
                        "timeout": {
                            "type": "integer",
                            "description": "Maximum time in seconds to wait for the command to finish (default: 5, only used with collect_output)",
                            "default": 5
                        },
                        "if_busy": {
                            "type": "string",
                            "enum": ["fail", "terminate", "nest"],
                            "default": "fail",
                            "description": "What to do if a PRACTICE script is still active, e.g. running after a timeout or paused in a dialog. fail: report its state and call stack. terminate: end it first. nest: run on top of a paused script, e.g. to operate its dialog or to resume it with CONTinue. Only used with collect_output."
                        },
                        "device_prompt": {
                            "type": "string",
                            "default": "B::",
                            "description": "Device prompt selected before the command runs (default: B::, the debugger and simulator). It stays selected in PowerView after the call. Do not set it unless you know what you are doing and need to run a command for a non-standard device (extremely rare)."
                        }
                    },
                    "required": ["command"]
                })
                .as_object()
                .unwrap()
                .clone(),
                schema: None,
            },
            meta: None,
            name: "trace32_run_command".to_string(),
            output_schema: None,
            title: None,
        },
        handler: trace32_run_command,
    }
}
