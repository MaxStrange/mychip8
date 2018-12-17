use super::opcode::Opcode;
use super::register::{Register, RegisterArray};
use std::fmt::Write;

/// The length of our RAM in bytes
const MEMORY_LENGTH_NBYTES: usize = 4096;
/// The start address for a program binary
const PROGRAM_START_BYTE_ADDR: u16 = 0x0200;
/// The largest a program can be in bytes
const MAX_PROGRAM_SIZE_NBYTES: usize = MEMORY_LENGTH_NBYTES - (PROGRAM_START_BYTE_ADDR as usize);
/// There are this many addresses in the special stack array at most.
const STACK_SIZE_N_ADDRS: usize = 16;

/// An address in RAM. RAM's address space can be described by 12 bits.
type Address = u16;

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
    stack: [u16; STACK_SIZE_N_ADDRS],
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
            match self.execute(opcode) {
                Ok(()) => (),
                Err(msg) => {
                    panic!("Problem executing instruction {:?}: {}", opcode, msg)
                },
            }
        }
    }

    fn execute_sys(&mut self, addr: Address) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_cls(&mut self) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_ret(&mut self) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_jp(&mut self, addr: Address) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_call(&mut self, addr: Address) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_sevxbyte(&mut self, x: Register, byte: u8) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_sevxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_ldvxbyte(&mut self, x: Register, byte: u8) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_orvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_andvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_xorvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_addvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    fn execute_subvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_shrvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_subnvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_shlvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_snevxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_ldiaddr(&mut self, addr: Address) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_jpv0addr(&mut self, addr: Address) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_rndvxbyte(&mut self, x: Register, byte: u8) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_drwvxvynibble(&mut self, x: Register, y: Register, byte: u8) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_skpvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_sknpvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_ldvxdt(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_ldvxk(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_lddtv(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_ldstvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_addivx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_ldfvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_ldbvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_ldivx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute_ldvxi(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    fn execute(&mut self, op: Opcode) -> Result<(), String> {
        match op {
            Opcode::SYS(addr) => self.execute_sys(addr),
            Opcode::CLS => self.execute_cls(),
            Opcode::RET => self.execute_ret(),
            Opcode::JP(addr) => self.execute_jp(addr),
            Opcode::CALL(addr) => self.execute_call(addr),
            Opcode::SEVxByte(x, kk) => self.execute_sevxbyte(x, kk),
            Opcode::SEVxVy(x, y) => self.execute_sevxvy(x, y),
            Opcode::LDVxByte(x, kk) => self.execute_ldvxbyte(x, kk),
            Opcode::ORVxVy(x, y) => self.execute_orvxvy(x, y),
            Opcode::ANDVxVy(x, y) => self.execute_andvxvy(x, y),
            Opcode::XORVxVy(x, y) => self.execute_xorvxvy(x, y),
            Opcode::ADDVxVy(x, y) => self.execute_addvxvy(x, y),
            Opcode::SUBVxVy(x, y) => self.execute_subvxvy(x, y),
            Opcode::SHRVx(x) => self.execute_shrvx(x),
            Opcode::SUBNVxVy(x, y) => self.execute_subnvxvy(x, y),
            Opcode::SHLVx(x) => self.execute_shlvx(x),
            Opcode::SNEVxVy(x, y) => self.execute_snevxvy(x, y),
            Opcode::LDIAddr(addr) => self.execute_ldiaddr(addr),
            Opcode::JPV0Addr(addr) => self.execute_jpv0addr(addr),
            Opcode::RNDVxByte(x, kk) => self.execute_rndvxbyte(x, kk),
            Opcode::DRWVxVyNibble(x, y, n) => self.execute_drwvxvynibble(x, y, n),
            Opcode::SKPVx(x) => self.execute_skpvx(x),
            Opcode::SKNPVx(x) => self.execute_sknpvx(x),
            Opcode::LDVxDT(x) => self.execute_ldvxdt(x),
            Opcode::LDVxK(x) => self.execute_ldvxk(x),
            Opcode::LDDTVx(x) => self.execute_lddtv(x),
            Opcode::LDSTVx(x) => self.execute_ldstvx(x),
            Opcode::ADDIVx(x) => self.execute_addivx(x),
            Opcode::LDFVx(x) => self.execute_ldfvx(x),
            Opcode::LDBVx(x) => self.execute_ldbvx(x),
            Opcode::LDIVx(x) => self.execute_ldivx(x),
            Opcode::LDVxI(x) => self.execute_ldvxi(x),
        }
    }
}
