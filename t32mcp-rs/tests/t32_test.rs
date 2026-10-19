// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Mutex, MutexGuard, mpsc};
use std::time::Duration;

use t32mcp::t32;

/// All tests use the same PowerView instance, which runs only one script at a
/// time. Serialize the tests, also if a previous test panicked.
static T32_LOCK: Mutex<()> = Mutex::new(());

fn t32_lock() -> MutexGuard<'static, ()> {
    T32_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// Remote API port of PowerView. T32MCP_T32_PORT overrides it, e.g. if
/// 20000 is already in use.
fn t32_port() -> u16 {
    std::env::var("T32MCP_T32_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(20000)
}

#[test]
fn connect() {
    let _guard = t32_lock();
    // Connections are established lazily on the first request: run a minimal
    // script to verify the connection including the output pipe.
    let mut worker = t32::Worker::new(vec![t32_port()]);
    let script_path = format!("{}/tests/print.cmm", env!("CARGO_MANIFEST_DIR"));

    let (tx, rx) = mpsc::channel();
    let request = t32::WorkerRequest::ExecuteScript {
        script_path,
        args: " \"1.\"".to_string(),
        port: None,
        core: None,
        timeout: Duration::from_secs(5),
        if_busy: t32::IfBusy::Fail,
        device_prompt: "B::".to_string(),
        response_tx: tx,
    };

    let (worker_tx, worker_rx) = mpsc::channel::<t32::WorkerRequest>();
    worker_tx.send(request).unwrap();

    std::thread::spawn(move || {
        worker.run(worker_rx);
    });

    let result = rx
        .recv()
        .expect("worker thread dropped the response channel");

    assert!(result.is_ok(), "connection failed: {:?}", result);
}

#[test]
fn print() {
    let _guard = t32_lock();
    let mut worker = t32::Worker::new(vec![t32_port()]);
    let script_path = format!("{}/tests/print.cmm", env!("CARGO_MANIFEST_DIR"));

    let expected_lines = 10;
    let (tx, rx) = mpsc::channel();
    let request = t32::WorkerRequest::ExecuteScript {
        script_path,
        args: format!(" \"{}.\"", expected_lines),
        port: None,
        core: None,
        timeout: Duration::from_secs(5),
        if_busy: t32::IfBusy::Fail,
        device_prompt: "B::".to_string(),
        response_tx: tx,
    };

    let (worker_tx, worker_rx) = mpsc::channel::<t32::WorkerRequest>();
    worker_tx.send(request).unwrap();

    std::thread::spawn(move || {
        worker.run(worker_rx);
    });

    let result = rx
        .recv()
        .expect("worker thread dropped the response channel");

    assert!(result.is_ok(), "script execution failed: {:?}", result);
    let output = result.unwrap();
    assert_eq!(output.lines().count(), expected_lines);
}

#[test]
fn winprint() {
    let _guard = t32_lock();
    let mut worker = t32::Worker::new(vec![t32_port()]);
    let script_path = format!("{}/tests/winprint.cmm", env!("CARGO_MANIFEST_DIR"));

    let repeat = 10;
    let (tx, rx) = mpsc::channel();
    let request = t32::WorkerRequest::ExecuteScript {
        script_path,
        args: format!(" \"{}.\"", repeat),
        port: None,
        core: None,
        timeout: Duration::from_secs(10),
        if_busy: t32::IfBusy::Fail,
        device_prompt: "B::".to_string(),
        response_tx: tx,
    };

    let (worker_tx, worker_rx) = mpsc::channel::<t32::WorkerRequest>();
    worker_tx.send(request).unwrap();

    std::thread::spawn(move || {
        worker.run(worker_rx);
    });

    let result = rx
        .recv()
        .expect("worker thread dropped the response channel");

    assert!(result.is_ok(), "script execution failed: {:?}", result);
    let output = result.unwrap();

    let lines = output.lines().count();
    assert!(lines > 1);
    assert_eq!(lines % repeat, 0)
}

#[test]
fn message() {
    let _guard = t32_lock();
    let worker = start_worker();

    // PWD reports the directory as a TRACE32 message, not via PRINT. Messages
    // always go to A000, so they are only returned if A000 is piped.
    let output = execute(&worker, "message.cmm", t32::IfBusy::Fail).expect("should succeed");
    let lines: Vec<&str> = output
        .lines()
        .map(|line| line.trim().trim_end_matches(['/', '\\']))
        .filter(|line| !line.is_empty())
        .collect();
    assert_eq!(
        lines.len(),
        2,
        "expected PWD message and PRINT, got: {output}"
    );
    assert_eq!(lines[0], lines[1], "expected PWD message, got: {output}");
}

#[test]
fn noreturn() {
    let _guard = t32_lock();
    let worker = start_worker();

    // A script paused at STOP keeps waiting and is reported, not terminated.
    let output =
        execute(&worker, "noreturn.cmm", t32::IfBusy::Fail).expect("paused script should succeed");
    assert!(
        output.contains("Hello"),
        "expected output before STOP, got: {output}"
    );
    assert!(!output.contains("World"), "script ran past STOP: {output}");
    assert!(
        output.contains("paused"),
        "expected paused note, got: {output}"
    );
    assert!(
        output.contains("noreturn.cmm:2: STOP"),
        "expected call stack, got: {output}"
    );

    end_scripts(&worker);
}

#[test]
fn dialog() {
    let _guard = t32_lock();
    let worker = start_worker();

    // The script waits in a named dialog. A second script fills it in and
    // presses the button, which continues the first script.
    let output =
        execute(&worker, "dialog.cmm", t32::IfBusy::Fail).expect("dialog script should pause");
    assert!(
        output.contains("t32mcp_test_dlg"),
        "expected dialog name, got: {output}"
    );

    let output =
        execute(&worker, "dialog_fill.cmm", t32::IfBusy::Nest).expect("dialog fill should succeed");
    assert!(
        output.contains("value=hello"),
        "first script did not continue: {output}"
    );
    assert!(
        !output.contains("paused"),
        "no script should wait anymore: {output}"
    );

    end_scripts(&worker);
}

#[test]
fn busy() {
    let _guard = t32_lock();
    let worker = start_worker();

    execute(&worker, "noreturn.cmm", t32::IfBusy::Fail).expect("script should pause");

    // A paused script is reported with its call stack, not terminated.
    let err = execute(&worker, "error.cmm", t32::IfBusy::Fail).expect_err("should be busy");
    assert!(err.contains("busy"), "expected busy error, got: {err}");
    assert!(
        err.contains("noreturn.cmm:2: STOP"),
        "expected call stack, got: {err}"
    );

    // terminate ends it first.
    let output = execute(&worker, "noreturn.cmm", t32::IfBusy::Terminate)
        .expect("script should pause again");
    assert!(output.contains("Hello"), "expected output, got: {output}");

    end_scripts(&worker);

    // A running script cannot be nested on.
    let err = execute(&worker, "loop.cmm", t32::IfBusy::Fail).expect_err("should time out");
    assert!(
        err.contains("keeps running"),
        "expected running note, got: {err}"
    );
    assert!(err.contains("loop.cmm:"), "expected location, got: {err}");
    let err = execute(&worker, "print.cmm", t32::IfBusy::Nest).expect_err("should refuse");
    assert!(
        err.contains("Cannot nest"),
        "expected nest refusal, got: {err}"
    );

    end_scripts(&worker);
}

#[test]
fn area_switch() {
    let _guard = t32_lock();
    let worker = start_worker();

    // Output after the AREA switch is missing: the result says where it went.
    let output = execute(&worker, "area_switch.cmm", t32::IfBusy::Fail).expect("should succeed");
    assert!(
        output.contains("before"),
        "expected output before the switch: {output}"
    );
    assert!(
        !output.contains("\nafter"),
        "output after the switch is unexpected: {output}"
    );
    assert!(
        output.contains("changed to t32mcp_test_area"),
        "expected AREA warning, got: {output}"
    );

    for command in ["AREA.Select A000", "AREA.Delete t32mcp_test_area"] {
        let (tx, rx) = mpsc::channel();
        worker
            .send(t32::WorkerRequest::RunCommand {
                command: command.to_string(),
                port: None,
                core: None,
                device_prompt: "B::".to_string(),
                response_tx: tx,
            })
            .unwrap();
        let _ = rx.recv();
    }
}

fn start_worker() -> mpsc::Sender<t32::WorkerRequest> {
    let mut worker = t32::Worker::new(vec![t32_port()]);
    let (worker_tx, worker_rx) = mpsc::channel::<t32::WorkerRequest>();
    std::thread::spawn(move || {
        worker.run(worker_rx);
    });
    worker_tx
}

fn execute(
    worker: &mpsc::Sender<t32::WorkerRequest>,
    script: &str,
    if_busy: t32::IfBusy,
) -> Result<String, String> {
    let (tx, rx) = mpsc::channel();
    worker
        .send(t32::WorkerRequest::ExecuteScript {
            script_path: format!("{}/tests/{script}", env!("CARGO_MANIFEST_DIR")),
            args: String::new(),
            port: None,
            core: None,
            timeout: Duration::from_secs(3),
            if_busy,
            device_prompt: "B::".to_string(),
            response_tx: tx,
        })
        .unwrap();
    rx.recv()
        .expect("worker thread dropped the response channel")
}

/// Terminate paused scripts, so they do not affect later tests.
fn end_scripts(worker: &mpsc::Sender<t32::WorkerRequest>) {
    let (tx, rx) = mpsc::channel();
    worker
        .send(t32::WorkerRequest::RunCommand {
            command: "END".to_string(),
            port: None,
            core: None,
            device_prompt: "B::".to_string(),
            response_tx: tx,
        })
        .unwrap();
    rx.recv()
        .expect("worker thread dropped the response channel")
        .expect("END failed");
}

#[test]
fn error() {
    let _guard = t32_lock();
    let mut worker = t32::Worker::new(vec![t32_port()]);
    let script_path = format!("{}/tests/error.cmm", env!("CARGO_MANIFEST_DIR"));

    let (tx, rx) = mpsc::channel();
    let request = t32::WorkerRequest::ExecuteScript {
        script_path,
        args: String::new(),
        port: None,
        core: None,
        timeout: Duration::from_secs(3),
        if_busy: t32::IfBusy::Fail,
        device_prompt: "B::".to_string(),
        response_tx: tx,
    };

    let (worker_tx, worker_rx) = mpsc::channel::<t32::WorkerRequest>();
    worker_tx.send(request).unwrap();

    std::thread::spawn(move || {
        worker.run(worker_rx);
    });

    let result = rx
        .recv()
        .expect("worker thread dropped the response channel");

    let err = result.expect_err("script should have stopped at the error");
    assert!(
        err.contains("unknown command") && err.contains("Command: UnknownCommand"),
        "expected error message and command, got: {err}"
    );
    assert!(
        err.contains("HI"),
        "expected output before the error, got: {err}"
    );
}

#[test]
fn close() {
    let _guard = t32_lock();
    let mut worker = t32::Worker::new(vec![t32_port()]);
    let script_path = format!("{}/tests/close.cmm", env!("CARGO_MANIFEST_DIR"));

    let (tx, rx) = mpsc::channel();
    let request = t32::WorkerRequest::ExecuteScript {
        script_path,
        args: String::new(),
        port: None,
        core: None,
        timeout: Duration::from_secs(3),
        if_busy: t32::IfBusy::Fail,
        device_prompt: "B::".to_string(),
        response_tx: tx,
    };

    let (worker_tx, worker_rx) = mpsc::channel::<t32::WorkerRequest>();
    worker_tx.send(request).unwrap();

    std::thread::spawn(move || {
        worker.run(worker_rx);
    });

    let result = rx
        .recv()
        .expect("worker thread dropped the response channel");

    // Select the default AREA again, before the asserts: close.cmm leaves
    // the closed AREA "new" selected, so PRINT output would be invisible.
    let (tx, rx) = mpsc::channel();
    worker_tx
        .send(t32::WorkerRequest::RunCommand {
            command: "AREA.Select A000".to_string(),
            port: None,
            core: None,
            device_prompt: "B::".to_string(),
            response_tx: tx,
        })
        .unwrap();
    let _ = rx.recv();

    let err = result.expect_err("closing the AREA should fail the call");
    assert!(
        err.contains("AREA output pipe was closed"),
        "expected closed pipe error, got: {err}"
    );
}

#[test]
fn query() {
    let _guard = t32_lock();
    let worker = start_worker();

    // One entry per instance with its port, the PRACTICE state and the cores
    // with their TRACE32 core number. Exactly one core is selected.
    let (tx, rx) = mpsc::channel();
    worker
        .send(t32::WorkerRequest::QueryTargetSystem {
            timeout: Duration::from_secs(5),
            response_tx: tx,
        })
        .unwrap();
    let result = rx.recv().unwrap().expect("should succeed");
    let instance = &result["instances"][0];
    assert_eq!(instance["port"], t32_port(), "got: {result}");
    assert_eq!(instance["state"], "connected", "got: {result}");
    assert!(instance["practice"]["state"].is_string(), "got: {result}");
    let cores = instance["cores"].as_array().expect("cores");
    assert!(!cores.is_empty(), "got: {result}");
    assert!(cores.iter().all(|c| c["core"].is_u64()), "got: {result}");
    let selected = cores.iter().filter(|c| c["selected"] == true).count();
    assert_eq!(selected, 1, "got: {result}");

    // An unknown port is an error, not a fallback to another instance.
    let (tx, rx) = mpsc::channel();
    worker
        .send(t32::WorkerRequest::RunCommand {
            command: "PRINT 1".to_string(),
            port: Some(1),
            core: None,
            device_prompt: "B::".to_string(),
            response_tx: tx,
        })
        .unwrap();
    let err = rx.recv().unwrap().expect_err("should fail");
    assert!(err.contains("No instance with port 1"), "got: {err}");
}
