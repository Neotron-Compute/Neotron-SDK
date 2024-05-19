//! Build script for the Neotron SDK
//!
//! Sets up Rust to link with a Cortex-M linker script if you are building for
//! an Arm bare-metal target.

use std::io::prelude::*;

fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH variable");
    let os = std::env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS variable");
    match (arch.as_str(), os.as_str()) {
        ("arm", "none") => {
            setup_cortexm_linker();
        }
        _ => {
            // no script required
        }
    }
}

fn setup_cortexm_linker() {
    // Put `neotron-cortex-m.ld` in our output directory and ensure it's
    // on the linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join("neotron-cortex-m.ld"))
        .unwrap()
        .write_all(include_bytes!("./neotron-cortex-m.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `neotron-cortex-m.ld`
    // here, we ensure the build script is only re-run when
    // `neotron-cortex-m.ld` is changed.
    println!("cargo:rerun-if-changed=./neotron-cortex-m.ld");
}
