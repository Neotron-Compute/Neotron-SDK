//! The Neotron SDK
//!
//! Defines the API supplied to applications that run on Neotron OS
//!
//! You should use this crate when writing applications that run on Neotron OS.
//!
//! This SDK attempts to detect targets that support UNIX or Windows, and
//! implements some code to talk to the appropriate UNIX or Windows API. This
//! allows some level of portable, mainly to support application testing on
//! those OSes.
//!
//! On a *bare-metal* target (i.e. where the OS is `none`), the SDK expects the
//! Neotron OS to pass the callback table to the entry point
//! ([`app_entry()`](app_entry)). Once initialised, the SDK then expects you
//! application to provide an `extern "C"` `no-mangle` function called
//! `neotron_main`, which the SDK will call.

#![cfg_attr(target_os = "none", no_std)]

// ============================================================================
// Imports
// ============================================================================

use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

pub use neotron_ffi::{FfiBuffer, FfiByteSlice, FfiString};

pub use neotron_api::{file::Flags, path, Api, Error};

use neotron_api as api;

pub mod console;

#[cfg(not(target_os = "none"))]
mod fake_os_api;

// ============================================================================
// Constants
// ============================================================================

/// Maximum length of a path to a file.
///
/// This is shorter than on MS-DOS, to save on memory.
pub const MAX_PATH_LEN: usize = 64;

extern "C" {
    /// This is what the user's application entry point must be called.
    ///
    /// They can return `0` for success, or anything else to indicate an error.
    fn neotron_main() -> i32;
}

// ============================================================================
// Static Variables
// ============================================================================

/// Holds a pointer to the OS API provided by the OS on start-up.
///
/// Once you've hit the application `main()`, this will be non-null.
static API: AtomicPtr<Api> = AtomicPtr::new(core::ptr::null_mut());

/// Number of arguments passed
static ARG_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Start of the argument list
static ARG_PTR: AtomicPtr<FfiString> = AtomicPtr::new(core::ptr::null_mut());

/// Random number generator state
static RAND_STATE: core::sync::atomic::AtomicU16 = core::sync::atomic::AtomicU16::new(0);

// ============================================================================
// Types
// ============================================================================

/// The type of the application entry-point.
///
/// The OS calls a function of this type.
pub use neotron_api::AppStartFn;

/// The result type for any SDK function.
///
/// Like a [`core::result::Result`] but the error type is [`Error`].
pub type Result<T> = core::result::Result<T, Error>;

/// Represents an open File
pub struct File(api::file::Handle);

