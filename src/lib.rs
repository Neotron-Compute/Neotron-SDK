//! The Neotron SDK
//!
//! Defines the API supplied to applications that run on Neotron OS

#![no_std]

// ============================================================================
// Imports
// ============================================================================

use bitflags::bitflags;

pub use neotron_ffi::{FfiBuffer, FfiByteSlice, FfiString};

// ============================================================================
// Constants
// ============================================================================

/// Maximum length of a filename (with no directory components), including the
/// extension.
pub const MAX_FILENAME_LEN: usize = 11;

/// Maximum length of a path to a file.
///
/// This is shorter than on MS-DOS, to save on memory.
pub const MAX_PATH_LEN: usize = 64;

#[cfg(feature = "application")]
extern "C" {
    fn main() -> i32;
}

// ============================================================================
// Types
// ============================================================================

/// The type of the application entry-point.
///
/// The OS calls a function of this type.
pub type AppStartFn = extern "C" fn(*mut crate::Api) -> i32;

/// The result type for any SDK API function.
///
/// Like a [`neotron_ffi::FfiResult`] but the error type is [`Error`].
pub type Result<T> = neotron_ffi::FfiResult<T, Error>;

/// The syscalls provided by the Neotron OS to a Neotron Application.
#[repr(C)]
pub struct Api {
    /// Open a file, given a path as UTF-8 string.
    ///
    /// If the file does not exist, or is already open, it returns an error.
    ///
    /// Path may be relative to current directory, or it may be an absolute
    /// path.
    pub open: extern "C" fn(path: FfiString, flags: FileFlags) -> Result<FileHandle>,
    /// Close a previously opened file.
    pub close: extern "C" fn(fd: FileHandle) -> Result<()>,
    /// Write to an open file handle, blocking until everything is written.
    ///
    /// Some files do not support writing and will produce an error.
    pub write: extern "C" fn(fd: FileHandle, buffer: FfiByteSlice) -> Result<()>,
    /// Read from an open file, returning how much was actually read.
    ///
    /// If you hit the end of the file, you might get less data than you asked for.
    pub read: extern "C" fn(fd: FileHandle, buffer: FfiBuffer) -> Result<usize>,
    /// Move the file offset (for the given file handle) to the given position.
    ///
    /// Some files do not support seeking and will produce an error.
    pub seek_set: extern "C" fn(fd: FileHandle, position: u64) -> Result<()>,
    /// Move the file offset (for the given file handle) relative to the current position
    ///
    /// Some files do not support seeking and will produce an error.
    pub seek_cur: extern "C" fn(fd: FileHandle, offset: i64) -> Result<()>,
    /// Move the file offset (for the given file handle) to the end of the file
    ///
    /// Some files do not support seeking and will produce an error.
    pub seek_end: extern "C" fn(fd: FileHandle) -> Result<()>,
    /// Rename a file
    pub rename: extern "C" fn(old_path: FfiString, new_path: FfiString) -> Result<()>,
    /// Perform a special I/O control operation.
    pub ioctl: extern "C" fn(fd: FileHandle, command: u64, value: u64) -> Result<u64>,
    /// Open a directory, given a path as a UTF-8 string.
    pub opendir: extern "C" fn(path: FfiString) -> Result<DirHandle>,
    /// Close a previously opened directory.
    pub closedir: extern "C" fn(dir: DirHandle) -> Result<()>,
    /// Read from an open directory
    pub readdir: extern "C" fn(dir: DirHandle) -> Result<DirEntry>,
    /// Get information about a file
    pub stat: extern "C" fn(path: FfiString) -> Result<Stat>,
    /// Get information about an open file
    pub fstat: extern "C" fn(fd: FileHandle) -> Result<Stat>,
    /// Delete a file or directory
    ///
    /// If the file is currently open, or the directory has anything in it, this
    /// will give an error.
    pub delete: extern "C" fn(path: FfiString) -> Result<()>,
    /// Change the current directory
    ///
    /// Relative file paths are taken to be relative to the current directory.
    ///
    /// Unlike on MS-DOS, there is only one current directory for the whole
    /// system, not one per drive.
    pub chdir: extern "C" fn(path: FfiString) -> Result<()>,
    /// Change the current directory to the open directory
    ///
    /// Relative file paths are taken to be relative to the current directory.
    ///
    /// Unlike on MS-DOS, there is only one current directory for the whole
    /// system, not one per drive.
    pub dchdir: extern "C" fn(dir: DirHandle) -> Result<()>,
    /// Allocate some memory
    pub malloc: extern "C" fn(size: usize, alignment: usize) -> Result<*mut core::ffi::c_void>,
    /// Free some previously allocated memory
    pub free: extern "C" fn(ptr: *mut core::ffi::c_void, size: usize, alignment: usize),
}

