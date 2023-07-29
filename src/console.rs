//! Helper functions for sending ANSI sequences

// ============================================================================
// Imports
// ============================================================================

use core::fmt::Write;

use crate::File;

// ============================================================================
// Types
// ============================================================================

/// Represents a position on a screen.
///
/// A position is 0-indexed. That is (0, 0) is the top-left corner.
///
/// Translation to 1-based is performed before sending the ANSI sequences.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

impl Position {
    pub const fn origin() -> Position {
        Position { row: 0, col: 0 }
    }
}

/// Represents a Select Graphic Rendition parameter you can send in an SGR ANSI
/// sequence.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SgrParam {
    Reset = 0,
    Bold = 1,
    Reverse = 7,
    NotBold = 22,
    NotReverse = 27,
    FgBlack = 30,
    FgRed = 31,
    FgGreen = 32,
    FgYellow = 33,
    FgBlue = 34,
    FgMagenta = 35,
    FgCyan = 36,
    FgWhite = 37,
    BgBlack = 40,
    BgRed = 41,
    BgGreen = 42,
    BgYellow = 43,
    BgBlue = 44,
    BgMagenta = 45,
    BgCyan = 46,
    BgWhite = 47,
}

// ============================================================================
// Functions
// ============================================================================

/// Erase the screen
pub fn clear_screen(f: &mut File) {
    let _ = f.write_str("\u{001b}[2J");
}

/// Turn the cursor on
pub fn cursor_on(f: &mut File) {
    let _ = f.write_str("\u{001b}[?25h");
}

/// Turn the cursor off
pub fn cursor_off(f: &mut File) {
    let _ = f.write_str("\u{001b}[?25l");
}

/// Move the cursor to the given position
pub fn move_cursor(f: &mut File, pos: Position) {
    let _ = write!(f, "\u{001b}[{};{}H", 1 + pos.row, 1 + pos.col);
}

/// Change the background
///
/// Only values 0..8 will work.
pub fn set_sgr<T>(f: &mut File, values: T)
where
    T: IntoIterator<Item = SgrParam>,
{
    let _ = write!(f, "\u{001b}[");
    let mut iter = values.into_iter();
    if let Some(value) = iter.next() {
        let _ = write!(f, "{}", value as u8);
    }
    for value in iter {
        let _ = write!(f, ";{}", value as u8);
    }
    let _ = write!(f, "m");
}

// ============================================================================
// End of File
// ============================================================================
