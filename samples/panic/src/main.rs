#![no_std]
#![no_main]

use core::fmt::Write;

#[no_mangle]
extern "C" fn neotron_main() -> i32 {
    let stdout = neotron_sdk::stdout();
    writeln!(&stdout, "About to panic...\n").unwrap();
    panic!("Oh no, I panicked!");
}
