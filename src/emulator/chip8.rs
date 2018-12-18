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

    /// Executes a SYS instruction.
    ///
    /// The SYS instruction jumps to a machine code routine at the given address.
    /// This instruction is only used on the old computers on which Chip-8 was originally
    /// implemented. It is ignored by modern interpreters.
    fn execute_sys(&mut self, _addr: Address) -> Result<(), String> {
        // Does nothing - NOP
        Ok(())
    }

    /// Executes a CLS instruction.
    ///
    /// Clears the display.
    fn execute_cls(&mut self) -> Result<(), String> {
        // TODO
        Err("Not yet implemented".to_string())
    }

    /// Executes a RET instruction.
    ///
    /// Sets the program counter to the address at the top of the stack,
    /// then subtracts one from the stack pointer.
    fn execute_ret(&mut self) -> Result<(), String> {
        if self.sp as usize >= self.stack.len() {
            let mut errmsg = String::new();
            write!(errmsg, "Stack pointer ({}) is too big.", self.sp).unwrap();
            Err(errmsg)
        } else {
            self.pc = self.stack[self.sp as usize];
            self.sp -= 1;
            Ok(())
        }
    }

    /// Executes a JP instruction.
    ///
    /// Sets the program counter to the given address.
    fn execute_jp(&mut self, addr: Address) -> Result<(), String> {
        if addr as usize >= self.memory.len() {
            let mut errmsg = String::new();
            write!(errmsg, "Address {} is larger than the memory.", addr).unwrap();
            Err(errmsg)
        } else {
            self.pc = self.memory[addr as usize] as u16;
            Ok(())
        }
    }

    /// Executes a CALL instruction.
    ///
    /// Increments the stack pointer, puts the current program counter on top of the stack,
    /// then sets the program counter to the given address.
    fn execute_call(&mut self, addr: Address) -> Result<(), String> {
        if (self.sp + 1) as usize >= self.stack.len() {
            let mut errmsg = String::new();
            write!(errmsg, "Stack pointer ({}) is greater than the length of the stack.", self.sp).unwrap();
            Err(errmsg)
        } else {
            self.sp += 1;
            self.stack[self.sp as usize] = self.pc;
            self.pc = addr;
            Ok(())
        }
    }

    /// Executes an SE instruction on register `x` and byte `byte`.
    ///
    /// If the contents of register Vx equal `byte`, the program counter is incremented
    /// by 2 (in other words, we skip the next instruction).
    fn execute_sevxbyte(&mut self, x: Register, byte: u8) -> Result<(), String> {
        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        if vx == byte {
            self.pc += 2;
        }

        Ok(())
    }

    /// Executes an SNE instruction on register `x` and byte `byte`.
    ///
    /// If the contents of register Vx do NOT equal `byte`, the program counter is incremented
    /// by 2 (in other words, we skip the next instruction).
    fn execute_snevxbyte(&mut self, x: Register, byte: u8) -> Result<(), String> {
        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        if vx != byte {
            self.pc += 2;
        }

        Ok(())
    }
    /// Executes an SNE instruction on registers `x` and `y`.
    ///
    /// The values of Vx and Vy are compared and if they are equal, the program counter is
    /// incremented by 2.
    fn execute_sevxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        if vx == vy {
            self.pc += 2;
        }

        Ok(())
    }

    /// Executes an LD instruction on register `x` and byte `byte`.
    ///
    /// Stores the given byte in register `x`.
    fn execute_ldvxbyte(&mut self, x: Register, byte: u8) -> Result<(), String> {
        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx = byte;

        Ok(())
    }

    /// Executes an ADD instruction on register `x` and byte `byte`.
    ///
    /// Adds the value `byte` to the contents of register Vx, then stores the result in Vx.
    fn execute_addvxbyte(&mut self, x: Register, byte: u8) -> Result<(), String> {
        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx += byte;

        Ok(())
    }

    /// Executes an LD instruction on registers `x` and `y`.
    ///
    /// Stores the value of register Vy in register Vx.
    fn execute_ldvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx = vy;

        Ok(())
    }

    /// Executes an OR instruction on registers `x` and `y`.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    fn execute_orvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    /// Executes an AND instruction on registers `x` and `y`.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    fn execute_andvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    /// Executes an XOR instruction on registers `x` and `y`.
    ///
    /// Performs a bitwise XOR on the values of Vx and Vy, then stores the result in Vx.
    fn execute_xorvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    /// Executes an ADD instruction on registers `x` and `y`.
    ///
    /// Adds the value in the register `y` to the value of register `x`, then stores the result in register `x`.
    /// If the result is greater than 255, VF is set to 1, otherwise it is set to 0.
    /// Only the lowest 8 bits are stored in Vx.
    fn execute_addvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    /// Executes a SUB instruction on registers `x` and `y`.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise set it to 0. Then Vy is subtracted from Vx
    /// and the result is stored in Vx.
    fn execute_subvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a SHR instruction on register `x`.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is
    /// bit shifted right by one (in other words, Vx is divided by 2).
    fn execute_shrvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a SUBN instruction on registers `x` and `y`.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy
    /// and the result is stored in Vx.
    fn execute_subnvxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a SHL instruction on register `x`.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is shifted
    /// left by one bit (in other words, Vx is multiplied by 2).
    fn execute_shlvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes an SNE instruction on registers `x` and `y`.
    ///
    /// The values of Vx and Vy are compared and if they are NOT equal, the program counter is
    /// incremented by 2.
    fn execute_snevxvy(&mut self, x: Register, y: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes an LD instruction on register I and `addr`.
    ///
    /// The value of regsiter I is set to the value at RAM address `addr`.
    fn execute_ldiaddr(&mut self, addr: Address) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a JP instruction on V0 and `addr`.
    ///
    /// The program counter is set to `addr` plus the value of V0.
    fn execute_jpv0addr(&mut self, addr: Address) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a RND instruction on `x` and byte `byte`.
    ///
    /// Generate a random number in the interval [0, 255], which is then ANDed with the value
    /// `byte`. The results are stored in Vx.
    fn execute_rndvxbyte(&mut self, x: Register, byte: u8) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a DRW instruction on registers `x` and `y` and nibble `byte`.
    ///
    /// Read `byte` bytes from memory, starting at the address stored in I. These bytes are then
    /// displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing
    /// screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// If the sprite is positioned so part of it is outside the coordinates of the display, it wraps
    /// around to the opposite side of the screen.
    fn execute_drwvxvynibble(&mut self, x: Register, y: Register, byte: u8) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a SKP instruction on register `x`.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently
    /// in the down position, the program counter is increased by 2.
    fn execute_skpvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a SKNP instruction on register `x`.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently
    /// in the up position, the program counter is increased by 2.
    fn execute_sknpvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a LD instruction on register `x` from the delay timer.
    ///
    /// The value of the delay timer is placed into Vx.
    fn execute_ldvxdt(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a LD instruction on register `x` from a key press.
    ///
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in Vx.
    fn execute_ldvxk(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a LD instruction on the delay timer and register `x`.
    ///
    /// The delay timer is set equal to the value of Vx.
    fn execute_lddtvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes a LD instruction on the sound timer and register `x`.
    ///
    /// The sound timer is set to the value of Vx.
    fn execute_ldstvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes an ADD instruction on I and `x`.
    ///
    /// The values of I and Vx are added, and the result is stored in I.
    fn execute_addivx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes an sprite LD instruction.
    ///
    /// The value of I is set to the location of the hexadecimal sprite
    /// corresponding to the value of Vx.
    fn execute_ldfvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes an BCD LD instruction.
    ///
    /// Takes the decimal value of Vx and places the hundreds digit in
    /// memory at location I, the tens digit at location I+1, and the
    /// ones digit at location I+2.
    fn execute_ldbvx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes an array LD instruction.
    ///
    /// Copies the values of registers V0 through Vx into memory,
    /// starting at the address in I.
    fn execute_ldivx(&mut self, x: Register) -> Result<(), String> {
        Err("Not yet implemented.".to_string())
    }

    /// Executes an array LD instruction.
    ///
    /// Reads values from memory starting at location I into
    /// registers V0 through Vx.
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
            Opcode::SNEVxByte(x, kk) => self.execute_snevxbyte(x, kk),
            Opcode::SEVxVy(x, y) => self.execute_sevxvy(x, y),
            Opcode::LDVxByte(x, kk) => self.execute_ldvxbyte(x, kk),
            Opcode::ADDVxByte(x, kk) => self.execute_addvxbyte(x, kk),
            Opcode::LDVxVy(x, y) => self.execute_ldvxvy(x, y),
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
            Opcode::LDDTVx(x) => self.execute_lddtvx(x),
            Opcode::LDSTVx(x) => self.execute_ldstvx(x),
            Opcode::ADDIVx(x) => self.execute_addivx(x),
            Opcode::LDFVx(x) => self.execute_ldfvx(x),
            Opcode::LDBVx(x) => self.execute_ldbvx(x),
            Opcode::LDIVx(x) => self.execute_ldivx(x),
            Opcode::LDVxI(x) => self.execute_ldvxi(x),
        }
    }

    /// Returns a mutable reference to the specified register if it exists.
    fn get_register<'a>(&'a mut self, v: Register) -> Result<&'a mut Register, String> {
        match v {
            0 => Ok(&mut self.registers.v0),
            1 => Ok(&mut self.registers.v1),
            2 => Ok(&mut self.registers.v2),
            3 => Ok(&mut self.registers.v3),
            4 => Ok(&mut self.registers.v4),
            5 => Ok(&mut self.registers.v5),
            6 => Ok(&mut self.registers.v6),
            7 => Ok(&mut self.registers.v7),
            8 => Ok(&mut self.registers.v8),
            9 => Ok(&mut self.registers.v9),
            10 => Ok(&mut self.registers.va),
            11 => Ok(&mut self.registers.vb),
            12 => Ok(&mut self.registers.vc),
            13 => Ok(&mut self.registers.vd),
            14 => Ok(&mut self.registers.ve),
            15 => Ok(&mut self.registers.vf),
            _ => {
                let mut msg = String::new();
                write!(msg, "Register {} does not exist.", v).unwrap();
                Err(msg)
            },
        }
    }
}
