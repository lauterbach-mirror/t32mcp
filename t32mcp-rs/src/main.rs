#![allow(dead_code)]

// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

use t32mcp::*;

use anyhow::Result;
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
};

use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::{BuildHasherDefault, DefaultHasher};
use std::io::Write;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, Ordering};
use tempfile::Builder;
use tokio::{
    io::{AsyncReadExt, BufReader},
    sync::Mutex,
    time::{Duration, sleep},
};

// Provides access to the AREA output of TRACE32
#[derive(Debug)]
struct T32Sink {
    pub active: bool,
    pub received_data: String,
}

impl T32Sink {
    pub fn new() -> Self {
        Self {
            active: false,
            received_data: String::new(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.received_data.clear();
    }
}

static SINK: LazyLock<Mutex<T32Sink>> = LazyLock::new(|| Mutex::new(T32Sink::new()));

static CONNECTED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
pub struct T32Options {
    pub unlock_all_tools: bool,
    pub skills_base: String,
    pub port: u32,
    pub pipe_name: String,
}

#[derive(Debug, Clone)]
pub struct T32 {
    tool_router: ToolRouter<Self>,
    options: T32Options,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SkillRequest {
    #[schemars(
        description = "Name of the skill containing the script: .../skill_name/scripts/script_name"
    )]
    pub skill_name: String,

    #[schemars(description = "Name of the PRACTICE script file (*.cmm) to execute.")]
    pub script_name: String,

    #[schemars(description = "Arguments to pass to the script")]
    pub script_args: Option<HashMap<String, String, BuildHasherDefault<DefaultHasher>>>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PracticeRequest {
    #[schemars(description = "Content of the PRACTICE script.")]
    pub practice: String,

    #[schemars(description = "Value indicating whether to execute the PRACTICE script.")]
    pub execute: bool,
}

#[tool_router]
impl T32 {
    // https://github.com/Cookee24/GithubFetcher/blob/4bdd7c6290b278e9e0645e4f9cf9b390c8075cd9/src/server/mod.rs#L40
    pub fn new(options: T32Options) -> Self {
        let mut server = Self {
            tool_router: Self::tool_router(),
            options,
        };

        if !server.options.unlock_all_tools {
            server.tool_router.remove_route("execute_practice");
        }

        server
    }

    fn empty_stack(&self) -> Result<bool, String> {
        let depth = t32_fnc("PRACTICE.StackDepth()".to_string())?;
        let is_empty = depth == "0.";
        Ok(is_empty)
    }

    fn establish_debug_connection(&self, ignore_stack: bool) -> Result<(), String> {
        if !CONNECTED.load(Ordering::Relaxed) {
            t32_connect(self.options.port)?;
        }

        CONNECTED.store(true, Ordering::Relaxed);

        if let Ok(is_empty) = self.empty_stack() {
            if !is_empty && !ignore_stack {
                return Err("Discovered that the PRACTICE stack depth is greater than 0, indicating that another script is active. To continue, the running script must first finish execution. We do not allow multiple scripts to run simultaneously.".to_string());
            } else {
                return Ok(());
            }
        }

        t32_connect(self.options.port)?;

        if !self.empty_stack()? {
            return Err("Discovered that the PRACTICE stack depth is greater than 0, indicating that another script is active. To continue, the running script must first finish execution. We do not allow multiple scripts to run simultaneously.".to_string());
        }

        Ok(())
    }

    #[tool(
        name = "execute_practice_skill",
        description = "Execute a PRACTICE script from a specified skill using TRACE32 PowerView.
This tool starts the execution of a TRACE32 PRACTICE script (.cmm file)
located within a skill's scripts directory.
The script is run on a connected TRACE32 PowerView instance via the API Port.

We do not allow the execution of multiple scripts simultaneously. To ensure that,
we check the PRACTICE stack depth before the launch of the script.

It is possible that the executed script contains an error.
In that case, this function does NOT return an error since it only initiates
the execution and does not wait for completion. Script errors can only be observed
via the PowerView GUI by the user, or inferred if the script is still running
during the next tool call.

This tool call does not output the generated text by the script.
For that, use the tool collect_practice_skill_response.

Returns: Result of the operation.

Possible exceptions:
    - The specified (skill_name, script_name) is not found
    - TRACE32 PowerView is not running or the API Port is not enabled
    - Another script is already running

Prerequisites:
    - TRACE32 PowerView must be running
    - API Port must be enabled in PowerView configuration
    - The specified skill and script must exist in the expected directory structure"
    )]
    async fn execute_practice_skill(
        &self,
        Parameters(SkillRequest {
            skill_name,
            script_name,
            script_args,
        }): Parameters<SkillRequest>,
    ) -> Result<String, String> {
        if !script_name.ends_with(".cmm") {
            return Err("Not a valid PRACTICE (*.cmm) script".to_string());
        }

        let mut path = PathBuf::from(self.options.skills_base.as_str());
        path.push(format!("skill-{skill_name}"));
        path.push("scripts");
        path.push(script_name);
        if !path.is_file() {
            return Err(format!("File not found: {path:?}"));
        }

        let mut sink = match SINK.try_lock() {
            Ok(s) => {
                if s.active {
                    return Err("Cannot setup named pipe (currently active)".to_string());
                }
                s
            }
            Err(e) => {
                return Err(format!("Cannot setup named pipe ({e})"));
            }
        };

        self.establish_debug_connection(false)?;

        let connection = t32_open_pipe(&self.options.pipe_name).await?;

        sink.activate();

        let path = path.to_string_lossy().to_string();
        let mut args = "".to_string();
        if let Some(a) = script_args {
            for (key, value) in &a {
                let key = key.trim();
                if key.contains(" ") {
                    return Err(format!("Invalid key '{key}': Contains whitespace"));
                }
                let value = value.replace("\"", "\"\"");
                args += &format!(" \"{key}={value}\"");
            }
        }

        t32_cmd(format!("DO \"{path}\"{args}"))?;

        tokio::spawn(async move {
            let mut receiver = BufReader::with_capacity(256 * 1024, connection);
            let mut buf = vec![0u8; 64 * 1024];

            loop {
                let res = receiver.read(&mut buf).await;
                let mut sink = SINK.lock().await;
                match res {
                    Ok(0) => {
                        // Server disconnected
                        sink.active = false;
                        break;
                    }
                    Ok(n) => {
                        let message = String::from_utf8_lossy(&buf[..n]);
                        sink.received_data.push_str(&message);
                    }
                    Err(e) => {
                        sink.active = false;
                        sink.received_data.push_str(&format!("\nRead error: {e}"));
                        break;
                    }
                }
            }
        });

        drop(sink);

        self.collect_practice_skill_response().await
    }

