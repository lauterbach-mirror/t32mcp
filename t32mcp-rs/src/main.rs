// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! t32mcp: Multi-threaded MCP server by Lauterbach for agentic debugging.
//!
//! rmcp (and tokio) dependencies has been dropped for better control.
//!
//! MCP types are using typify based on the official JSON MCP schemas.
//! The protocol currently only defines a handful messages that are relevant
//! for us, so we use serde for serialization.

use std::{
    collections::VecDeque, io::Write, net::IpAddr, net::SocketAddr, path::PathBuf,
    process::ExitCode, sync::mpsc,
};

use serde_json::Deserializer;

use t32mcp::{
    docs, execution_policy, mcp, t32,
    tools::{
        self, trace32_get_status, trace32_manage_instances, trace32_run_command, trace32_run_script,
    },
};

// Stay compatible with older MCP versions

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::wrong_self_convention)]
#[rustfmt::skip]
mod mcp_2025_11_25;
const MCP_VERSION_2025_11_25: &str = "2025-11-25";

const MCP_VERSION_CURRENT: &str = "2026-07-28";

/// Return errors instead of panicking
fn jsonrpc_error(req: &mcp::JsonrpcRequest, code: i64, message: &str) -> serde_json::Value {
    serde_json::json!({
        "jsonrpc": "2.0",
        "id": &req.id,
        "error": { "code": code, "message": message },
    })
}

