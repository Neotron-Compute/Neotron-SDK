#![no_std]
#![no_main]

use core::fmt::Write;

#[no_mangle]
extern "C" fn neotron_main() -> i32 {
    let stdout = neotron_sdk::stdout();
    writeln!(&stdout, "About to fault...\n").unwrap();
    let bad_address: u32 = 0xDEAD_C0DE;
    let bad_fn: fn() = unsafe { core::mem::transmute(bad_address) };
    bad_fn();
    0
}
