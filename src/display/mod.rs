//! The display modules contain all the code for displaying stuff to the screen.

/* Externs */
extern crate piston_window;
extern crate rusttype;

/* Public interfaces */
pub mod gui;
pub mod sprite;

/* Internal mods */
mod chip8panel;
mod panel;
mod rampanel;
mod stackpanel;

/* Useful types for this module internally */

#[derive(Debug, Clone, Copy)]
pub struct Point32 {
    pub x: u32,
    pub y: u32,
}

/// A bunch of arguments that the GUI may need to draw at any given time.
#[derive(Debug, Clone)]
pub struct DrawingContext {
    /// The current program counter.
    pub pc: Option<u16>,
    /// The RAM to draw.
    pub ram: Option<Vec<u8>>,
    /// The stack pointer.
    pub sp: Option<u8>,
    /// The stack to draw.
    pub stack: Option<Vec<u16>>,
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    /// Top left point of the rectangle
    pub topleft: Point32,
    /// Bottom right point of the rectangle
    pub bottomright: Point32,
}

impl Rectangle {
    /// The height of the rectangle
    pub fn height(&self) -> u32 {
        self.bottomright.y - self.topleft.y
    }

    /// The width of the rectangle
    pub fn width(&self) -> u32 {
        self.bottomright.x - self.topleft.x
    }
}
