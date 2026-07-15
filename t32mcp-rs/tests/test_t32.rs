// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

use t32mcp::*;

use std::{env, fs::exists, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    time::sleep,
};

static T32_PORT: u16 = 20000;

#[test]
fn hello_world() {
    t32_connect(T32_PORT).unwrap();
    t32_cmd("PRINT \"Hello, world!\"".to_string()).unwrap();
}

#[tokio::test]
async fn pipe() {
    t32_connect(T32_PORT).unwrap();
    let connection = t32_open_pipe("t32chat").await.unwrap();

    let lines = ["Hello", "World"];
    for l in lines {
        t32_cmd(format!("PRINT \"{l}\"")).unwrap();
    }

    let mut receiver = BufReader::new(connection);
    let mut buf = String::with_capacity(256);

    for l in lines {
        buf.clear();
        let n = receiver
            .read_line(&mut buf)
            .await
            .expect("cannot receive string");
        assert!(n > 0);
        assert_eq!(l, buf.trim());
    }
}

#[ignore]
#[tokio::test]
async fn flood_pipe() {
    t32_connect(T32_PORT).unwrap();
    let connection = t32_open_pipe("t32chat").await.unwrap();

    let mut path = env::current_dir().expect("cannot get current directory");
    path.push("tests");
    path.push("flood_pipe.cmm");
    let path = path.as_path();
    assert!(exists(path).expect("cannot check whether file exists"));

    let reader_handle = tokio::spawn(async move {
        println!("start reading");
        let mut receiver = BufReader::with_capacity(256 * 1024, connection);
        let mut buf = vec![0u8; 64 * 1024];
        let mut lines = 0;
        loop {
            match receiver.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    let l = buf[0..n].iter().filter(|x| **x == b'\n').count();
                    lines += l;
                }
                Err(e) => {
                    println!("Read error: {e}");
                    unreachable!();
                }
            }
        }

        lines
    });

    sleep(Duration::from_millis(10)).await;

    println!("sending signal to PowerView");
    let path = path.to_string_lossy();
    t32_cmd(format!("DO \"{path}\"")).unwrap();

    let lines = reader_handle.await.expect("reader task panicked");
    println!("{lines} lines successfully read");

    // normally, this should be much more
    // on Windows: PowerView error "no memory for file pipe"
    assert!(lines > 10000);
}
