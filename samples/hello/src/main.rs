#![no_std]
#![no_main]

extern crate neotron_sdk;

#[no_mangle]
extern "C" fn main() -> i32 {
    let stdout = neotron_sdk::FileHandle::new_stdout();
    neotron_sdk::write(stdout, b"Hello, world\n");
    0
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        // Spin
    }
}
