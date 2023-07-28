#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(not(target_os = "none"))]
fn main() {
    neotron_sdk::init();
}

static mut APP: snake::App = snake::App::new(80, 25);

#[no_mangle]
extern "C" fn neotron_main() -> i32 {
    if let Err(e) = unsafe { APP.play() } {
        let mut stdout = neotron_sdk::stdout();
        use core::fmt::Write;
        let _ = writeln!(stdout, "Error: {:?}", e);
        1
    } else {
        0
    }
}
