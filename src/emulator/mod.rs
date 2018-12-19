//! This module contains all the code for the emulator itself.

/* External crates */
extern crate rand;

/* Imports */
use super::display;

/* Public interface */
pub mod chip8;

/* Internal Mods */
mod opcode;
mod register;
