#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

use core::fmt::Write;

#[cfg(not(target_os = "none"))]
fn main() {
    neotron_sdk::init();
}

#[no_mangle]
extern "C" fn neotron_main() -> i32 {
    if let Err(e) = real_main() {
        let mut stdout = neotron_sdk::stdout();
        let _ = writeln!(stdout, "Error: {:?}", e);
        1
    } else {
        0
    }
}

fn real_main() -> Result<(), neotron_sdk::Error> {
    let mut stdout = neotron_sdk::stdout();
    let Some(filename) = neotron_sdk::arg(0) else {
        return Err(neotron_sdk::Error::InvalidArg);
    };
    let _ = writeln!(stdout, "Dumping {:?}...", filename);
    let path = neotron_sdk::path::Path::new(&filename)?;
    let f = neotron_sdk::File::open(path, neotron_sdk::Flags::empty())?;
    let stat = f.stat()?;
    let mut bytes_remaining = stat.file_size;
    let _ = writeln!(stdout, "File is {} bytes", bytes_remaining);

    let mut lines_remaining = 24;
    let mut buffer = [0u8; 16];
    let mut addr = 0;
    while bytes_remaining > 0 {
        let this_time = f.read(&mut buffer)?;
        let valid = &buffer[0..this_time];
        // print address
        let _ = write!(stdout, "{:08x}: ", addr);
        // print bytes (with padding)
        for b in valid {
            let _ = write!(stdout, "{:02x} ", b);
        }
        for _padding in 0..(buffer.len() - valid.len()) {
            let _ = write!(stdout, ".. ");
        }
        let _ = write!(stdout, "| ");
        // print ascii (with padding)
        for b in valid {
            let ch = *b as char;
            let _ = write!(stdout, "{}", if !ch.is_control() { ch } else { '?' });
        }
        for _padding in 0..(buffer.len() - valid.len()) {
            let _ = write!(stdout, ".");
        }
        let _ = writeln!(stdout, "|");
        addr += this_time;
        bytes_remaining = bytes_remaining.saturating_sub(this_time as u64);
        if lines_remaining == 0 {
            if neotron_sdk::wait_for_key() == neotron_sdk::WaitForKey::Quit {
                break;
            }
            lines_remaining = 25;
        } else {
            lines_remaining -= 1;
        }
    }

    Ok(())
}
