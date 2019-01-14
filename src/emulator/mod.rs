//! This module contains all the code for the emulator itself.

/* External crates */
extern crate rand;

/* Imports */
use super::display;

/* Public interface */
pub mod chip8;
pub mod debugiface;

/* Internal Mods */
mod keyboard;
mod opcode;
mod register;

/* Some datatypes that are common to this whole module */

/// An address in RAM. RAM's address space can be described by 12 bits.
type Address = u16;
