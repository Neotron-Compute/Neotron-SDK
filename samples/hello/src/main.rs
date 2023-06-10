#![no_std]
#![no_main]

extern crate neotron_sdk;

#[no_mangle]
extern "C" fn neotron_main() -> i32 {
    let stdout = neotron_sdk::stdout();
    if stdout.write(b"Hello, world\n").is_ok() {
        0
    } else {
        1
    }
}
