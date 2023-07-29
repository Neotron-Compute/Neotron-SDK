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
    writeln!(&stdout, "About to fault...\n").unwrap();
    let bad_address: usize = 0xDEAD_C0DE;
    let bad_fn: fn() = unsafe { core::mem::transmute(bad_address) };
    bad_fn();
    0
}
