// SPDX-FileCopyrightText: 2026 Lauterbach GmbH
// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-lib=t32api64");

    let bindings = bindgen::Builder::default()
        .header("lib/t32.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("t32.rs"))
        .expect("Couldn't write bindings!");
}
