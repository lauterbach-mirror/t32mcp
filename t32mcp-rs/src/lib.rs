// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! Communicating with TRACE32 from Rust. Built on top of Remote API.
//!
//! Pipe communication:
//! - Windows: Named Pipe
//! - Linux: FIFO

use anyhow::Result;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;

#[allow(warnings)]
pub mod t32 {
    include!(concat!(env!("OUT_DIR"), "/t32.rs"));
}

#[cfg(target_os = "windows")]
use interprocess::os::windows::named_pipe::{PipeListenerOptions, pipe_mode, tokio::PipeStream};

#[cfg(not(target_os = "windows"))]
use {interprocess::os::unix::fifo_file, tokio::net::unix::pipe, tokio::net::unix::pipe::Receiver};

#[cfg(target_os = "windows")]
pub async fn t32_open_pipe(
    name: &str,
) -> Result<PipeStream<pipe_mode::Bytes, pipe_mode::None>, String> {
    let path = format!(r"\\.\pipe\{name}");

    let listener = PipeListenerOptions::new()
        .path(path)
        .create_tokio_recv_only::<pipe_mode::Bytes>()
        .expect("cannot create pipe");

    let area = t32_fnc(r"AREA.SELECTed()".to_string())?;
    let name = format!("\\\\.\\pipe\\{name}");
    t32_cmd(format!(r"AREA.PIPE {} {}", area, name))?;

    let connection = match listener.accept().await {
        Ok(c) => c,
        Err(e) => {
            return Err(format!("Cannot setup named pipe: {e}"));
        }
    };

    Ok(connection)
}

#[cfg(not(target_os = "windows"))]
pub async fn t32_open_pipe(name: &str) -> Result<Receiver, String> {
    let fifo_name = format!("/tmp/{name}");
    // Remove a stale FIFO left over from a previous run before re-creating it.
    let _ = std::fs::remove_file(&fifo_name);
    if let Err(e) = fifo_file::create_fifo(&fifo_name, 0o777) {
        return Err(format!("FIFO cannot be created: {e}"));
    }

    let area = t32_fnc(r"AREA.SELECTed()".to_string())?;
    t32_cmd(format!(r"AREA.PIPE {} {}", area, &fifo_name))?;

    let connection = match pipe::OpenOptions::new().open_receiver(&fifo_name) {
        Ok(c) => c,
        Err(e) => {
            let _ = std::fs::remove_file(&fifo_name);
            return Err(format!("Cannot connect to FIFO: {e}"));
        }
    };

    // The receiver holds an open fd; unlink the path so it is cleaned up on close.
    let _ = std::fs::remove_file(&fifo_name);

    Ok(connection)
}

pub fn t32_connect(port: u32) -> Result<(), String> {
    let mut channel: *mut c_void = ptr::null_mut();

    let e = unsafe { t32::T32_RequestChannelNetTcp(&mut channel as *mut *mut c_void) };
    if e != 0 {
        return Err(format!(
            "Connection with TRACE32 failed: T32_RequestChannelNetTcp returned {e}"
        ));
    }

    unsafe {
        t32::T32_SetChannel(channel);
    }

    let port_config = CString::new("PORT=").expect("CString::new failed");
    let port_value = CString::new(port.to_string()).expect("CString::new failed");
    let e = unsafe { t32::T32_Config(port_config.as_ptr(), port_value.as_ptr()) };
    if e != 0 {
        return Err(format!(
            "Connection with TRACE32 failed: T32_Config returned {e}"
        ));
    }

    let e = unsafe { t32::T32_Init() };
    if e != 0 {
        return Err(format!(
            "Connection with TRACE32 failed: T32_Init returned {e}"
        ));
    }

    let e = unsafe { t32::T32_Attach(t32::T32_DEV_ICD as i32) };
    if e != 0 {
        return Err(format!(
            "Connection with TRACE32 failed: T32_Attach returned {e}"
        ));
    }

    let area = t32_fnc(r"AREA.SELECTed()".to_string())?;
    t32_cmd(format!(r"AREA.OPEN {area}"))?;

    Ok(())
}

pub fn t32_cmd(cmd: String) -> Result<(), String> {
    let c = CString::new(cmd).expect("CString::new failed");
    let e = unsafe { t32::T32_Cmd(c.as_ptr()) };
    if e != 0 {
        return Err(format!("TRACE32 command failed: T32_Cmd returned {e}"));
    }

    Ok(())
}

pub fn t32_fnc(fnc: String) -> Result<String, String> {
    let fnc = CString::new(fnc).expect("CString::new failed");

    let buffer_size: u32 = 1024;
    let mut buffer: Vec<u8> = vec![0u8; buffer_size as usize];
    let mut result_type: u32 = 0;

    let e = unsafe {
        t32::T32_ExecuteFunction(
            fnc.as_ptr(),
            buffer.as_mut_ptr() as *mut i8,
            buffer_size,
            &mut result_type,
        )
    };
    if e != 0 {
        return Err(format!(
            "TRACE32 function failed: T32_ExecuteFunction returned {e}"
        ));
    }

    let result = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) }
        .to_string_lossy()
        .into_owned();

    Ok(result)
}
