//! Graphics ioctl constants

/// Clear the screen
///
/// The corresponding value is the fill colour, which is taken modulo the number of on-screen colours.
pub const COMMAND_CLEAR_SCREEN: u64 = 0;

/// Plot a chunky pixel (and move the cursor).
///
/// The command contains [ x | y | mode | colour ].
///
/// * `x` is 16 bits and marks the horizontal position (0 is left)
/// * `y` is 16 bits and marks the vertical position (0 is top)
/// * `mode` is 8 bits and is currently ignored
/// * `colour` is 24 bits, and is taken modulo the number of on-screen colours
///
/// Use [`chunky_plot_value`] to create a suitable value for this ioctl command.
pub const COMMAND_CHUNKY_PLOT: u64 = 1;

/// Change graphics mode
///
/// The command contains the video mode in the upper 32 bits and a pointer to a
/// framebuffer in the lower 32 bits.
///
/// The framebuffer pointer must point to a 32-bit aligned region of memory
/// that is large enough for the selected mode. If you pass `null`, then the OS
/// will attempt to allocate a framebuffer for you.
///
/// Use [`change_mode_value`] to construct a value.
pub const COMMAND_CHANGE_MODE: u64 = 2;

/// Move the cursor
///
/// The command contains [ x | y | <padding> ].
///
/// The graphics handle maintains a virtual cursor position. This is used for
/// plotting lines for example - you move the cursor to one end of the line, and
/// then issue a LINE_DRAW ioctl containing the other end of the line. This saves
/// having to try and pass two sets of co-ordinates within one ioctl.
///
/// Use [`move_cursor_value`] to construct a value.
pub const COMMAND_MOVE_CURSOR: u64 = 3;

/// Draw a line
///
/// The command contains [ x | y | mode | colour ].
///
/// * `x` is 16 bits and marks the final horizontal position (0 is left)
/// * `y` is 16 bits and marks the final vertical position (0 is top)
/// * `mode` is 8 bits and is currently ignored
/// * `colour` is 24 bits, and is taken modulo the number of on-screen colours
///
/// The start position is the cursor position. The cursor is updated to the final position.
///
/// Use [`draw_line_value`] to construct a value.
pub const COMMAND_DRAW_LINE: u64 = 4;

/// Set a palette entry
///
/// The command contains [ <padding> | II | RR | GG | BB ]
///
/// II, RR, GG and BB are 8-bit values where II is the index into the 256 long
/// palette, and RR, GG and BB are the 24-bit RGB colour for that index.
///
/// Use [`set_palette_value`] to construct a value.
pub const COMMAND_SET_PALETTE: u64 = 5;

/// Calculate a 64-bit value argument for the [`COMMAND_CHUNKY_PLOT`] gfx ioctl
pub fn chunky_plot_value(x: u16, y: u16, colour: u32) -> u64 {
    (x as u64) << 48 | (y as u64) << 32 | (colour & 0xFFFFFF) as u64
}

/// Calculate a 64-bit value argument for the [`COMMAND_CHANGE_MODE`] gfx ioctl
pub fn change_mode_value(mode: crate::VideoMode, fb_ptr: *mut u32) -> u64 {
    let fb_ptr = fb_ptr as usize as u64;
    let mode = mode.as_u8() as u64;
    mode << 32 | fb_ptr
}

/// Calculate a 64-bit value argument for the [`COMMAND_MOVE_CURSOR`] gfx ioctl
pub fn move_cursor_value(x: u16, y: u16) -> u64 {
    (x as u64) << 48 | (y as u64) << 32
}

/// Calculate a 64-bit value argument for the [`COMMAND_DRAW_LINE`] gfx ioctl
pub fn draw_line_value(end_x: u16, end_y: u16, colour: u32) -> u64 {
    (end_x as u64) << 48 | (end_y as u64) << 32 | (colour & 0xFFFFFF) as u64
}

/// Calculate a 64-bit value argument for the [`COMMAND_SET_PALETTE`] gfx ioctl
pub fn set_palette_value(index: u8, r: u8, g: u8, b: u8) -> u64 {
    let mut result = (index as u64) << 24;
    result |= (r as u64) << 16;
    result |= (g as u64) << 8;
    result |= b as u64;
    result
}

// End of file
