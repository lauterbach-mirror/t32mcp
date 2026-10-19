// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

//! Rust port of the parts of the TRACE32 Remote API (RCL) that are needed
//! by this MCP server.
//!
//! Only the TCP transport (NETTCP configuration) is implemented.

use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

const RAPI_CMD_ATTACH: u8 = 0x71;
const RAPI_CMD_EXECUTE_PRACTICE: u8 = 0x72;
const RAPI_CMD_DEVICE_SPECIFIC: u8 = 0x74;

/// Device-specific subcommand: extended protocol (EXP)
const DS_SUBCMD_EXP: u8 = 0xFE;

/// EXP command: register read/write
const EXP_CMD_REGISTER_READ_WRITE: u16 = 3;

const PRACTICE_GETSTATE: u8 = 0x03;
const PRACTICE_EXECUTECOMMAND: u8 = 0x04;
const PRACTICE_EXECUTEFUNCTION: u8 = 0x05;

/// Register read mode: read all registers matching the given spec
const REGISTER_MODE_READALL: u16 = 0x00;
/// Number of register specs following the mode
const REGISTER_NREGS: u16 = 1;
/// Register unit: CPU
const REGISTER_UNIT_CPU: u16 = 0x04;

/// `T32_ExecuteFunction` result type: the function failed
pub const T32_RETURN_TYPE_ERROR: u32 = 0x0000;

const T32_ERR_OK: u8 = 0x00;

const T32_NETTCP_RCL_REQ: u32 = 0x0010;
const T32_NETTCP_RCL_RESP: u32 = 0x0011;
const T32_NETTCP_RCL_NOTIFY: u32 = 0x0012;

const T32_API_KEEPALIVE: u8 = 0xFE;

const HEADER_LEN: usize = 8;
const ALIGN64: usize = 8;

/// Maximum receive buffer size
const PACKLEN: usize = 0x4000;

/// Default response buffer size
const RESPONSE_BUFFER_SIZE: u32 = 4096;

pub const T32_DEV_ICD: u32 = 1;

fn align_up(n: usize, m: usize) -> usize {
    if n.is_multiple_of(m) { 0 } else { m - (n % m) }
}

/// A TCP connection to the TRACE32 Remote API of a PowerView instance.
pub struct Connection {
    socket: TcpStream,
    socket_content: Vec<u8>,
    response_queue: VecDeque<Vec<u8>>,
    /// Notification messages are not used by this client, but must be
    /// separated from responses to keep the stream in sync.
    notify_queue: VecDeque<Vec<u8>>,
    message_id: u32,
}

impl Connection {
    pub fn connect(node: &str, port: u16, timeout: Option<Duration>) -> Result<Self, String> {
        let addr = (node, port)
            .to_socket_addrs()
            .map_err(|e| format!("cannot resolve {node}:{port}: {e}"))?
            .next()
            .ok_or_else(|| format!("cannot resolve {node}:{port}"))?;

        let socket = match timeout {
            Some(t) => TcpStream::connect_timeout(&addr, t),
            None => TcpStream::connect(addr),
        }
        .map_err(|e| format!("cannot connect to {addr}: {e}"))?;

        socket
            .set_read_timeout(timeout)
            .map_err(|e| format!("cannot set read timeout: {e}"))?;
        socket
            .set_write_timeout(timeout)
            .map_err(|e| format!("cannot set write timeout: {e}"))?;
        let _ = socket.set_nodelay(true);

        Ok(Connection {
            socket,
            socket_content: Vec::new(),
            response_queue: VecDeque::new(),
            notify_queue: VecDeque::new(),
            message_id: 0,
        })
    }

    /// Adjust the read/write timeout of the underlying socket.
    pub fn set_timeout(&mut self, timeout: Option<Duration>) -> Result<(), String> {
        self.socket
            .set_read_timeout(timeout)
            .map_err(|e| format!("cannot set read timeout: {e}"))?;
        self.socket
            .set_write_timeout(timeout)
            .map_err(|e| format!("cannot set write timeout: {e}"))?;
        Ok(())
    }

    fn current_message_id(&self) -> u8 {
        (self.message_id % 255) as u8
    }

    fn increment_message_id(&mut self) -> u8 {
        self.message_id += 1;
        self.current_message_id()
    }

