use super::opcode::Opcode;
use super::register::{Register, RegisterArray};
use std::fmt::Write;

/// The length of our RAM in bytes
const MEMORY_LENGTH_NBYTES: usize = 4096;
/// The start address for a program binary
const PROGRAM_START_BYTE_ADDR: u16 = 0x0200;
/// The largest a program can be in bytes
const MAX_PROGRAM_SIZE_NBYTES: usize = MEMORY_LENGTH_NBYTES - (PROGRAM_START_BYTE_ADDR as usize);

/// The Chip 8 emulator
pub struct Chip8 {
    /// The RAM:
    /// 0x0000 to 0x01FF is reserved for the interpreter
    /// 0x0200 to 0x0FFF is where the ROM will be loaded and scratch space for the program
    memory: [u8; MEMORY_LENGTH_NBYTES],
    /// The Chip-8 has 15 1-byte general purpose registers and one that is used as a carry flag.
    registers:  RegisterArray,
    /// Program counter
    pc: u16,
    /// Special index register - generally used to store memory addresses
    index: u16,
    /// Stack pointer - simply an index into the stack, which is up to 16 addresses
    sp: u8,
    /// The stack is implemented as its own array of 16 16-bit values, rather than just a section of RAM
    stack: [u16; 16],
}

impl Chip8 {
    /// Create a new instance of the emulator.
    pub fn new() -> Self {
        Chip8 {
            memory: [0u8; MEMORY_LENGTH_NBYTES],
            registers: RegisterArray::new(),
            pc: 0,
            index: 0,
            sp: 0,
            stack: [0u16; 16],
        }
    }

    /// Attempts to load the given binary into RAM and run it.
    pub fn load(&mut self, binary: &Vec<u8>) -> Result<(), String> {
        if binary.len() < MAX_PROGRAM_SIZE_NBYTES {
            let mut index = 0x0200;
            for byte in binary {
                self.memory[index] = *byte;
                index += 1;
            }
            Ok(())
        } else {
            let mut msg = String::new();
            write!(msg,
                    "Binary is too large. Maximum size is {} bytes, but this binary is {} bytes.",
                    MAX_PROGRAM_SIZE_NBYTES,
                    binary.len()
                  ).unwrap();
            Err(msg)
        }
    }

    /// Runs the emulator forever.
    pub fn run(&mut self) -> ! {
        loop {
            // Fetch an instruction with pc
            let msb = self.memory[self.pc as usize];
            let lsb = self.memory[(self.pc + 1) as usize];
            let instruction: u16 = ((msb as u16) << 8) | (lsb as u16);

            // Decode opcode
            let opcode = match Opcode::new(instruction) {
                Ok(o) => o,
                Err(msg) => {
                    panic!("Problem with instruction {:x}: {}", instruction, msg)
                },
            };

            // Execute instruction
            // TODO
        }
    }
}
