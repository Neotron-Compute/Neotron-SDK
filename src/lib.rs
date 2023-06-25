//! The Neotron SDK
//!
//! Defines the API supplied to applications that run on Neotron OS

#![no_std]

// ============================================================================
// Imports
// ============================================================================

use core::sync::atomic::{AtomicPtr, Ordering};

pub use neotron_ffi::{FfiBuffer, FfiByteSlice, FfiString};

pub use neotron_api::{path, Api, Error};

use neotron_api as api;

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

// ============================================================================
// Types
// ============================================================================

/// The type of the application entry-point.
///
/// The OS calls a function of this type.
pub type AppStartFn = extern "C" fn(*mut crate::Api) -> i32;

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

impl core::ops::Drop for File {
    fn drop(&mut self) {
        let api = get_api();
        // We could panic on error, but let's silently ignore it for now.
        // If you care, call `file.close()`.
        let _ = (api.close)(self.0);
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

// ============================================================================
// Functions
// ============================================================================

/// The function the OS calls to start the application.
///
/// Will jump to the application entry point, and `extern "C"` function
/// called `main`.
#[no_mangle]
extern "C" fn app_entry(api: *mut Api) -> i32 {
    API.store(api, Ordering::Relaxed);
    unsafe { neotron_main() }
}

/// Get information about a file on disk.
pub fn stat(_path: path::Path) -> Result<api::file::Stat> {
    todo!()
}

/// Delete a file from disk
pub fn delete(_path: path::Path) -> Result<()> {
    todo!()
}

/// Change the current working directory to the given path.
pub fn chdir(_path: path::Path) -> Result<()> {
    todo!()
}

/// Change the current working directory to that given by the handle.
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
pub fn malloc(_size: usize, _alignment: usize) -> Result<*mut core::ffi::c_void> {
    todo!()
}

/// Free some previously allocated memory.
pub fn free(_ptr: *mut core::ffi::c_void, _size: usize, _alignment: usize) {
    todo!()
}

/// Get a handle for Standard Input
pub fn stdin() -> File {
    File(api::file::Handle::new_stdin())
}

/// Get a handle for Standard Output
pub fn stdout() -> File {
    File(api::file::Handle::new_stdout())
}

/// Get a handle for Standard Error
pub fn stderr() -> File {
    File(api::file::Handle::new_stderr())
}

/// Get the API structure so we can call APIs manually.
///
/// If you managed to not have `app_entry` called on start-up, this will panic.
fn get_api() -> &'static Api {
    let ptr = API.load(Ordering::Relaxed);
    unsafe { ptr.as_ref().unwrap() }
}

#[cfg(all(feature = "fancy-panic", not(test)))]
#[inline(never)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    let stdout = stdout();
    let _ = writeln!(&stdout, "Panic:\n{:#?}", info);
    loop {}
}

#[cfg(all(not(feature = "fancy-panic"), not(test)))]
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
