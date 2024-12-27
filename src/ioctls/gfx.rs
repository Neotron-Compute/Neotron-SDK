//! Graphics ioctl constants

/// Clear the screen
///
/// The corresponding value is the fill colour, which is taken modulo the number of on-screen colours.
pub const COMMAND_CLEAR_SCREEN: u64 = 0;

/// Plot a chunky pixel
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

/// Calculate a 64-bit value argument for a gfx ioctl
pub fn chunky_plot_value(x: u16, y: u16, colour: u32) -> u64 {
    (x as u64) << 48 | (y as u64) << 32 | (colour & 0xFFFFFF) as u64
}

/// Calculate a 64-bit value argument for a gfx ioctl
pub fn change_mode_value(mode: crate::VideoMode, fb_ptr: *mut u32) -> u64 {
    let fb_ptr = fb_ptr as usize as u64;
    let mode = mode.as_u8() as u64;
    mode << 32 | fb_ptr
}

// End of file
