// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! The trace32_run_script tool: runs a PRACTICE script, usually a skill
//! script. Available in every execution policy.

use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use super::{Context, ToolEntry, error_result, text_result, worker_call};
use crate::{docs, execution_policy, mcp, t32};

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

/// A skill or script name: only letters, digits, `-` and `_`. It is joined
/// into a path below the --skills directories, so `..` and path separators
/// must be impossible.
fn is_safe_name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Resolves `skill_name` and `script_name` inside the --skills directories:
/// `<root>/<skill_name>/SKILL.md` next to `<root>/<skill_name>/scripts/<script_name>.cmm`.
/// The first root that provides the script wins, in the order of --skills.
fn resolve_skill_script(
    skill_name: &str,
    script_name: &str,
    skill_roots: &[PathBuf],
) -> Result<PathBuf, String> {
    if !is_safe_name(skill_name) {
        return Err(format!(
            "Invalid skill_name '{skill_name}': only letters, digits, '-' and '_'"
        ));
    }
    let base = script_name.strip_suffix(".cmm").unwrap_or(script_name);
    if !is_safe_name(base) {
        return Err(format!(
            "Invalid script_name '{script_name}': only letters, digits, '-' and '_', with or without the .cmm suffix"
        ));
    }
    if skill_roots.is_empty() {
        return Err(
            "No --skills directories configured (required by the strict execution policy)"
                .to_string(),
        );
    }
    let mut skill_found = false;
    for root in skill_roots {
        let skill_dir = root.join(skill_name);
        if !skill_dir.join("SKILL.md").is_file() {
            continue;
        }
        skill_found = true;
        let path = skill_dir.join("scripts").join(format!("{base}.cmm"));
        if path.is_file() {
            return Ok(path);
        }
    }
    if skill_found {
        Err(format!(
            "Script '{base}.cmm' not found in skill '{skill_name}': no scripts/{base}.cmm next to its SKILL.md"
        ))
    } else {
        Err(format!(
            "Skill '{skill_name}' not found in the --skills directories: no SKILL.md next to a scripts directory"
        ))
    }
}

/// A skill script lives at `<skill>/scripts/<name>.cmm` with `<skill>/SKILL.md` next to it.
/// With skill roots, it must also lie below one of them.
///
/// The check is lexical: symlinks are not resolved, so skills linked into a root
/// (e.g. by `npx skills`) count. `..` is rejected, it could leave the root.
fn is_skill_script(path: &Path, skill_roots: &[PathBuf]) -> bool {
    if path.components().any(|c| c == Component::ParentDir) {
        return false;
    }
    let Some(scripts_dir) = path.parent() else {
        return false;
    };
    if scripts_dir.file_name().and_then(|n| n.to_str()) != Some("scripts") {
        return false;
    }
    let Some(skill_dir) = scripts_dir.parent() else {
        return false;
    };
    if !skill_dir.join("SKILL.md").is_file() {
        return false;
    }
    skill_roots.is_empty() || skill_roots.iter().any(|root| path_starts_with(path, root))
}

/// `Path::starts_with`, but case-insensitive on Windows.
fn path_starts_with(path: &Path, root: &Path) -> bool {
    if !cfg!(windows) {
        return path.starts_with(root);
    }
    let mut path = path.components();
    root.components().all(|r| {
        path.next().is_some_and(|p| {
            p.as_os_str()
                .to_string_lossy()
                .eq_ignore_ascii_case(&r.as_os_str().to_string_lossy())
        })
    })
}