fn process_request(req: &mcp::JsonrpcRequest, ctx: &mut tools::Context) -> serde_json::Value {
    eprintln!("processing request {}: {}", req.id, req.method);
    match req.method.as_str() {
        "initialize" => {
            let Some(protocol_version) = req.params.get("protocolVersion").and_then(|v| v.as_str())
            else {
                return jsonrpc_error(
                    req,
                    -32602,
                    "Invalid params: initialize request missing protocolVersion",
                );
            };

            // Version negotiation: if the requested version is not supported, the server
            // answers with a version it supports and the client decides whether to continue.
            if protocol_version != MCP_VERSION_2025_11_25 {
                eprintln!(
                    "client requested MCP version {protocol_version}, answering with {MCP_VERSION_2025_11_25}"
                );
            }
            let result = mcp_2025_11_25::InitializeResult {
                capabilities: mcp_2025_11_25::ServerCapabilities {
                    logging: serde_json::Map::new(),
                    prompts: Some(mcp_2025_11_25::ServerCapabilitiesPrompts {
                        list_changed: Some(false),
                    }),
                    resources: Some(mcp_2025_11_25::ServerCapabilitiesResources {
                        list_changed: Some(false),
                        subscribe: Some(false),
                    }),
                    tools: Some(mcp_2025_11_25::ServerCapabilitiesTools {
                        list_changed: Some(false),
                    }),
                    ..Default::default()
                },
                instructions: Some(ctx.instructions.clone()),
                meta: serde_json::Map::new(),
                protocol_version: MCP_VERSION_2025_11_25.to_string(),
                server_info: mcp_2025_11_25::Implementation {
                    description: None,
                    icons: Vec::new(),
                    name: "t32mcp".to_string(),
                    title: None,
                    version: env!("CARGO_PKG_VERSION").to_owned(),
                    website_url: None,
                },
            };

            serde_json::json!({
                "jsonrpc": "2.0",
                "id": &req.id,
                "result": result,
            })
        }
        "server/discover" => {
            let Some(protocol_version) = req
                .params
                .get("_meta")
                .and_then(|v| v.get("io.modelcontextprotocol/protocolVersion"))
                .and_then(|v| v.as_str())
            else {
                return jsonrpc_error(
                    req,
                    -32602,
                    "Invalid params: server/discover request missing protocolVersion",
                );
            };

            if protocol_version != MCP_VERSION_CURRENT {
                return jsonrpc_error(
                    req,
                    -32602,
                    &format!(
                        "Invalid params: server/discover expects protocolVersion \
                         {MCP_VERSION_CURRENT}, got {protocol_version}"
                    ),
                );
            }

            let result = mcp::DiscoverResult {
                cache_scope: mcp::DiscoverResultCacheScope::Public,
                capabilities: mcp::ServerCapabilities {
                    prompts: Some(mcp::ServerCapabilitiesPrompts {
                        list_changed: Some(false),
                    }),
                    resources: Some(mcp::ServerCapabilitiesResources {
                        list_changed: Some(false),
                        subscribe: Some(false),
                    }),
                    tools: Some(mcp::ServerCapabilitiesTools {
                        list_changed: Some(false),
                    }),
                    ..Default::default()
                },
                instructions: Some(ctx.instructions.clone()),
                meta: Some(mcp::ResultMetaObject {
                    io_modelcontextprotocol_server_info: Some(mcp::Implementation {
                        description: None,
                        icons: Vec::new(),
                        name: "t32mcp".to_string(),
                        title: None,
                        version: env!("CARGO_PKG_VERSION").to_owned(),
                        website_url: None,
                    }),
                }),
                result_type: "complete".to_string(),
                supported_versions: vec![
                    MCP_VERSION_2025_11_25.to_string(),
                    MCP_VERSION_CURRENT.to_string(),
                ],
                ttl_ms: 3600000,
            };

            serde_json::json!({
                "jsonrpc": "2.0",
                "id": &req.id,
                "result": result,
            })
        }
        "tools/list" => {
            let result = mcp::ListToolsResult {
                cache_scope: mcp::ListToolsResultCacheScope::Public,
                meta: None,
                next_cursor: None,
                result_type: "complete".to_string(),
                tools: ctx.tools.iter().map(|e| e.tool.clone()).collect(),
                ttl_ms: 3600000,
            };
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": &req.id,
                "result": result,
            })
        }
        "resources/list" => {
            let result = mcp::ListResourcesResult {
                cache_scope: mcp::ListResourcesResultCacheScope::Public,
                meta: None,
                next_cursor: None,
                result_type: "complete".to_string(),
                resources: Vec::new(),
                ttl_ms: 3600000,
            };
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": &req.id,
                "result": result,
            })
        }
        "prompts/list" => {
            let result = mcp::ListPromptsResult {
                cache_scope: mcp::ListPromptsResultCacheScope::Public,
                meta: None,
                next_cursor: None,
                result_type: "complete".to_string(),
                prompts: Vec::new(),
                ttl_ms: 3600000,
            };
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": &req.id,
                "result": result,
            })
        }
        "resources/templates/list" => {
            let result = mcp::ListResourceTemplatesResult {
                cache_scope: mcp::ListResourceTemplatesResultCacheScope::Public,
                meta: None,
                next_cursor: None,
                result_type: "complete".to_string(),
                resource_templates: Vec::new(),
                ttl_ms: 3600000,
            };
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": &req.id,
                "result": result,
            })
        }
        "tools/call" => {
            let tool_name = req.params.get("name").and_then(|v| v.as_str());
            let arguments = req.params.get("arguments");
            let result = match tool_name {
                Some(name) => match ctx.tools.iter().find(|e| e.tool.name == name) {
                    Some(entry) => (entry.handler)(arguments, ctx),
                    None => mcp::CallToolResult {
                        content: vec![mcp::ContentBlock::TextContent(mcp::TextContent {
                            annotations: None,
                            meta: None,
                            text: format!("unknown tool: {name}"),
                            type_: "text".to_string(),
                        })],
                        is_error: Some(true),
                        meta: None,
                        result_type: "complete".to_string(),
                        structured_content: None,
                    },
                },
                None => mcp::CallToolResult {
                    content: vec![mcp::ContentBlock::TextContent(mcp::TextContent {
                        annotations: None,
                        meta: None,
                        text: "missing tool name".to_string(),
                        type_: "text".to_string(),
                    })],
                    is_error: Some(true),
                    meta: None,
                    result_type: "complete".to_string(),
                    structured_content: None,
                },
            };
            serde_json::json!({
                "jsonrpc": "2.0",
                "id": &req.id,
                "result": result,
            })
        }
        "ping" => serde_json::json!({
            "jsonrpc": "2.0",
            "id": &req.id,
            "result": {},
        }),
        _ => {
            eprintln!("request method not implemented: {}", req.method);
            jsonrpc_error(req, -32601, &format!("Method not found: {}", req.method))
        }
    }
}

/// Compares in a time that does not depend on where the first difference is.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Create a new HTTP header
fn http_into_header(name: &str, value: &str) -> tiny_http::Header {
    tiny_http::Header::from_bytes(name.as_bytes(), value.as_bytes()).unwrap()
}

/// The value of the first header `name` of an HTTP request
fn http_from_header(request: &tiny_http::Request, name: &'static str) -> Option<String> {
    request
        .headers()
        .iter()
        .find(|h| h.field.equiv(name))
        .map(|h| h.value.to_string())
}

