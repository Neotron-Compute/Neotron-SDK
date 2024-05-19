/**
 * Header file for the Neoton OS API.
 * 
 * Copyright (c) 2023 Jonathan Pallant and the Neotron Developers
 * 
 * This file is licensed under the MIT or Apache 2.0 licences, at your option.
 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Maximum length of a filename (with no directory components), including the
 * extension.
 */
#define MAX_FILENAME_LEN 11

/**
 * The character that separates one directory name from another directory name.
 */
#define Path_PATH_SEP '/'

/**
 * The character that separates drive specifiers from directories.
 */
#define Path_DRIVE_SEP ':'

/**
 * Describes how something has failed
 */
typedef enum Error
{
  /**
   * The given file/directory path was not found
   */
  NotFound,
  /**
   * Tried to write to a read-only file
   */
  FileReadOnly,
  /**
   * Reached the end of the file
   */
  EndOfFile,
  /**
   * The API has not been implemented
   */
  Unimplemented,
  /**
   * An invalid argument was given to the API
   */
  InvalidArg,
  /**
   * A bad handle was given to the API
   */
  BadHandle,
  /**
   * An device-specific error occurred. Look at the BIOS source for more details.
   */
  DeviceSpecific,
  /**
   * The OS does not have enough memory
   */
  OutOfMemory,
  /**
   * The given path was invalid
   */
  InvalidPath,
} Error;

/**
 * Represents an open directory
 */
typedef struct Handle
{
  uint8_t _0;
} Handle;

typedef enum FfiResult_Tag
{
  /**
   * The operation succeeded (like [`core::result::Result::Ok`]).
   */
  FfiResult_Ok,
  /**
   * The operation failed (like [`core::result::Result::Err`]).
   */
  FfiResult_Err,
} FfiResult_Tag;

typedef struct FfiResult_Handle
{
  FfiResult_Tag tag;
  union
  {
    struct
    {
      struct Handle ok;
    };
    struct
    {
      enum Error err;
    };
  };
} FfiResult_Handle;

/**
 * A Rust u8 slice, but compatible with FFI. Assume the lifetime is only valid
 * until the callee returns to the caller.
 */
typedef struct FfiByteSlice
{
  /**
   * A pointer to the data
   */
  const uint8_t *data;
  /**
   * The number of bytes we are pointing at
   */
  uintptr_t data_len;
} FfiByteSlice;

/**
 * A Rust UTF-8 string, but compatible with FFI.
 *
 * Assume the lifetime is only valid until the callee returns to the caller. Is
 * not null-terminated.
 */
typedef struct FfiString
{
  struct FfiByteSlice _0;
} FfiString;

typedef struct FfiResult_usize
{
  FfiResult_Tag tag;
  union
  {
    struct
    {
      uintptr_t ok;
    };
    struct
    {
      enum Error err;
    };
  };
} FfiResult_usize;

/**
 * A Rust u8 mutable slice, but compatible with FFI. Assume the lifetime is
 * only valid until the callee returns to the caller.
 */
typedef struct FfiBuffer
{
  /**
   * A pointer to where the data can be put
   */
  uint8_t *data;
  /**
   * The maximum number of bytes we can store in this buffer
   */
  uintptr_t data_len;
} FfiBuffer;

typedef struct FfiResult_u64
{
  FfiResult_Tag tag;
  union
  {
    struct
    {
      uint64_t ok;
    };
    struct
    {
      enum Error err;
    };
  };
} FfiResult_u64;

/**
 * Represents an instant in time, in the local time zone.
 */
typedef struct Time
{
  /**
   * Add 1970 to this file to get the calendar year
   */
  uint8_t year_since_1970;
  /**
   * Add one to this value to get the calendar month
   */
  uint8_t zero_indexed_month;
  /**
   * Add one to this value to get the calendar day
   */
  uint8_t zero_indexed_day;
  /**
   * The number of hours past midnight
   */
  uint8_t hours;
  /**
   * The number of minutes past the hour
   */
  uint8_t minutes;
  /**
   * The number of seconds past the minute
   */
  uint8_t seconds;
} Time;

/**
 * Describes a file on disk.
 *
 * This is set up for 8.3 filenames on MS-DOS FAT32 partitions currently.
 */
typedef struct Stat
{
  /**
   * How big is this file
   */
  uint64_t file_size;
  /**
   * When was the file created
   */
  struct Time ctime;
  /**
   * When was the last modified
   */
  struct Time mtime;
  /**
   * File attributes (Directory, Volume, etc)
   */
  uint8_t attr;
} Stat;

/**
 * Describes an entry in a directory.
 *
 * This is set up for 8.3 filenames on MS-DOS FAT32 partitions currently.
 */
typedef struct Entry
{
  /**
   * The name and extension of the file.
   *
   * The name and extension are separated by a single '.'.
   *
   * The filename will be in ASCII. Unicode filenames are not supported.
   */
  uint8_t name[MAX_FILENAME_LEN];
  /**
   * The properties for the file/directory this entry represents.
   */
  struct Stat properties;
} Entry;

typedef struct FfiResult_Entry
{
  FfiResult_Tag tag;
  union
  {
    struct
    {
      struct Entry ok;
    };
    struct
    {
      enum Error err;
    };
  };
} FfiResult_Entry;

typedef struct FfiResult_Stat
{
  FfiResult_Tag tag;
  union
  {
    struct
    {
      struct Stat ok;
    };
    struct
    {
      enum Error err;
    };
  };
} FfiResult_Stat;