impl File {
    /// Open a file, given a path as UTF-8 string.
    ///
    /// If the file does not exist, or is already open, it returns an error.
    ///
    /// Path may be relative to current directory, or it may be an absolute
    /// path.
    ///
    /// # Limitations
    ///
    /// * You cannot open a file if it is currently open.
    /// * Paths must confirm to the rules for the filesystem for the given drive.
    /// * Relative paths are taken relative to the current directory (see `Api::chdir`).
    pub fn open(path: path::Path, flags: api::file::Flags) -> Result<Self> {
        let api = get_api();
        match (api.open)(FfiString::new(path.as_str()), flags) {
            neotron_ffi::FfiResult::Ok(handle) => Ok(File(handle)),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }

    /// Write to an open file handle, blocking until everything is written.
    ///
    /// Some files do not support writing and will produce an error. You will
    /// also get an error if you run out of disk space.
    ///
    /// The `buffer` is only borrowed for the duration of the function call and
    /// is then forgotten.
    pub fn write(&self, buffer: &[u8]) -> Result<()> {
        let api = get_api();
        match (api.write)(self.0, FfiByteSlice::new(buffer)) {
            neotron_ffi::FfiResult::Ok(_) => Ok(()),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }

    /// Read from an open file, returning how much was actually read.
    ///
    /// You might get less data than you asked for. If you do an `Api::read` and
    /// you are already at the end of the file you will get
    /// `Err(Error::EndOfFile)`.
    ///
    /// Data is stored to the given `buffer. The `buffer` is only borrowed for
    /// the duration of the function call and is then forgotten.
    pub fn read(&self, buffer: &mut [u8]) -> Result<usize> {
        let api = get_api();
        match (api.read)(self.0, FfiBuffer::new(buffer)) {
            neotron_ffi::FfiResult::Ok(n) => Ok(n),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }

    /// Close a file
    pub fn close(self) -> Result<()> {
        let api = get_api();
        // We could panic on error, but let's silently ignore it for now
        let result = (api.close)(self.0);
        core::mem::forget(self);
        result.into()
    }

    /// Move the file offset (for the given file handle) to the given position.
    ///
    /// Some files do not support seeking and will produce an error.
    pub fn seek_set(&self, position: u64) -> Result<()> {
        let api = get_api();
        match (api.seek_set)(self.0, position) {
            neotron_ffi::FfiResult::Ok(_) => Ok(()),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }

    /// Move the file offset (for the given file handle) relative to the current position.
    ///
    /// Returns the new position, or an error.
    ///
    /// Some files do not support seeking and will produce an error.
    pub fn seek_cur(&self, offset: i64) -> Result<u64> {
        let api = get_api();
        match (api.seek_cur)(self.0, offset) {
            neotron_ffi::FfiResult::Ok(new_offset) => Ok(new_offset),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }

    /// Move the file offset (for the given file handle) to the end of the file
    ///
    /// Returns the new position, or an error.
    ///
    /// Some files do not support seeking and will produce an error.
    pub fn seek_end(&self) -> Result<u64> {
        let api = get_api();
        match (api.seek_end)(self.0) {
            neotron_ffi::FfiResult::Ok(new_offset) => Ok(new_offset),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }

    /// Rename a file.
    ///
    /// # Limitations
    ///
    /// * You cannot rename a file if it is currently open.
    /// * You cannot rename a file where the `old_path` and the `new_path` are
    /// not on the same drive.
    /// * Paths must confirm to the rules for the filesystem for the given drive.
    pub fn rename(old_path: path::Path, new_path: path::Path) -> Result<()> {
        let api = get_api();
        match (api.rename)(
            FfiString::new(old_path.as_str()),
            FfiString::new(new_path.as_str()),
        ) {
            neotron_ffi::FfiResult::Ok(_) => Ok(()),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }

    /// Perform a special I/O control operation.
    ///
    /// The allowed values of `command` and `value` are TBD.
    pub fn ioctl(&self, command: u64, value: u64) -> Result<u64> {
        let api = get_api();
        match (api.ioctl)(self.0, command, value) {
            neotron_ffi::FfiResult::Ok(output) => Ok(output),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }

    /// Get information about this file.
    pub fn stat(&self) -> Result<api::file::Stat> {
        let api = get_api();
        match (api.fstat)(self.0) {
            neotron_ffi::FfiResult::Ok(output) => Ok(output),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let api = get_api();
        // Don't close default (in, out, err) handles on drop because we can't
        // re-open them.
        if self.0.value() <= 2 {
            // don't close
        } else {
            // close it
            let _ = (api.close)(self.0);
        }
    }
}

impl core::fmt::Write for File {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes()).map_err(|_| core::fmt::Error)
    }
}

impl core::fmt::Write for &File {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes()).map_err(|_| core::fmt::Error)
    }
}

/// Represents an open directory that we are iterating through.
pub struct ReadDir(api::dir::Handle);

impl ReadDir {
    pub fn open(path: path::Path) -> Result<ReadDir> {
        let api = get_api();
        match (api.opendir)(FfiString::new(path.as_str())) {
            neotron_ffi::FfiResult::Ok(output) => Ok(ReadDir(output)),
            neotron_ffi::FfiResult::Err(e) => Err(e),
        }
    }
}

impl Iterator for ReadDir {
    type Item = Result<api::dir::Entry>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Drop for ReadDir {
    fn drop(&mut self) {
        let api = get_api();
        let _ = (api.closedir)(self.0);
    }
}

/// The result of a *Wait for Key* operation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WaitForKey {
    /// User wants more data
    More,
    /// User wants to quit
    Quit,
}

// ============================================================================
// Functions
// ============================================================================

/// The function the OS calls to start the application.
///
/// Will initialise the SDK and then jump to the application entry point, which
/// is an `extern "C"` function called `neotron_main`.
#[no_mangle]
pub extern "C" fn app_entry(api: *const Api, argc: usize, argv: *const FfiString) -> i32 {
    let _check: AppStartFn = app_entry;
    API.store(api as *mut Api, Ordering::Relaxed);
    ARG_COUNT.store(argc, Ordering::Relaxed);
    ARG_PTR.store(argv as *mut FfiString, Ordering::Relaxed);
    unsafe { neotron_main() }
}

/// Get a command line argument.
///
/// Given an zero-based index, returns `Some(str)` if that argument was
/// provided, otherwise None.
///
/// Does not return the name of the program in the first argument.
#[cfg(target_os = "none")]
pub fn arg(n: usize) -> Option<&'static str> {
    let arg_count = ARG_COUNT.load(Ordering::Relaxed);
    let arg_ptr = ARG_PTR.load(Ordering::Relaxed);
    let arg_slice = unsafe { core::slice::from_raw_parts(arg_ptr, arg_count) };
    arg_slice.get(n).map(|ffi| ffi.as_str())
}

/// Get a command line argument.
///
/// Given an zero-based index, returns `Some(str)` if that argument was
/// provided, otherwise None.
///
/// Does not return the name of the program in the first argument.
#[cfg(not(target_os = "none"))]
pub fn arg(n: usize) -> Option<String> {
    std::env::args().skip(1).nth(n)
}

/// Get information about a file on disk.
///
/// **Note:** This function is not implemented currently.
pub fn stat(_path: path::Path) -> Result<api::file::Stat> {
    todo!()
}

/// Delete a file from disk
///
/// **Note:** This function is not implemented currently.
pub fn delete(_path: path::Path) -> Result<()> {
    todo!()
}

/// Change the current working directory to the given path.
///
/// **Note:** This function is not implemented currently.
pub fn chdir(_path: path::Path) -> Result<()> {
    todo!()
}

/// Change the current working directory to that given by the handle.
///
/// **Note:** This function is not implemented currently.
pub fn dchdir(_dir: api::dir::Handle) -> Result<()> {
    todo!()
}

/// Get the current working directory.
///
/// Provided as a call-back, so the caller doesn't need to allocate storage space for the string.
pub fn pwd<F: FnOnce(Result<path::Path>)>(callback: F) {
    callback(Err(Error::NotFound))
}

/// Alllocate some memory
///
/// **Note:** This function is not implemented currently.
pub fn malloc(_size: usize, _alignment: usize) -> Result<*mut core::ffi::c_void> {
    todo!()
}

/// Free some previously allocated memory.
///
/// **Note:** This function is not implemented currently.
pub fn free(_ptr: *mut core::ffi::c_void, _size: usize, _alignment: usize) {
    todo!()
}

/// Get a handle for Standard Input
pub const fn stdin() -> File {
    File(api::file::Handle::new_stdin())
}

/// Get a handle for Standard Output
pub const fn stdout() -> File {
    File(api::file::Handle::new_stdout())
}

/// Get a handle for Standard Error
pub const fn stderr() -> File {
    File(api::file::Handle::new_stderr())
}

/// Delay for some given duration before returning.
///
/// Currently this does a badly calibrated nop busy-wait.
#[cfg(target_os = "none")]
pub fn delay(period: core::time::Duration) {
    // TODO: call OS sleep API?
    for _ in 0..period.as_micros() {
        for _ in 0..50 {
            unsafe { core::arch::asm!("nop") }
        }
    }
}

/// Delay for some given duration before returning.
#[cfg(not(target_os = "none"))]
pub fn delay(period: core::time::Duration) {
    std::thread::sleep(period);
}

/// Wait for a key
///
/// Prints `Press Space for more, 'q' to quit...` with a spinner, and waits
/// for you to press the appropriate key.
pub fn wait_for_key() -> WaitForKey {
    use core::fmt::Write;
    let mut ticker = "|/-\\".chars().cycle();
    let stdin = stdin();
    let mut stdout = stdout();
    let result = loop {
        let _ = write!(
            stdout,
            "\rPress Space for more, 'q' to quit... {}",
            ticker.next().unwrap()
        );
        let mut buffer = [0u8; 1];
        match stdin.read(&mut buffer) {
            Ok(0) => {
                // No data
            }
            Ok(_n) => {
                if buffer[0] == b' ' {
                    break WaitForKey::More;
                } else if buffer[0] == b'q' || buffer[0] == b'Q' {
                    break WaitForKey::Quit;
                }
            }
            Err(e) => {
                let _ = writeln!(stdout, "Error {:?}", e);
                break WaitForKey::Quit;
            }
        }
    };
    let _ = write!(stdout, "\r                                         \r");
    result
}

/// Seed the 16-bit psuedorandom number generator
pub fn srand(seed: u16) {
    RAND_STATE.store(seed, core::sync::atomic::Ordering::Relaxed);
}

/// Get a 16-bit psuedorandom number
pub fn rand() -> u16 {
    let mut state = RAND_STATE.load(core::sync::atomic::Ordering::Relaxed);
    let bit = (state ^ (state >> 2) ^ (state >> 3) ^ (state >> 5)) & 0x01;
    state = (state >> 1) | (bit << 15);
    RAND_STATE.store(state, core::sync::atomic::Ordering::Relaxed);
    state
}

/// Get the API structure so we can call APIs manually.
///
/// If you managed to not have `app_entry` called on start-up, this will panic.
fn get_api() -> &'static Api {
    let ptr = API.load(Ordering::Relaxed);
    unsafe { ptr.as_ref().unwrap() }
}

/// Initialisation function for OSes other than Neotron OS
///
/// If you are using this SDK on Windows or UNIX, your main function should
/// call this function as the first thing it does. It will set up the SDK
/// and then jump to `neotron_main()`.
///
/// ```no_run
/// #[cfg(not(target_os = "none"))]
/// fn main() {
///     neotron_sdk::init();
/// }
///
///
/// #[no_mangle]
/// extern "C" fn neotron_main() -> i32 {
///     // Your code here
///     0
/// }
/// ```
#[cfg(not(target_os = "none"))]
pub fn init() {
    API.store(fake_os_api::get_ptr() as *mut Api, Ordering::Relaxed);
    crossterm::terminal::enable_raw_mode().expect("enable raw mode");
    let res = unsafe { neotron_main() };
    crossterm::terminal::disable_raw_mode().expect("disable raw mode");
    std::process::exit(res);
}

#[cfg(all(target_os = "none", feature = "fancy-panic"))]
#[inline(never)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    let stdout = stdout();
    let _ = writeln!(&stdout, "Panic:\n{:#?}", info);
    loop {}
}

#[cfg(all(target_os = "none", not(feature = "fancy-panic")))]
#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    let stdout = stdout();
    let _ = writeln!(&stdout, "Panic!");
    loop {}
}

// ============================================================================
// End of File
// ============================================================================