/// Send a HTTP response with CORS headers
fn http_respond<R: std::io::Read>(
    request: tiny_http::Request,
    mut response: tiny_http::Response<R>,
    cors: &[tiny_http::Header],
) {
    for h in cors {
        response.add_header(h.clone());
    }
    let _ = request.respond(response);
}

fn remote_mcp(
    address: IpAddr,
    port: u16,
    token: Option<String>,
    allowed_origins: Vec<String>,
    mut ctx: tools::Context,
) -> ExitCode {
    let address = SocketAddr::new(address, port);
    let server = match tiny_http::Server::http(address) {
        Ok(server) => server,
        Err(e) => {
            eprintln!("could not create http server: {}", e);
            return ExitCode::FAILURE;
        }
    };

    eprintln!("Remote MCP server started. Listening on http://{}", address);
    if token.is_none() {
        eprintln!("warning: no --token or T32MCP_TOKEN set, HTTP requests are not authenticated");
    }

    loop {
        let mut request = match server.recv() {
            Ok(rq) => rq,
            Err(e) => {
                println!("error while receiving request: {}", e);
                break;
            }
        };

        // Prevent that any web page could reach the server via localhost
        let cors = match http_from_header(&request, "Origin") {
            None => Vec::new(),
            Some(origin)
                if allowed_origins
                    .iter()
                    .any(|o| o.eq_ignore_ascii_case(&origin)) =>
            {
                vec![
                    http_into_header("Access-Control-Allow-Origin", &origin),
                    http_into_header("Vary", "Origin"),
                ]
            }
            Some(origin) => {
                eprintln!("rejected request from origin {origin} (see --allow-origin)");
                let _ = request.respond(tiny_http::Response::empty(403));
                continue;
            }
        };

        // Handle CORS preflight request
        if request.method() == &tiny_http::Method::Options && !cors.is_empty() {
            let mut response = tiny_http::Response::empty(204)
                .with_header(http_into_header("Access-Control-Allow-Methods", "POST"))
                .with_header(http_into_header("Access-Control-Max-Age", "600"));

            if let Some(h) = http_from_header(&request, "Access-Control-Request-Headers") {
                response.add_header(http_into_header("Access-Control-Allow-Headers", &h));
            }

            http_respond(request, response, &cors);
            continue;
        }

        // Without a token, every request is accepted
        let authorized = if let Some(token) = &token {
            let expected = format!("Bearer {token}");
            request
                .headers()
                .iter()
                .filter(|h| h.field.equiv("Authorization"))
                .any(|h| constant_time_eq(h.value.as_bytes(), expected.as_bytes()))
        } else {
            true
        };

        if !authorized {
            let response = tiny_http::Response::empty(401)
                .with_header(http_into_header("WWW-Authenticate", "Bearer"));
            http_respond(request, response, &cors);
            continue;
        }

        if request.method() != &tiny_http::Method::Post {
            http_respond(request, tiny_http::Response::empty(405), &cors);
            continue;
        }

        let mut responses: Vec<serde_json::Value> = Vec::new();
        for message in
            Deserializer::from_reader(request.as_reader()).into_iter::<mcp::JsonrpcMessage>()
        {
            match &message {
                Ok(msg) => {
                    if let mcp::JsonrpcMessage::Request(req) = msg {
                        let response = process_request(req, &mut ctx);
                        responses.push(response);
                    } else {
                        eprintln!("request expected, but got: {:?}", msg);
                    }
                }
                Err(e) => eprintln!("error parsing JSON-RPC message: {e}"),
            }
        }

        // NOTE: Not tested yet because no server-side events yet implemented
        responses.extend(ctx.event_queue.drain(..));

        let body = match responses.len() {
            0 => String::new(),
            1 => serde_json::to_string(&responses[0]).unwrap(),
            _ => serde_json::to_string(&responses).unwrap(),
        };

        let response = tiny_http::Response::from_string(body)
            .with_header(http_into_header("Content-Type", "application/json"));
        http_respond(request, response, &cors);
    }

    ExitCode::SUCCESS
}