typedef struct FfiResult_void
{
  FfiResult_Tag tag;
  union
  {
    struct
    {
      enum Error err;
    };
  };
} FfiResult_void;

/**
 * The syscalls provided by the Neotron OS to a Neotron Application.
 */
typedef struct NeotronApi
{
  /**
   * Open a file, given a path as UTF-8 string.
   *
   * If the file does not exist, or is already open, it returns an error.
   *
   * Path may be relative to current directory, or it may be an absolute
   * path.
   *
   * # Limitations
   *
   * * You cannot open a file if it is currently open.
   * * Paths must confirm to the rules for the filesystem for the given drive.
   * * Relative paths are taken relative to the current directory (see `Api::chdir`).
   */
  struct FfiResult_Handle (*open)(struct FfiString path, uint8_t flags);
  /**
   * Close a previously opened file.
   *
   * Closing a file is important, as only this action will cause the
   * directory entry for the file to be updated. Crashing the system without
   * closing a file may cause the directory entry to be incorrect, and you
   * may need to run `CHKDSK` (or similar) on your disk to fix it.
   */
  struct FfiResult_void (*close)(struct Handle fd);
  /**
   * Write to an open file handle, blocking until everything is written.
   *
   * Some files do not support writing and will produce an error. You will
   * also get an error if you run out of disk space.
   *
   * The `buffer` is only borrowed for the duration of the function call and
   * is then forgotten.
   */
  struct FfiResult_void (*write)(struct Handle fd, struct FfiByteSlice buffer);
  /**
   * Read from an open file, returning how much was actually read.
   *
   * You might get less data than you asked for. If you do an `Api::read` and
   * you are already at the end of the file you will get
   * `Err(Error::EndOfFile)`.
   *
   * Data is stored to the given `buffer. The `buffer` is only borrowed for
   * the duration of the function call and is then forgotten.
   */
  struct FfiResult_usize (*read)(struct Handle fd, struct FfiBuffer buffer);
  /**
   * Move the file offset (for the given file handle) to the given position.
   *
   * Some files do not support seeking and will produce an error.
   */
  struct FfiResult_void (*seek_set)(struct Handle fd, uint64_t position);
  /**
   * Move the file offset (for the given file handle) relative to the current position.
   *
   * Returns the new file offset.
   *
   * Some files do not support seeking and will produce an error.
   */
  struct FfiResult_u64 (*seek_cur)(struct Handle fd, int64_t offset);
  /**
   * Move the file offset (for the given file handle) to the end of the file
   *
   * Returns the new file offset.
   *
   * Some files do not support seeking and will produce an error.
   */
  struct FfiResult_u64 (*seek_end)(struct Handle fd);
  /**
   * Rename a file.
   *
   * # Limitations
   *
   * * You cannot rename a file if it is currently open.
   * * You cannot rename a file where the `old_path` and the `new_path` are
   * not on the same drive.
   * * Paths must confirm to the rules for the filesystem for the given drive.
   */
  struct FfiResult_void (*rename)(struct FfiString old_path, struct FfiString new_path);
  /**
   * Perform a special I/O control operation.
   */
  struct FfiResult_u64 (*ioctl)(struct Handle fd, uint64_t command, uint64_t value);
  /**
   * Open a directory, given a path as a UTF-8 string.
   */
  struct FfiResult_Handle (*opendir)(struct FfiString path);
  /**
   * Close a previously opened directory.
   */
  struct FfiResult_void (*closedir)(struct Handle dir);
  /**
   * Read from an open directory
   */
  struct FfiResult_Entry (*readdir)(struct Handle dir);
  /**
   * Get information about a file.
   */
  struct FfiResult_Stat (*stat)(struct FfiString path);
  /**
   * Get information about an open file.
   */
  struct FfiResult_Stat (*fstat)(struct Handle fd);
  /**
   * Delete a file.
   *
   * # Limitations
   *
   * * You cannot delete a file if it is currently open.
   */
  struct FfiResult_void (*deletefile)(struct FfiString path);
  /**
   * Delete a directory.
   *
   * # Limitations
   *
   * * You cannot delete a root directory.
   * * You cannot delete a directory that has any files or directories in it.
   */
  struct FfiResult_void (*deletedir)(struct FfiString path);
  /**
   * Change the current directory.
   *
   * Relative file paths (e.g. passed to `Api::open`) are taken to be relative to the current directory.
   *
   * Unlike on MS-DOS, there is only one current directory for the whole
   * system, not one per drive.
   */
  struct FfiResult_void (*chdir)(struct FfiString path);
  /**
   * Change the current directory to the given open directory.
   *
   * Unlike on MS-DOS, there is only one current directory for the whole
   * system, not one per drive.
   */
  struct FfiResult_void (*dchdir)(struct Handle dir);
  /**
   * Get the current directory.
   *
   * The current directory is stored as UTF-8 into the given buffer. The
   * function returns the number of bytes written to the buffer, or an error.
   * If the function did not return an error, the buffer can be assumed to
   * contain a valid file path. That path will not be null terminated.
   */
  struct FfiResult_usize (*pwd)(struct FfiBuffer path);
  /**
   * Allocate some memory.
   *
   * * `size` - the number of bytes required
   * * `alignment` - the returned address will have this alignment, or
   *   better. For example, pass `4` if you are allocating an array of `u32`.
   */
  struct FfiResult_void (*malloc)(uintptr_t size, uintptr_t alignment);
  /**
   * Free some previously allocated memory.
   *
   * You must pass the same `size` and `alignment` values that you passed to `malloc`.
   */
  void (*free)(void *ptr, uintptr_t size, uintptr_t alignment);
} NeotronApi;