pub fn trace32_run_script(
    args: Option<&serde_json::Value>,
    ctx: &mut Context,
) -> mcp::CallToolResult {
    let Some(args) = args else {
        return error_result("missing arguments");
    };

    let script_args = args.get("script_args");
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

    let mut warnings: Vec<String> = Vec::new();

    // Under strict, a skill script addressed by skill_name and script_name
    // (the names translate between the agent's and the PowerView host's
    // file systems); otherwise an absolute path, or ~~ (resolved by
    // PowerView, no local file to check).
    let (path, script_path): (Option<PathBuf>, String);
    if ctx.policy == execution_policy::ExecutionPolicy::Strict {
        let name = |key: &str| {
            args.get(key)
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| format!("missing {key}"))
        };
        let resolved = match (name("skill_name"), name("script_name")) {
            (Ok(skill), Ok(script)) => resolve_skill_script(skill, script, &ctx.skill_roots),
            (Err(msg), _) | (_, Err(msg)) => Err(msg),
        };
        let resolved = match resolved {
            Ok(path) => path,
            Err(msg) => return error_result(&msg),
        };
        script_path = resolved.to_string_lossy().to_string();
        path = Some(resolved);
    } else {
        let requested = args
            .get("script_path")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        if !requested.to_ascii_lowercase().ends_with(".cmm") {
            return error_result("Not a valid PRACTICE (*.cmm) script");
        }
        if requested.starts_with("~~") {
            path = None;
        } else {
            let script = Path::new(requested);
            if !script.is_absolute() {
                return error_result(&format!(
                    "script_path must be absolute or start with ~~: {requested}"
                ));
            }
            if !script.is_file() {
                return error_result(&format!("File not found: {requested}"));
            }
            if !is_skill_script(script, &ctx.skill_roots) {
                warnings.push(
                    "Warning: not a skill script, see skill lauterbach-trace32-skill-creator."
                        .to_string(),
                );
            }
            path = Some(script.to_path_buf());
        }
        script_path = requested.to_string();
    };

    // The header of a local script enforces the execution policy and
    // validates the arguments under every policy.
    if let Some(path) = &path {
        let source = match std::fs::read(path) {
            Ok(source) => source,
            Err(e) => return error_result(&format!("Cannot read {}: {e}", path.display())),
        };
        let header = match execution_policy::ScriptHeader::parse(
            &String::from_utf8_lossy(&source),
            ctx.policy,
        ) {
            Ok(header) => header,
            Err(msg) => return error_result(&msg),
        };
        if let Err(msg) = header.validate_args(script_args) {
            return error_result(&msg);
        }
        warnings.extend(header.warnings);
    }

    let mut args_str = String::new();
    if let Some(serde_json::Value::Object(map)) = script_args {
        for (key, value) in map {
            let key = key.trim();
            if key.contains(' ') {
                return error_result(&format!("Invalid key '{key}': Contains whitespace"));
            }
            let value = value.as_str().unwrap_or("").replace('"', "\"\"");
            args_str.push_str(&format!(" \"{key}={value}\""));
        }
    }

    // The warnings explain a failure, e.g. an argument the script does not expect.
    let with_warnings = |text: String| {
        if warnings.is_empty() {
            text
        } else {
            format!("{}\n\n{text}", warnings.join("\n"))
        }
    };
    match worker_call(ctx, |tx| t32::WorkerRequest::ExecuteScript {
        script_path,
        args: args_str,
        port,
        core,
        timeout,
        if_busy,
        device_prompt,
        response_tx: tx,
    }) {
        Ok(Ok(output)) => text_result(&with_warnings(output)),
        Ok(Err(msg)) | Err(msg) => error_result(&with_warnings(msg)),
    }
}

/// The complete tool: schema, description, and handler. Description and input
/// schema depend on the execution policy.
pub fn entry(policy: execution_policy::ExecutionPolicy, skills_source: &str) -> ToolEntry {
    ToolEntry {
        tool: mcp::Tool {
            annotations: None,
            description: Some(execute_practice_script_description(policy, skills_source)),
            icons: Vec::new(),
            input_schema: run_script_input_schema(policy),
            meta: None,
            name: "trace32_run_script".to_string(),
            output_schema: None,
            title: None,
        },
        handler: trace32_run_script,
    }
}

/// Description of trace32_run_script: one template per execution policy,
/// see docs/run_script_*.md.
fn execute_practice_script_description(
    policy: execution_policy::ExecutionPolicy,
    source: &str,
) -> String {
    match policy {
        execution_policy::ExecutionPolicy::Unrestricted => {
            include_str!("../docs/run_script_unrestricted.md")
                .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
                .replace(
                    "{INTERFACE}",
                    &execution_policy::SCRIPT_INTERFACE_VERSION.to_string(),
                )
                .replace("{SKILLS_SOURCE}", &docs::skills_source(policy, source))
        }
        execution_policy::ExecutionPolicy::Scripts => include_str!("../docs/run_script_scripts.md")
            .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
            .replace(
                "{INTERFACE}",
                &execution_policy::SCRIPT_INTERFACE_VERSION.to_string(),
            )
            .replace("{SKILLS_SOURCE}", &docs::skills_source(policy, source)),
        execution_policy::ExecutionPolicy::Strict => include_str!("../docs/run_script_strict.md")
            .replace("{VERSION}", env!("CARGO_PKG_VERSION"))
            .replace(
                "{INTERFACE}",
                &execution_policy::SCRIPT_INTERFACE_VERSION.to_string(),
            )
            .replace("{SKILLS_SOURCE}", &docs::skills_source(policy, source)),
    }
}