    #[tool(
        name = "collect_practice_skill_response",
        description = "Collects the response generated so far by a previous call to execute_practice_skill.

        It is possible that the script has not finished execution yet.
        To communicate this information, the returned string starts with <[NOT] FINISHED>.

Returns: Generated output by a previously called script. String begins either finished <FINISHED> or <NOT FINSIHED>,
indicating whether the the script is still active. If the output of a finished script has been collected,
the internal buffer will be cleared until the next tool call of collect_practice_skill_response.

Prerequisites:
    - TRACE32 PowerView must be running
    - API Port must be enabled in PowerView configuration
    - execute_practice_skill must have been called.

Important: Only call this tool after a execute_practice_skill that has NOT finished execution (indicated by its result)."
    )]
    async fn collect_practice_skill_response(&self) -> Result<String, String> {
        loop {
            sleep(Duration::from_millis(100)).await;

            let mut sink = SINK.lock().await;

            let mut response: String = if sink.active {
                "<NOT FINISHED>".to_string()
            } else {
                drop(sink);
                // let the task fetch the rest
                sleep(Duration::from_millis(10)).await;
                sink = SINK.lock().await;
                "<FINISHED>".to_string()
            };
            response.push_str("\n<CONTENT>\n");
            response += &sink.received_data;

            if sink.active {
                if self.empty_stack()? {
                    t32_cmd("AREA.CLOSE".to_string())?;
                    sink.active = false;
                    drop(sink);
                    // script finished: loop once more to collect the remaining output
                    continue;
                }
            } else {
                sink.received_data.clear();
            }

            return Ok(response);
        }
    }

    #[tool(
        name = "abort_practice_skill",
        description = "Aborts the execution of a running PRACTICE skill.

        Use this tool when you are certain that the PRACTICE skill should have terminated by now.
        Communicate this with the user and hint at the PLIST command that can be used in the GUI
        to show the currently active script."
    )]
    async fn abort_practice_skill(&self) -> Result<(), String> {
        self.establish_debug_connection(true)?;
        t32_cmd("END".to_string())?;
        let mut sink = SINK.lock().await;
        sink.active = false;

        Ok(())
    }

    #[tool(
        name = "execute_practice",
        description = "Execute PRACTICE commands via Remote API"
    )]
    fn execute_practice(
        &self,
        Parameters(PracticeRequest { practice, execute }): Parameters<PracticeRequest>,
    ) -> Result<(), String> {
        // Establish the debug connection first to avoid the creation of a temp file without feedback
        self.establish_debug_connection(false)?;

        let practice = practice.trim();
        let cmd = if !execute || practice.contains("\n") || practice.contains(";") {
            // PRACTICE script will always be executed as files
            // Comments cannot be passed to the command line

            // For some unknown reason, the tempfile's write function does not create a file
            // Use the library for unique name generation and create the file separately
            let path = match Builder::new().prefix("t32chat_").suffix(".cmm").tempfile() {
                Ok(file) => file.path().to_string_lossy().to_string(),
                Err(e) => {
                    return Err(format!("Temp file creation failed ({e})"));
                }
            };

            let mut script = match File::create(&path) {
                Ok(file) => file,
                Err(e) => {
                    return Err(format!("File creation failed ({e})"));
                }
            };

            if let Err(e) = script.write_all(practice.as_bytes()) {
                return Err(format!("File creation failed ({e})"));
            };

            let op = if execute { "DO" } else { "PEDIT" };
            format!("{op} \"{path}\"").to_string()
        } else {
            practice.to_string()
        };

        t32_cmd(cmd)?;

        Ok(())
    }
}

