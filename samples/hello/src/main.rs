#![no_std]
#![no_main]

extern crate neotron_sdk;

#[no_mangle]
extern "C" fn neotron_main() -> i32 {
    let stdout = neotron_sdk::stdout();
    stdout.write(b"Hello, world\n").unwrap();
    0
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        // Spin
    }
}
