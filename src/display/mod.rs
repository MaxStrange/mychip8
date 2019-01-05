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

struct Point32 {
    x: u32,
    y: u32,
}

/// A bunch of arguments that the GUI may need to draw at any given time.
pub struct DrawingContext<'a> {
    /// Piston Event for this drawing.
    event: &'a piston_window::Event,
    /// The current program counter.
    pc: u16,
    /// The RAM to draw.
    ram: &'a [u8],
    /// The stack pointer.
    sp: u8,
    /// The stack to draw.
    stack: &'a [u16],
    /// The Piston Window to draw in.
    window: &'a mut piston_window::PistonWindow,
}
