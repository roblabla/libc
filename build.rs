// TODO: Keep this here ?
use std::env;
use std::process::Command;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS must be set") == "horizon" {
        let dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set") + "/../../libtransistor-rs/libtransistor-sys";
        // Build and get libc.
        let status = Command::new("make")
            .current_dir(&format!("{}/libtransistor", dir))
            .status().expect("Make failed");
        if !status.success() {
            panic!("Make failed");
        }
        println!("cargo:rustc-link-search=native={}/libtransistor/newlib/aarch64-none-switch/newlib", dir);
    }
}
