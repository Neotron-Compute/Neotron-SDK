#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(not(target_os = "none"))]
fn main() {
    neotron_sdk::init();
}

use core::fmt::Write;

#[no_mangle]
extern "C" fn neotron_main() -> i32 {
    let mut stdout = neotron_sdk::stdout();
    let stdin = neotron_sdk::stdin();
    let _ = stdout.write(b"Type some things, press Ctrl-X to quit...\n");
    loop {
        let mut buffer = [0u8; 16];
        match stdin.read(&mut buffer) {
            Err(_) => {
                return 1;
            }
            Ok(0) => {
                // Do nothing
            }
            Ok(n) => {
                for b in &buffer[0..n] {
                    let _ = writeln!(stdout, "0x{:02x}", b);
                    if *b == 0x18 {
                        return 0;
                    }
                }
            }
        }
    }
}
