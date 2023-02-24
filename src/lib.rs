//! The Neotron SDK
//!
//! Defines the API supplied to applications that run on Neotron OS

#![no_std]

// ============================================================================
// Imports
// ============================================================================

// None

// ============================================================================
// Constants
// ============================================================================

/// Maximum length of a filename (with no directory components), including the
/// extension.
const MAX_FILENAME_LEN: usize = 11;

// ============================================================================
// Types
// ============================================================================

/// The type of the function which starts up the Operating System. The BIOS
/// finds and calls this function.
pub type OsStartFn = extern "C" fn(&crate::Api) -> !;

#[repr(C)]
pub struct Api {
    /// Open a file, given a path as UTF-8 string.
    ///
    /// Path may be relative to current directory, or it may be an absolute
    /// path.
    open: fn(path: *const u8, path_len: usize, flags: i32) -> Result<FileHandle>,
    /// Close a previously opened file.
    close: fn(fd: FileHandle) -> Result<()>,
    /// Write to an open file, returning how much was actually written.
    write: fn(fd: FileHandle, buffer: *const u8, buffer_len: usize) -> Result<usize>,
    /// Read from an open file, returning how much was actually read.
    read: fn(fd: FileHandle, buffer: *mut u8, buffer_len: usize) -> Result<usize>,
    /// Move the file offset (for the given file handle) to the given position
    seek_set: fn(fd: FileHandle, position: u64) -> Result<()>,
    /// Move the file offset (for the given file handle) relative to the current position
    seek_cur: fn(fd: FileHandle, offset: i64) -> Result<()>,
    /// Move the file offset (for the given file handle) to the end of the file
    seek_end: fn(fd: FileHandle) -> Result<()>,
    /// Rename a file
    rename: fn(
        old_path: *const u8,
        old_path_len: usize,
        new_path: *const u8,
        new_path_len: usize,
    ) -> Result<()>,
    /// Perform a special I/O control operation.
    ioctl: fn(fd: FileHandle, command: u64, value: u64) -> Result<u64>,
    /// Open a directory, given a path as a UTF-8 string.
    opendir: fn(path: *const u8, path_len: usize) -> Result<DirHandle>,
    /// Close a previously opened directory.
    closedir: fn(dir: DirHandle) -> Result<()>,
    /// Read from an open directory
    readdir: fn(dir: DirHandle, dir_entry: *mut DirEntry) -> Result<()>,
    /// Get information about a file
    stat: fn(path: *const u8, path_len: usize, stat: *mut Stat) -> Result<()>,
    /// Get information about an open file
    fstat: fn(fd: FileHandle, stat: *mut Stat) -> Result<()>,
    /// Change the current directory
    chdir: fn(path: *const u8, path_len: usize) -> Result<()>,
    /// Change the current directory to the open directory
    dchdir: fn(dir: DirHandle) -> Result<()>,
    /// Allocate some memory
    malloc: fn(size: usize, alignment: u8) -> Result<*mut ()>,
    /// Free some previously allocated memory
    free: fn(ptr: *mut (), size: usize, alignment: u8),
}

/// Represents a time on a clock with millisecond accuracy.
#[repr(C)]
pub struct SystemTime(u64);

/// Represents an open file
#[repr(C)]
pub struct FileHandle(u8);

/// Represents an open directory
#[repr(C)]
pub struct DirHandle(u8);

/// Describes how something has failed
#[repr(C)]
pub struct Error(u32);

/// Describes an entry in a directory.
///
/// This is set up for 8.3 filenames on MS-DOS FAT32 partitions currently.
#[repr(C)]
pub struct DirEntry {
    /// The name and extension of the file
    pub name: [u8; MAX_FILENAME_LEN],
    /// File attributes (Directory, Volume, etc)
    pub attr: u8,
    /// How big is this file
    pub file_size: u64,
    /// When was the file created
    pub ctime: SystemTime,
    /// When was the last modified
    pub mtime: SystemTime,
}

/// Describes a file on disk.
///
/// This is set up for 8.3 filenames on MS-DOS FAT32 partitions currently.
#[repr(C)]
pub struct Stat {
    /// Which volume is this file on
    pub volume: u8,
    /// File attributes (Directory, Volume, etc)
    pub attr: u8,
    /// How big is this file
    pub file_size: u64,
    /// When was the file created
    pub ctime: SystemTime,
    /// When was the last modified
    pub mtime: SystemTime,
}

/// All API functions which can fail return this type. We don't use the
/// `Result` type from the standard library because that isn't FFI safe and
/// may change layout between compiler versions.
#[repr(C)]
pub enum Result<T> {
    /// The operation succeeded (the same as `core::result::Result::Ok`).
    Ok(T),
    /// The operation failed (the same as `core::result::Result::Err`).
    Err(Error),
}

/// All API functions which take/return optional values return this type. We
/// don't use the `Option` type from the standard library because that isn't
/// FFI safe and may change layout between compiler versions.
#[repr(C)]
pub enum Option<T> {
    /// There is some data (the same as `core::option::Option::Some`)
    Some(T),
    /// There is no data (the same as `core::option::Option::None`)
    None,
}

// ============================================================================
// Impls
// ============================================================================
