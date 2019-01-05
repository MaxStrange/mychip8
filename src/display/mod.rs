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
mod pixelgrid;
mod rampanel;
mod stackpanel;

/* Useful types for this module internally */

#[derive(Debug, Clone, Copy)]
struct Point32 {
    pub x: u32,
    pub y: u32,
}

/// A bunch of arguments that the GUI may need to draw at any given time.
pub struct DrawingContext<'a> {
    /// Piston Event for this drawing.
    pub event: &'a piston_window::Event,
    /// The current program counter.
    pub pc: u16,
    /// The RAM to draw.
    pub ram: &'a [u8],
    /// The stack pointer.
    pub sp: u8,
    /// The stack to draw.
    pub stack: &'a [u16],
    /// The Piston Window to draw in.
    pub window: &'a mut piston_window::PistonWindow,
}

#[derive(Debug, Clone)]
struct Rectangle {
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