    fn transmit(&mut self, data: &[u8]) -> Result<(), String> {
        let size = data.len();
        let padding = align_up(size + HEADER_LEN, ALIGN64);

        let mut buf = Vec::with_capacity(HEADER_LEN + size + padding);
        buf.extend_from_slice(&(size as u32).to_le_bytes());
        buf.extend_from_slice(&T32_NETTCP_RCL_REQ.to_le_bytes());
        buf.extend_from_slice(data);
        buf.resize(HEADER_LEN + size + padding, 0);

        self.socket
            .write_all(&buf)
            .map_err(|e| format!("transmit failed: {e}"))
    }

    fn extract_messages(&mut self) -> Result<(), String> {
        loop {
            if self.socket_content.len() < HEADER_LEN {
                break;
            }
            let payload_len = u32::from_le_bytes(
                self.socket_content[0..4]
                    .try_into()
                    .expect("slice of length 4"),
            ) as usize;
            let message_len = payload_len + HEADER_LEN;
            if self.socket_content.len() < message_len {
                break;
            }
            let message_type = u32::from_le_bytes(
                self.socket_content[4..8]
                    .try_into()
                    .expect("slice of length 4"),
            );
            let payload = self.socket_content[HEADER_LEN..message_len].to_vec();

            // TRACE32 aligns messages to 64 bit.
            let stuffed = message_len + align_up(message_len, ALIGN64);
            let drain_to = stuffed.min(self.socket_content.len());
            self.socket_content.drain(..drain_to);

            match message_type {
                T32_NETTCP_RCL_NOTIFY => self.notify_queue.push_back(payload),
                T32_NETTCP_RCL_RESP => self.response_queue.push_back(payload),
                t => return Err(format!("TCP packet contains invalid message type {t:#x}")),
            }
        }
        Ok(())
    }

    fn poll_response(&mut self) -> Result<(), String> {
        let mut buf = [0u8; PACKLEN];
        let n = self
            .socket
            .read(&mut buf)
            .map_err(|e| format!("receive failed: {e}"))?;
        if n == 0 {
            return Err("connection closed by peer".to_string());
        }
        self.socket_content.extend_from_slice(&buf[..n]);
        self.extract_messages()
    }

    fn receive(&mut self) -> Result<Vec<u8>, String> {
        let message_id = self.current_message_id();
        loop {
            match self.response_queue.pop_front() {
                Some(message) => {
                    if message.len() < 2 {
                        continue;
                    }
                    if message[0] == T32_API_KEEPALIVE {
                        continue;
                    }
                    if message[1] < message_id {
                        continue;
                    }
                    if message[1] == message_id {
                        return Ok(message);
                    }
                    return Err("messages out of sync, connection broken".to_string());
                }
                None => self.poll_response()?,
            }
        }
    }

    fn generic_api_call(
        &mut self,
        rapi_cmd: u8,
        opt_arg: u8,
        payload: &[u8],
        sendonly: bool,
        force_length: Option<usize>,
    ) -> Result<Vec<u8>, String> {
        let payload_len = payload.len();
        let mut msg_len = force_length.unwrap_or(2 + payload_len);

        let mut send_data: Vec<u8> = Vec::with_capacity(6 + payload_len + 1);
        if msg_len > 0xFF {
            msg_len += 2;
            if msg_len > 0xF000 {
                return Err("message buffer too large".to_string());
            }
            let id = self.increment_message_id();
            send_data.extend_from_slice(&[0, rapi_cmd, opt_arg, id]);
            send_data.extend_from_slice(&(msg_len as u16).to_le_bytes());
        } else {
            let id = self.increment_message_id();
            send_data.extend_from_slice(&[msg_len as u8, rapi_cmd, opt_arg, id]);
        }
        send_data.extend_from_slice(payload);
        // pad payload to even length
        send_data.resize(send_data.len() + payload_len % 2, 0);

        self.transmit(&send_data)?;

        if sendonly {
            return Ok(Vec::new());
        }

        self.receive_checked()
    }

    /// Receive the response to the request with the current message id and
    /// check the TRACE32 error code. Returns the payload after the
    /// `[err_code, request_id]` header.
    fn receive_checked(&mut self) -> Result<Vec<u8>, String> {
        let recv = self.receive()?;
        if recv.is_empty() {
            return Err("empty response received".to_string());
        }
        if recv[0] != T32_ERR_OK {
            let mut msg = None;
            if recv.len() > 10 {
                let len =
                    u32::from_le_bytes(recv[6..10].try_into().expect("slice of length 4")) as usize;
                let end = (10 + len).min(recv.len());
                let text = String::from_utf8_lossy(&recv[10..end])
                    .trim_matches('\0')
                    .to_string();
                if !text.is_empty() {
                    msg = Some(text);
                }
            }
            return Err(match msg {
                Some(m) => format!("TRACE32 error {}: {m}", recv[0]),
                None => format!("TRACE32 error {}", recv[0]),
            });
        }

        Ok(recv[2..].to_vec())
    }