/// Input schema of trace32_run_script. Under strict, scripts are addressed by
/// skill_name and script_name inside the --skills directories instead of an
/// absolute script_path.
fn run_script_input_schema(policy: execution_policy::ExecutionPolicy) -> mcp::ToolInputSchema {
    let mut properties = serde_json::Map::new();
    let mut required: Vec<&str> = Vec::new();
    match policy {
        execution_policy::ExecutionPolicy::Strict => {
            properties.insert(
                "skill_name".to_string(),
                serde_json::json!({
                    "type": "string",
                    "description": "Name of the skill that provides the script: its directory name, e.g. lauterbach-trace32-debug-basic. Must exist in the --skills directories of this server; its SKILL.md documents the scripts."
                }),
            );
            properties.insert(
                "script_name".to_string(),
                serde_json::json!({
                    "type": "string",
                    "description": "Name of the PRACTICE script in <skill>/scripts/, e.g. get_state or get_state.cmm"
                }),
            );
            required.extend(["skill_name", "script_name"]);
        }
        _ => {
            properties.insert(
                "script_path".to_string(),
                serde_json::json!({
                    "type": "string",
                    "description": "Absolute path of the PRACTICE script (*.cmm), e.g. <skill directory>/scripts/get_state.cmm. Paths starting with ~~ are resolved by PowerView."
                }),
            );
            required.push("script_path");
        }
    }
    properties.insert(
        "script_args".to_string(),
        serde_json::json!({
            "type": "object",
            "description": "Arguments to pass to the script",
            "additionalProperties": { "type": "string" }
        }),
    );
    properties.insert(
        "port".to_string(),
        serde_json::json!({
            "type": "integer",
            "description": "Remote API port of the PowerView instance, see trace32_get_status. If omitted, the first reachable instance is used."
        }),
    );
    properties.insert(
        "core".to_string(),
        serde_json::json!({
            "type": "integer",
            "description": "TRACE32 core number within the instance ('core' in trace32_get_status). Selected (CORE.Select) before the script runs. If omitted, the selected core is used."
        }),
    );
    properties.insert(
        "timeout".to_string(),
        serde_json::json!({
            "type": "integer",
            "description": "Maximum time in seconds to wait for the script to finish (default: 5)",
            "default": 5
        }),
    );
    properties.insert(
        "if_busy".to_string(),
        serde_json::json!({
            "type": "string",
            "enum": ["fail", "terminate", "nest"],
            "default": "fail",
            "description": "What to do if a PRACTICE script is still active, e.g. running after a timeout or paused in a dialog. fail: report its state and call stack. terminate: end it first. nest: run on top of a paused script, e.g. to operate its dialog."
        }),
    );
    properties.insert(
        "device_prompt".to_string(),
        serde_json::json!({
            "type": "string",
            "default": "B::",
            "description": "Device prompt selected before the script runs (default: B::, the debugger and simulator). It stays selected in PowerView after the call. Do not set it unless you know what you are doing and need to run a script for a non-standard device (extremely rare)."
        }),
    );
    mcp::ToolInputSchema {
        type_: "object".to_string(),
        extra: serde_json::json!({ "properties": properties, "required": required })
            .as_object()
            .unwrap()
            .clone(),
        schema: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_contains_skills_source() {
        // Where to get the skills, in every policy.
        for policy in [
            execution_policy::ExecutionPolicy::Unrestricted,
            execution_policy::ExecutionPolicy::Scripts,
            execution_policy::ExecutionPolicy::Strict,
        ] {
            assert!(
                execute_practice_script_description(policy, docs::DEFAULT_SKILLS_SOURCE)
                    .contains("assistant.ai.lauterbach.com/agents/skills")
            );
        }
        assert!(
            execute_practice_script_description(
                execution_policy::ExecutionPolicy::Strict,
                docs::DEFAULT_SKILLS_SOURCE
            )
            .contains("lauterbach-t32mcp-execution-policy")
        );
    }

    /// <tmp>/root/my-skill/{SKILL.md, scripts/a.cmm} and <tmp>/other/scripts/b.cmm (no SKILL.md).
    fn skill_tree(name: &str) -> PathBuf {
        let base = std::env::temp_dir().join(format!("t32mcp-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(base.join("root/my-skill/scripts")).unwrap();
        std::fs::create_dir_all(base.join("other/scripts")).unwrap();
        std::fs::write(base.join("root/my-skill/SKILL.md"), "").unwrap();
        std::fs::write(base.join("root/my-skill/scripts/a.cmm"), "").unwrap();
        std::fs::write(base.join("other/scripts/b.cmm"), "").unwrap();
        base
    }

    #[test]
    fn skill_script_structure() {
        let base = skill_tree("structure");
        assert!(is_skill_script(
            &base.join("root/my-skill/scripts/a.cmm"),
            &[]
        ));
        assert!(!is_skill_script(&base.join("other/scripts/b.cmm"), &[]));
        assert!(!is_skill_script(&base.join("root/my-skill/SKILL.md"), &[]));
        // .. could leave a skill root, even if it ends in a skill script.
        assert!(!is_skill_script(
            &base.join("other/../root/my-skill/scripts/a.cmm"),
            &[]
        ));
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn skill_script_roots() {
        let base = skill_tree("roots");
        let script = base.join("root/my-skill/scripts/a.cmm");
        assert!(is_skill_script(&script, &[base.join("root")]));
        assert!(is_skill_script(
            &script,
            &[base.join("other"), base.clone()]
        ));
        assert!(!is_skill_script(&script, &[base.join("other")]));
        // Lexical prefix only: "root2" is not below "root".
        assert!(!is_skill_script(&script, &[base.join("roo")]));
        let _ = std::fs::remove_dir_all(&base);
    }

    #[test]
    fn resolve_skill_script_by_names() {
        let base = skill_tree("resolve");
        let root = base.join("root");
        assert_eq!(
            resolve_skill_script("my-skill", "a", std::slice::from_ref(&root)).unwrap(),
            root.join("my-skill/scripts/a.cmm")
        );
        // The .cmm suffix is optional.
        assert_eq!(
            resolve_skill_script("my-skill", "a.cmm", std::slice::from_ref(&root)).unwrap(),
            root.join("my-skill/scripts/a.cmm")
        );
        // Names cannot leave the roots.
        for (skill, script) in [
            ("..", "a"),
            ("my-skill", "../a"),
            ("a/b", "a"),
            ("my-skill", "a/b.cmm"),
        ] {
            assert!(
                resolve_skill_script(skill, script, std::slice::from_ref(&root)).is_err(),
                "{skill}/{script}"
            );
        }
        // The first root that provides the script wins.
        let other = base.join("other");
        std::fs::create_dir_all(base.join("other/my-skill/scripts")).unwrap();
        std::fs::write(base.join("other/my-skill/SKILL.md"), "").unwrap();
        std::fs::write(base.join("other/my-skill/scripts/a.cmm"), "").unwrap();
        assert_eq!(
            resolve_skill_script("my-skill", "a", &[other.clone(), root.clone()]).unwrap(),
            other.join("my-skill/scripts/a.cmm")
        );
        assert_eq!(
            resolve_skill_script("my-skill", "b.cmm", &[other.clone(), root.clone()]).unwrap_err(),
            "Script 'b.cmm' not found in skill 'my-skill': no scripts/b.cmm next to its SKILL.md"
        );
        // Not a skill: SKILL.md missing.
        assert!(
            resolve_skill_script("no-such-skill", "a", &[root])
                .unwrap_err()
                .contains("not found")
        );
        assert!(
            resolve_skill_script("my-skill", "a", &[])
                .unwrap_err()
                .contains("--skills")
        );
        let _ = std::fs::remove_dir_all(&base);
    }
}