#[tool_handler]
impl ServerHandler for T32 {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("TRACE32 Remote API".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

/// npx @modelcontextprotocol/inspector cargo run
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let random = rand::rng().random::<u64>();
    let mut options = T32Options {
        unlock_all_tools: false,
        skills_base: "".to_string(),
        port: 20000,
        pipe_name: format!("t32chat.{random}"),
    };

    let mut args = env::args().skip(1); // skip program name
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "--t32chat" => {
                options.unlock_all_tools = true;
            }
            "--skills" => match args.next() {
                Some(base) => options.skills_base = base.trim_matches('"').to_string(),
                None => {
                    print_help();
                    return Ok(());
                }
            },
            "--port" => match args.next() {
                Some(port) => {
                    options.port = port.parse().expect("Not a number!");
                }
                None => {
                    print_help();
                    return Ok(());
                }
            },
            _ => {}
        }
    }

    if options.skills_base.is_empty() {
        let path: PathBuf = if cfg!(target_os = "windows") {
            let appdata = env::var("APPDATA").expect(
                "Skills base directory cannot be found (APPDATA not set)",
            );
            [appdata.as_str(), "TRACE32", "skills"].iter().collect()
        } else {
            // Linux/macOS: default to $XDG_CONFIG_HOME or ~/.config
            let config_home = env::var("XDG_CONFIG_HOME")
                .ok()
                .filter(|s| !s.is_empty())
                .or_else(|| env::var("HOME").ok().map(|h| format!("{h}/.config")))
                .expect("Skills base directory cannot be found (HOME not set)");
            [config_home.as_str(), "TRACE32", "skills"].iter().collect()
        };
        options.skills_base = path.to_string_lossy().to_string();
    }

    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting T32 MCP server");
    let service = T32::new(options).serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}

fn print_help() {
    println!(
        "t32mcp [--t32chat] [--port PORT] [--skills DIR]

Options:
  --port PORT      TRACE32 Remote API TCP port (default: 20000)
  --skills DIR     Sets the skills base directory to DIR (default: %APPDATA%/TRACE32/skill)
  --t32chat        Unlocks execute_practice tool
  -h, --help       Print this help message"
    );
}
