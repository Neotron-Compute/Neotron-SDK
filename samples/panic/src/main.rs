#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(not(target_os = "none"))]
fn main() {
    neotron_sdk::init();
}

use core::fmt::Write;

#[no_mangle]
extern "C" fn neotron_main() -> i32 {
    let stdout = neotron_sdk::stdout();
    writeln!(&stdout, "About to panic...\n").unwrap();
    panic!("Oh no, I panicked!");
}