fn local_mcp(mut ctx: tools::Context) -> ExitCode {
    eprintln!("Local MCP server started. Listening on stdin");
    let deserializer = Deserializer::from_reader(std::io::stdin().lock());
    let stream = deserializer.into_iter::<mcp::JsonrpcMessage>();

    for message in stream {
        match &message {
            Ok(msg) => {
                if let mcp::JsonrpcMessage::Request(req) = msg {
                    let response = process_request(req, &mut ctx);
                    let json = serde_json::to_string(&response).unwrap();
                    println!("{}", json);
                    let _ = std::io::stdout().flush();
                } else {
                    eprintln!("request expected, but got: {:?}", msg);
                }
            }
            Err(e) => {
                eprintln!("error parsing JSON-RPC message: {e}");
            }
        };

        while let Some(event) = ctx.event_queue.pop_front() {
            let json = serde_json::to_string(&event).unwrap();
            println!("{}", json);
        }
        let _ = std::io::stdout().flush();
    }

    ExitCode::SUCCESS
}

fn print_version() {
    let v = env!("CARGO_PKG_VERSION").to_owned();
    let v = format!("t32mcp v{v}");

    #[cfg(debug_assertions)]
    let v = format!("{v}.debug");

    println!(
        "{v} (script interface {})",
        execution_policy::SCRIPT_INTERFACE_VERSION
    );
}

