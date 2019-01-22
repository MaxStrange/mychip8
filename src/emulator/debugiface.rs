//! This module contains the debug commands and responses, mostly to refactor them out of the chip8 module.

use super::Address;

/// The different commands the emulator understands. Used for debugging.
#[derive(Debug)]
pub enum EmulatorCommand {
    /// Exit the emulator thread.
    Exit,
    /// Peek from address to address + nbytes.
    PeekAddr(Address, usize),
    /// Peek at register I.
    PeekI,
    /// Peek at the PC
    PeekPC,
    /// Peek at the given register
    PeekReg(u8),
    /// Peek at the sound timer's current value.
    PeekSoundTimer,
    /// Peek at the SP
    PeekSP,
    /// Peek at the whole stack.
    PeekStack,
    /// Resume normal execution of the program.
    ResumeExecution,
    /// Set the clock rate to the given value.
    SetClockRate(u64),
}

/// The possible responses from the emulator in response to EmulatorCommands
#[derive(Debug)]
pub enum EmulatorResponse {
    /// Returns the contents of register I (index register).
    I(u16),
    /// Returns a bunch of bytes.
    MemorySlice(Vec<u8>),
    /// Returns the current program counter.
    PC(u16),
    /// Returns the contents of a register.
    Reg(u8),
    /// Returns the current value of the sound timer.
    SoundTimer(u8),
    /// Returns the current stack pointer.
    SP(u8),
    /// Returns the current stack.
    Stack(Vec<u16>),
}