/// Describes how something has failed
#[repr(C)]
pub enum Error {
    /// The given file path was not found
    FileNotFound,
    /// Tried to write to a read-only file
    FileReadOnly,
    /// Reached the end of the file
    EndOfFile,
    /// The API has not been implemented
    Unimplemented,
    /// An invalid argument was given to the API
    InvalidArg,
    /// A bad file handle was given to the API
    BadFileHandle,
    /// An device-specific error occurred. Look at the BIOS source for more details.
    DeviceSpecific,
}

/// Represents an open file
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FileHandle(u8);

impl FileHandle {
    /// The magic file ID for Standard Output
    const STDOUT: u8 = 0;

    /// Construct a new `FileHandle` from an integer.
    ///
    /// Only the OS should call this.
    ///
    /// # Safety
    ///
    /// The integer given must be a valid index for an open File.
    #[cfg(feature = "os")]
    pub const fn new(value: u8) -> FileHandle {
        FileHandle(value)
    }

    /// Create a file handle for Standard Output
    pub const fn new_stdout() -> FileHandle {
        FileHandle(Self::STDOUT)
    }

    /// Get the numeric value of this File Handle
    pub const fn value(&self) -> u8 {
        self.0
    }
}

/// Represents an open directory
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct DirHandle(u8);

impl DirHandle {
    /// Construct a new `DirHandle` from an integer.
    ///
    /// Only the OS should call this.
    ///
    /// # Safety
    ///
    /// The integer given must be a valid index for an open Directory.
    #[cfg(feature = "os")]
    pub const fn new(value: u8) -> DirHandle {
        DirHandle(value)
    }

    /// Get the numeric value of this Directory Handle
    pub const fn value(&self) -> u8 {
        self.0
    }
}

/// Describes an entry in a directory.
///
/// This is set up for 8.3 filenames on MS-DOS FAT32 partitions currently.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirEntry {
    /// The name and extension of the file
    pub name: [u8; MAX_FILENAME_LEN],
    /// File properties
    pub properties: Stat,
}

/// Describes a file on disk.
///
/// This is set up for 8.3 filenames on MS-DOS FAT32 partitions currently.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stat {
    /// How big is this file
    pub file_size: u64,
    /// When was the file created
    pub ctime: FileTime,
    /// When was the last modified
    pub mtime: FileTime,
    /// File attributes (Directory, Volume, etc)
    pub attr: FileAttributes,
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// Describes the attributes of a file.
    pub struct FileFlags: u8 {
        /// Enable write support for this file.
        const WRITE = 0x01;
        /// Create the file if it doesn't exist.
        const CREATE = 0x02;
        /// Truncate the file to zero length upon opening.
        const TRUNCATE = 0x04;
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    /// The attributes a file on disk can have.alloc
    ///
    /// Based on that supported by the FAT32 file system.
    pub struct FileAttributes: u8 {
        /// File is read-only
        const READ_ONLY = 0x01;
        /// File should not appear in directory listing
        const HIDDEN = 0x02;
        /// File should not be moved on disk
        const SYSTEM = 0x04;
        /// File is a volume label
        const VOLUME = 0x08;
        /// File is a directory
        const DIRECTORY = 0x10;
        /// File has not been backed up
        const ARCHIVE = 0x20;
        /// File is actually a device
        const DEVICE = 0x40;
    }
}

/// Represents an instant in time, in the local time zone.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct FileTime {
    /// Add 1970 to this file to get the calendar year
    pub year_since_1970: u8,
    /// Add one to this value to get the calendar month
    pub zero_indexed_month: u8,
    /// Add one to this value to get the calendar day
    pub zero_indexed_day: u8,
    /// The number of hours past midnight
    pub hours: u8,
    /// The number of minutes past the hour
    pub minutes: u8,
    /// The number of seconds past the minute
    pub seconds: u8,
}

// ============================================================================
// Functions
// ============================================================================

#[cfg(feature = "application")]
mod application {
    use super::*;
    use core::sync::atomic::{AtomicPtr, Ordering};

    #[link_section = ".entry_point"]
    #[used]
    pub static APP_ENTRY: AppStartFn = app_entry;

    static API: AtomicPtr<Api> = AtomicPtr::new(core::ptr::null_mut());

    /// The function the OS calls to start the application.
    ///
    /// Will jump to the application entry point, and `extern "C"` function
    /// called `main`.
    extern "C" fn app_entry(api: *mut crate::Api) -> i32 {
        API.store(api as *mut Api, Ordering::Relaxed);
        unsafe { main() }
    }

    /// Write to a file handle.
    pub fn write(fd: FileHandle, data: &[u8]) -> Result<()> {
        let api = get_api();
        (api.write)(fd, FfiByteSlice::new(data))
    }

    /// Get the API structure so you can call APIs manually.
    fn get_api() -> &'static Api {
        let ptr = API.load(Ordering::Relaxed);
        unsafe { ptr.as_ref().unwrap() }
    }
}

#[cfg(feature = "application")]
pub use application::*;

// ============================================================================
// End of File
// ============================================================================