fn print_help() {
    print_version();
    println!(
        "General Options:
  --version                Print version number and exit
  -h, --help               Print this help message and exit

MCP Transport (default: stdio):
  --http PORT              Run as Remote MCP, served over HTTP
  --bind ADDRESS           Address to bind on (default: 127.0.0.1)
  --token TOKEN            Authentication Token (use with --http)
  --allow-origin ORIGIN    Allowed remote hosts (use with --http)

PowerView Connection:
  --execution-policy P     Execution Policy (required):
                           unrestricted, scripts, strict
  --port PORT              TRACE32 Remote API TCP port (default: 20000),
                           repeatable for AMP sessions. The agent can also
                           add and remove ports dynamically itself using the
                           trace32_manage_instances tool(disabled in
                           \"strict\" execution policy mode).
  --skills DIR             Root location of agent skills (repeatable,
                           required by execution policy \"strict\")"
    );
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut bind_address: IpAddr = "127.0.0.1".parse().expect("default is valid");
    let mut http_port: Option<u16> = None;
    let mut token: Option<String> = None;
    let mut allowed_origins: Vec<String> = Vec::new();
    let mut t32_ports: Vec<u16> = Vec::new();
    let mut policy: Option<execution_policy::ExecutionPolicy> = None;
    let mut skill_roots: Vec<PathBuf> = Vec::new();
    let mut skills_source = docs::DEFAULT_SKILLS_SOURCE.to_string();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return ExitCode::SUCCESS;
            }
            "--version" => {
                print_version();
                return ExitCode::SUCCESS;
            }
            "--execution-policy" => match args
                .next()
                .as_deref()
                .map(execution_policy::ExecutionPolicy::parse)
            {
                Some(Some(p)) => {
                    policy = match policy {
                        Some(prev) => Some(prev.max(p)),
                        None => Some(p),
                    };
                }
                _ => {
                    eprintln!("error: --execution-policy requires unrestricted, scripts or strict");
                    return ExitCode::from(2);
                }
            },
            // just for debugging the mcp server
            "--skills-source" => match args.next() {
                Some(url) if url.starts_with("http://") || url.starts_with("https://") => {
                    skills_source = url.trim_end_matches('/').to_string();
                }
                _ => {
                    eprintln!("error: --skills-source requires an http(s) URL");
                    return ExitCode::from(2);
                }
            },
            "--skills" => match args.next() {
                Some(dir) => match std::path::absolute(&dir) {
                    Ok(root) if root.is_dir() => skill_roots.push(root),
                    _ => {
                        eprintln!("error: --skills is not a directory: {dir}");
                        return ExitCode::from(2);
                    }
                },
                None => {
                    eprintln!("error: --skills requires a directory argument");
                    return ExitCode::from(2);
                }
            },
            "--http" => match args.next() {
                Some(port) => match port.parse() {
                    Ok(p) => http_port = Some(p),
                    Err(_) => {
                        eprintln!("error: invalid --http port: {port}");
                        return ExitCode::from(2);
                    }
                },
                None => {
                    eprintln!("error: --http requires a port argument");
                    return ExitCode::from(2);
                }
            },
            "--token" => match args.next() {
                Some(t) if !t.is_empty() => token = Some(t),
                _ => {
                    eprintln!("error: --token requires a non-empty token");
                    return ExitCode::from(2);
                }
            },
            "--bind" => match args.next() {
                Some(t) if !t.is_empty() => match t.parse::<IpAddr>() {
                    Ok(ip) => bind_address = ip,
                    Err(_) => {
                        eprintln!("error: --bind requires an IP address, e.g. 127.0.0.1");
                        return ExitCode::from(2);
                    }
                },
                _ => {
                    eprintln!("error: --bind requires an IP address, e.g. 127.0.0.1");
                    return ExitCode::from(2);
                }
            },
            "--allow-origin" => match args.next() {
                Some(o) if o.starts_with("http://") || o.starts_with("https://") => {
                    allowed_origins.push(o.trim_end_matches('/').to_string());
                }
                _ => {
                    eprintln!(
                        "error: --allow-origin requires an origin, e.g. http://localhost:6274"
                    );
                    return ExitCode::from(2);
                }
            },
            "--port" => match args.next() {
                Some(port) => match port.parse() {
                    Ok(p) => {
                        if t32_ports.contains(&p) {
                            eprintln!("warning: ignoring duplicate --port: {p}");
                        } else {
                            t32_ports.push(p);
                        }
                    }
                    Err(_) => {
                        eprintln!("error: invalid --port: {port}");
                        return ExitCode::from(2);
                    }
                },
                None => {
                    eprintln!("error: --port requires a port argument");
                    return ExitCode::from(2);
                }
            },
            _ => {
                eprintln!("error: unknown argument: {arg}");
                return ExitCode::from(2);
            }
        }
    }

    let policy = match policy {
        Some(p) => p,
        None => {
            eprintln!("error: --execution-policy must be set: [unrestricted, scripts, strict]");
            return ExitCode::FAILURE;
        }
    };

    if policy == execution_policy::ExecutionPolicy::Strict && skill_roots.is_empty() {
        eprintln!("error: --execution-policy strict requires --skills <directory>");
        return ExitCode::from(2);
    }

    if policy == execution_policy::ExecutionPolicy::Strict && t32_ports.is_empty() {
        eprintln!("error: --execution-policy strict requires --port <port>");
        return ExitCode::from(2);
    }

    // --token wins over the environment variable
    // note that an argument is visible in the process list
    let token = token.or_else(|| std::env::var("T32MCP_TOKEN").ok().filter(|t| !t.is_empty()));
    if token.is_some() && http_port.is_none() {
        eprintln!("warning: --token and T32MCP_TOKEN are ignored without --http");
    }
    if !allowed_origins.is_empty() && http_port.is_none() {
        eprintln!("warning: --allow-origin is ignored without --http");
    }

    // A non-loopback --bind makes the server reachable from the network:
    // without authentication and an allowed origin, any host could reach it
    if http_port.is_some() && !bind_address.is_loopback() {
        if token.is_none() {
            eprintln!(
                "error: --bind {bind_address} is not a loopback address, it requires --token (or T32MCP_TOKEN)"
            );
            return ExitCode::from(2);
        }
        if allowed_origins.is_empty() {
            eprintln!(
                "error: --bind {bind_address} is not a loopback address, it requires --allow-origin"
            );
            return ExitCode::from(2);
        }
    }

    let tools = match policy {
        execution_policy::ExecutionPolicy::Unrestricted => vec![
            trace32_get_status::entry(),
            trace32_run_script::entry(policy, &skills_source),
            trace32_manage_instances::entry(),
            trace32_run_command::entry(),
        ],
        execution_policy::ExecutionPolicy::Scripts => vec![
            trace32_get_status::entry(),
            trace32_run_script::entry(policy, &skills_source),
            trace32_manage_instances::entry(),
        ],
        execution_policy::ExecutionPolicy::Strict => vec![
            trace32_get_status::entry(),
            trace32_run_script::entry(policy, &skills_source),
        ],
    };

    let mut worker = t32::Worker::new(t32_ports);
    let (worker_tx, worker_rx) = mpsc::channel::<t32::WorkerRequest>();
    let worker_handle = std::thread::spawn(move || {
        worker.run(worker_rx);
    });

    let ctx = tools::Context {
        event_queue: VecDeque::new(),
        instructions: docs::server_instructions(policy, &skills_source),
        policy,
        skill_roots,
        tools,
        worker_tx,
    };

    let exit_code = if let Some(port) = http_port {
        remote_mcp(bind_address, port, token, allowed_origins, ctx)
    } else {
        local_mcp(ctx)
    };

    let _ = worker_handle.join();

    exit_code
}