    /// `T32_Attach`
    pub fn attach(&mut self, device_specifier: u8) -> Result<(), String> {
        self.generic_api_call(RAPI_CMD_ATTACH, device_specifier, &[], false, None)?;
        Ok(())
    }

    /// `T32_ExecuteCommand`: execute a PRACTICE command,
    /// error messages are reported back by PowerView.
    pub fn execute_command(&mut self, command: &str) -> Result<(), String> {
        let mut payload = RESPONSE_BUFFER_SIZE.to_le_bytes().to_vec();
        payload.extend_from_slice(command.as_bytes());
        payload.push(0);
        self.generic_api_call(
            RAPI_CMD_EXECUTE_PRACTICE,
            PRACTICE_EXECUTECOMMAND,
            &payload,
            false,
            None,
        )?;
        Ok(())
    }

    /// `T32_GetPracticeState`: true while the PRACTICE interpreter runs
    /// (or waits in a dialog). A script that halted at an error or at STOP
    /// is not running, although it is still on the PRACTICE stack.
    pub fn practice_running(&mut self) -> Result<bool, String> {
        let result = self.generic_api_call(
            RAPI_CMD_EXECUTE_PRACTICE,
            PRACTICE_GETSTATE,
            &[],
            false,
            None,
        )?;
        Ok(result.iter().any(|&b| b != 0))
    }

    /// `T32_ExecuteFunction`: evaluate a PRACTICE function.
    ///
    /// Returns the result string and the TRACE32 result type.
    pub fn execute_function(&mut self, function: &str) -> Result<(String, u32), String> {
        let mut payload = RESPONSE_BUFFER_SIZE.to_le_bytes().to_vec();
        payload.extend_from_slice(function.as_bytes());
        payload.push(0);

        let result = self.generic_api_call(
            RAPI_CMD_EXECUTE_PRACTICE,
            PRACTICE_EXECUTEFUNCTION,
            &payload,
            false,
            None,
        )?;

        if result.len() < 8 {
            return Err("invalid function response".to_string());
        }
        let result_type = u32::from_le_bytes(result[0..4].try_into().expect("slice of length 4"));
        let result_size =
            u32::from_le_bytes(result[4..8].try_into().expect("slice of length 4")) as usize;
        let end = (8 + result_size).min(result.len());
        let value = String::from_utf8_lossy(&result[8..end])
            .trim_end_matches('\0')
            .to_string();

        Ok((value, result_type))
    }

