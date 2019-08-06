/*
 * Note: The 'bindgen' crate is not used because it causes conflicts with
 * 'no_std' environments. See: https://github.com/rust-lang/cargo/issues/5730
 * you'll need to install bindgen system-wide until that is fixed:
 *
 * `cargo install bindgen`
 */

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Link the FreeRTOS static library.
    let pwd = &PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    println!(
        "cargo:rustc-link-search=native={}/freertos_gen",
        pwd.display()
    );
    println!("cargo:rustc-link-lib=static=freertos_interop");

    // Run bindgen to create Rust bindings for the C library.
    let fr_rs_path = pwd.join("src/freertos.rs");
    let mut bindings_rs = File::create(fr_rs_path.to_str().unwrap())
        .expect("Could not create FreeRTOS bindings file.");
    bindings_rs
        .write_all(
            std::str::from_utf8(
                &Command::new("bindgen")
                    .arg("--ctypes-prefix=cty")
                    .arg("--use-core")
                    .arg("--no-layout-tests")
                    .arg("--rust-target=nightly")
                    .arg("--blacklist-item=SystemCoreClock")
                    .arg("bindings.h")
                    .output()
                    .unwrap()
                    .stdout,
            )
            .unwrap()
            .as_bytes(),
        )
        .unwrap();
    // Format the auto-generated FFI.
    Command::new("rustfmt")
        .arg(fr_rs_path.to_str().unwrap())
        .status()
        .expect("Failed to format auto-generated bindings.");

    // Make sure the linker script is generated.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    macro_rules! ld_mem {
        () => {
            "ld/stm32l475xg.x"
        };
    };
    File::create(out_path.join("memory.x"))
        .expect("Failed to open memory sections file")
        .write_all(include_bytes!(ld_mem!()))
        .expect("Failed to write memory sections file");
    println!("cargo:rustc-link-search={}", out_path.display());
}