    /// Evaluate a PRACTICE function that returns a string.
    ///
    /// Enforces the overall request deadline: the socket timeout is
    /// shortened to the remaining time, and an expired deadline is an error.
    pub fn fnc_string(
        &mut self,
        deadline: Instant,
        timeout: Duration,
        function: &str,
    ) -> Result<String, String> {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(format!(
                "query timed out after {} seconds",
                timeout.as_secs()
            ));
        }
        self.set_timeout(Some(remaining))?;
        let (value, result_type) = self.execute_function(function)?;
        if result_type == T32_RETURN_TYPE_ERROR {
            return Err(format!("PRACTICE function {function} failed: {value}"));
        }
        Ok(value)
    }

    /// Evaluate a PRACTICE function that returns a decimal number.
    /// Integer results arrive with a trailing dot ("2."), parse defensively.
    pub fn fnc_u32(
        &mut self,
        deadline: Instant,
        timeout: Duration,
        function: &str,
    ) -> Result<u32, String> {
        let value = self.fnc_string(deadline, timeout, function)?;
        value
            .trim()
            .trim_end_matches('.')
            .parse()
            .map_err(|_| format!("unexpected result of {function}: {value}"))
    }

    /// Evaluate a PRACTICE function that returns a boolean ("TRUE()"/"FALSE()").
    pub fn fnc_bool(
        &mut self,
        deadline: Instant,
        timeout: Duration,
        function: &str,
    ) -> Result<bool, String> {
        let value = self.fnc_string(deadline, timeout, function)?;
        match value
            .trim_matches(|c: char| c == '(' || c == ')')
            .to_lowercase()
            .as_str()
        {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(format!("unexpected result of {function}: {value}")),
        }
    }

    /// Extended protocol call (`T32Api.t32_exp` in the RCL).
    ///
    /// The request is sent in the forced extended format:
    /// `[0x00, RAPI_CMD_DEVICE_SPECIFIC, DS_SUBCMD_EXP, id, u16_le(len)] +
    /// (u16_le(cmd) + payload)` where the length field equals the transmitted
    /// payload length (`2 + payload.len()`). This length quirk is ported 1:1
    /// from the RCL; deviating from it breaks the protocol.
    ///
    /// Returns the EXP data (`response_len`, `response_cmd`, letter-coded
    /// blocks) on success.
    fn exp_call(&mut self, cmd: u16, payload: &[u8]) -> Result<Vec<u8>, String> {
        let mut exp_payload = Vec::with_capacity(2 + payload.len());
        exp_payload.extend_from_slice(&cmd.to_le_bytes());
        exp_payload.extend_from_slice(payload);

        let msg_len = exp_payload.len();
        if msg_len > 0xF000 {
            return Err("message buffer too large".to_string());
        }

        let mut send_data = Vec::with_capacity(6 + msg_len + 1);
        let id = self.increment_message_id();
        send_data.extend_from_slice(&[0x00, RAPI_CMD_DEVICE_SPECIFIC, DS_SUBCMD_EXP, id]);
        send_data.extend_from_slice(&(msg_len as u16).to_le_bytes());
        send_data.extend_from_slice(&exp_payload);
        // pad payload to even length
        send_data.resize(send_data.len() + exp_payload.len() % 2, 0);

        self.transmit(&send_data)?;
        self.receive_checked()
    }

    /// Read all CPU registers of a core via the extended protocol.
    pub fn read_registers_all(&mut self, core: u16) -> Result<Vec<Register>, String> {
        let mut payload = Vec::with_capacity(16);
        payload.extend_from_slice(&REGISTER_MODE_READALL.to_le_bytes());
        payload.extend_from_slice(&REGISTER_NREGS.to_le_bytes());
        payload.extend_from_slice(b"TY");
        payload.extend_from_slice(&REGISTER_UNIT_CPU.to_le_bytes());
        payload.extend_from_slice(b"CO");
        payload.extend_from_slice(&core.to_le_bytes());
        payload.extend_from_slice(b"XXXX");

        let exp = self.exp_call(EXP_CMD_REGISTER_READ_WRITE, &payload)?;
        let register_list = parse_exp_response(&exp)?;
        Ok(parse_register_list(&register_list))
    }
}

/// A register as returned by the TRACE32 extended protocol.
#[derive(Debug, Default)]
pub struct Register {
    pub name: Option<String>,
    pub unit: Option<u16>,
    pub value: Option<u64>,
    /// Multiple values (vector registers with a width)
    pub values: Vec<u64>,
    pub width: Option<u16>,
    pub core: Option<i16>,
}

impl Register {
    /// Deserializes data until it has found a complete register.
    ///
    /// Port of `Register._deserialize` of the RCL: returns the number of bytes
    /// consumed. When the position does not start a recognized block, the
    /// scan advances byte-wise (+1), exactly like the RCL.
    fn deserialize(&mut self, data: &[u8]) -> usize {
        let mut counter = 0;

        let tag = |pos: usize| -> Option<[u8; 2]> {
            let tag = data.get(pos..pos + 2)?;
            let tag: [u8; 2] = tag.try_into().expect("slice of length 2");
            Some(tag)
        };

        if tag(counter) == Some(*b"NM")
            && let Some(name_len) = data
                .get(counter + 2..counter + 4)
                .and_then(|s| s.try_into().ok())
                .map(u16::from_le_bytes)
        {
            // Name extraction: length and name bytes, trailing NULs stripped
            let end = (counter + 4 + name_len as usize).min(data.len());
            let name = String::from_utf8_lossy(&data[counter + 4..end])
                .trim_end_matches('\0')
                .to_string();
            self.name = Some(name);
            counter += 4 + name_len as usize;
        }

        if tag(counter) == Some(*b"TY") {
            if let Some(unit) = data
                .get(counter + 2..counter + 4)
                .and_then(|s| s.try_into().ok())
            {
                // Unit extraction
                self.unit = Some(u16::from_le_bytes(unit));
            }
            counter += 4;
        }

        if tag(counter) == Some(*b"VA") {
            if let Some(value) = data
                .get(counter + 2..counter + 10)
                .and_then(|s| s.try_into().ok())
            {
                // Value extraction
                self.value = Some(u64::from_le_bytes(value));
            }
            counter += 10;
        }

        self.width = None;
        if tag(counter) == Some(*b"WD") {
            if let Some(width) = data
                .get(counter + 2..counter + 4)
                .and_then(|s| s.try_into().ok())
            {
                // Width extraction
                self.width = Some(u16::from_le_bytes(width));
            }
            counter += 4;
        }

        if tag(counter) == Some(*b"FV") {
            // Floating point value extraction: consumed, but only kept for
            // FPU registers which are not requested by this client.
            counter += 10;
        }

        if tag(counter) == Some(*b"CO") {
            if let Some(core) = data
                .get(counter + 2..counter + 4)
                .and_then(|s| s.try_into().ok())
            {
                // Core extraction
                self.core = Some(i16::from_le_bytes(core));
            }
            counter += 4;
        }

        if tag(counter) == Some(*b"VB")
            && let Some(value_size) = data
                .get(counter + 2..counter + 4)
                .and_then(|s| s.try_into().ok())
        {
            // Value extraction (byte buffer)
            let value_size = u16::from_le_bytes(value_size) as usize;
            counter += 4;
            match self.width {
                None => {
                    let end = (counter + value_size).min(data.len());
                    let mut buf = [0u8; 8];
                    let value = &data[counter..end];
                    buf[..value.len().min(8)].copy_from_slice(&value[..value.len().min(8)]);
                    self.value = Some(u64::from_le_bytes(buf));
                    counter += value_size;
                }
                Some(width) if width > 0 => {
                    let width = width as usize;
                    let mut shift = 0;
                    self.values = Vec::new();
                    while shift + width <= value_size {
                        let Some(chunk) = data.get(counter + shift..counter + shift + width) else {
                            break;
                        };
                        let mut buf = [0u8; 8];
                        buf[..width.min(8)].copy_from_slice(&chunk[..width.min(8)]);
                        self.values.push(u64::from_le_bytes(buf));
                        shift += width;
                    }
                    counter += shift;
                }
                Some(_) => {}
            }
        }

        if tag(counter) == Some(*b"XX") {
            return counter;
        }
        counter + 1
    }
}

/// Parses a register list as returned by the EXP register read command.
///
/// Port of `RegisterService.__deserialize_registers` of the RCL: registers are
/// letter-coded blocks terminated by `XXXX`; when a position does not start a
/// recognized block, the scan advances byte-wise (+1).
pub fn parse_register_list(data: &[u8]) -> Vec<Register> {
    let mut counter = 0;
    let mut registers = Vec::new();
    while counter < data.len() {
        let mut reg = Register::default();
        counter += reg.deserialize(&data[counter..]);
        let is_terminator = data.get(counter..counter + 2).is_some_and(|t| t == b"XX");
        if is_terminator && reg.name.is_some() {
            counter += 4;
            registers.push(reg);
        } else {
            counter += 1;
        }
    }
    registers
}

/// Parses an EXP response and returns the `PL` (payload) buffer.
///
/// The EXP data starts with `u16_le(response_len) | u16_le(response_cmd)`,
/// followed by letter-coded blocks: `EC` (error code, mandatory), `EM`
/// (error message, optional) and `PL` (payload, optional). The block decoder
/// stops at `00 00` padding. A missing `PL` block signals an EXP-level error.
fn parse_exp_response(exp: &[u8]) -> Result<Vec<u8>, String> {
    if exp.len() < 4 {
        return Err("invalid EXP response".to_string());
    }

    let mut err_code = None;
    let mut err_msg = None;
    let mut payload = None;

    let mut pos = 4;
    while let Some(tag) = exp.get(pos..pos + 2) {
        if tag == [0x00, 0x00] {
            break;
        }
        pos += 2;
        let Some(size) = exp
            .get(pos..pos + 2)
            .and_then(|s| s.try_into().ok())
            .map(u16::from_le_bytes)
        else {
            break;
        };
        pos += 2;
        let end = (pos + size as usize).min(exp.len());
        let content = &exp[pos..end];
        match tag {
            b"EC" => {
                err_code = Some(
                    content
                        .iter()
                        .rev()
                        .fold(0u64, |acc, &b| (acc << 8) | b as u64),
                );
            }
            b"EM" => {
                err_msg = Some(
                    String::from_utf8_lossy(content)
                        .trim_end_matches('\0')
                        .to_string(),
                );
            }
            b"PL" => {
                payload = Some(content.to_vec());
            }
            _ => break,
        }
        pos = end;
    }

    match payload {
        Some(p) => Ok(p),
        None => Err(match (err_msg, err_code) {
            (Some(msg), Some(code)) => format!("{msg} (code {code})"),
            (Some(msg), None) => msg,
            (None, Some(code)) => format!("extended protocol call failed (code {code})"),
            (None, None) => "extended protocol call failed".to_string(),
        }),
    }
}
