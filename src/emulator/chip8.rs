use super::Address;
use super::opcode::Opcode;
use super::debugiface::{EmulatorCommand, EmulatorResponse};
use super::display::{gui, sprite};
use super::rand::prelude::*;
use super::register::{Register, RegisterArray};
use std::fmt::{self, Write};
use std::sync::mpsc;

/// The length of our RAM in bytes
const MEMORY_LENGTH_NBYTES: usize = 4096;
/// The start address for a program binary
const PROGRAM_START_BYTE_ADDR: u16 = 0x0200;
/// The largest a program can be in bytes
const MAX_PROGRAM_SIZE_NBYTES: usize = MEMORY_LENGTH_NBYTES - (PROGRAM_START_BYTE_ADDR as usize);
/// There are this many addresses in the special stack array at most.
const STACK_SIZE_N_ADDRS: usize = 16;

/// In this module, most functions return an EmuResult, which returns either an error message or the number the PC should be incremented by.
type EmuResult = Result<usize, String>;

/// The Chip 8 emulator
pub struct Chip8 {
    /// Flag used in debugging to deterimine if the thread should exit
    debug_should_exit: bool,
    /// Debug pipe receiving end
    debugrx: mpsc::Receiver<EmulatorCommand>,
    /// Debug pipe sending end
    debugtx: mpsc::Sender<EmulatorResponse>,
    /// Special index register - generally used to store memory addresses
    index: u16,
    /// The RAM:
    /// 0x0000 to 0x01FF is reserved for the interpreter
    /// 0x0200 to 0x0FFF is where the ROM will be loaded
    memory: [u8; MEMORY_LENGTH_NBYTES],
    /// Program counter
    pc: u16,
    /// The Chip-8 has 15 1-byte general purpose registers and one that is used as a carry flag.
    registers:  RegisterArray,
   /// Stack pointer - simply an index into the stack, which is up to 16 addresses
    sp: u8,
    /// The stack is implemented as its own array of 16 16-bit values, rather than just a section of RAM
    stack: [u16; STACK_SIZE_N_ADDRS],
    /// The emulator GUI
    user_interface: gui::Gui,
}

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Registers: {:?}", self.registers)?;
        writeln!(f, "PC: 0x{:2x}", self.pc)?;
        writeln!(f, "I: 0x{:2x}", self.index)?;
        writeln!(f, "SP: 0x{:2x}", self.sp)?;
        writeln!(f, "Stack:")?;
        for (idx, item) in self.stack.iter().enumerate() {
            writeln!(f, "  {}: {:x}", idx, item)?;
        }

        // Examine the memory around the PC
        let low: usize = std::cmp::max(0, ((self.pc as isize) - 10) as usize);
        let high: usize = std::cmp::min(MEMORY_LENGTH_NBYTES - 1, (self.pc as usize) + 10);
        writeln!(f, "Sample of memory around PC: (0x{:2x} to 0x{:2x}):", low, high)?;
        for i in low..=high {
            writeln!(f, "  0x{:2x}: 0x{:x}", i, self.memory[i])?;
        }
        Ok(())
    }
}

impl Chip8 {
    /// Create a new instance of the emulator.
    pub fn new(tx: mpsc::Sender<EmulatorResponse>, rx: mpsc::Receiver<EmulatorCommand>) -> Self {
        Chip8 {
            debug_should_exit: false,
            debugrx: rx,
            debugtx: tx,
            memory: [0u8; MEMORY_LENGTH_NBYTES],
            registers: RegisterArray::new(),
            pc: PROGRAM_START_BYTE_ADDR,
            index: 0,
            sp: 0,
            stack: [0u16; 16],
            user_interface: gui::Gui::new(),
        }
    }

    /// Attempts to load the given binary into RAM and run it.
    pub fn load(&mut self, binary: &Vec<u8>) -> Result<(), String> {
        if binary.len() < MAX_PROGRAM_SIZE_NBYTES {
            let mut index = PROGRAM_START_BYTE_ADDR as usize;
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

    /// Runs the emulator forever (or until a user debugs it and issues the exit command).
    pub fn run(&mut self) {
        while let Some(pistonevent) = self.user_interface.next() {
            // First check if we are being debugged and the user wants to exit
            if self.debug_should_exit {
                break;
            }

            // Draw everything
            self.user_interface.clear_panels(&pistonevent, vec![gui::PanelType::Ram, gui::PanelType::Stack]);
            self.user_interface.draw_chip8(&pistonevent);
            self.user_interface.draw_ram(&pistonevent, self.pc, &self.memory);
            self.user_interface.draw_stack(&pistonevent, self.sp, &self.stack);
            self.user_interface.draw_paneling(&pistonevent);

            // Fetch an instruction with pc
            let msb = self.memory[self.pc as usize];
            let lsb = self.memory[(self.pc + 1) as usize];
            let instruction: u16 = ((msb as u16) << 8) | (lsb as u16);

            // Decode opcode
            let opcode = match Opcode::new(instruction) {
                Ok(o) => o,
                Err(msg) => {
                    panic!("Problem with instruction {:x}: {}. State of us:\n{:?}", instruction, msg, self)
                },
            };

            // Execute instruction and increment the PC
            match self.execute(opcode) {
                Ok(pcincr) => self.pc += pcincr as u16,
                Err(msg) => {
                    panic!("Problem executing instruction {:?}: {}. State of us:\n{:?}", opcode, msg, self)
                },
            }
        }
    }

    /// Stop executing code and instead wait around on self.debugrx, executing debug commands we receive over the pipeline.
    fn execute_brk(&mut self) -> EmuResult {
        // Sit around waiting for debug commands
        while let Ok(cmd) = self.debugrx.recv() {

            // Check the received command
            match cmd {
                // Get the I register and return it
                EmulatorCommand::PeekI => {
                    self.debugtx.send(EmulatorResponse::I(self.index)).unwrap();
                },

                // Get the PC and return it
                EmulatorCommand::PeekPC => {
                    self.debugtx.send(EmulatorResponse::PC(self.pc)).unwrap();
                },

                // Get some bytes and return them
                EmulatorCommand::PeekAddr(addr, nbytes) => {
                    let mut bytes = Vec::<u8>::new();
                    for i in 0..nbytes {
                        bytes.push(self.memory[(addr as usize + i) as usize]);
                    }
                    self.debugtx.send(EmulatorResponse::MemorySlice(bytes)).unwrap();
                },

                // Get the contents of a register
                EmulatorCommand::PeekReg(regidx) => {
                    let regval = match self.get_register(regidx) {
                        Ok(r) => *r,
                        Err(e) => panic!("Could not get requested register {}: {}", regidx, e),
                    };
                    self.debugtx.send(EmulatorResponse::Reg(regval)).unwrap();
                },

                // Send back the SP
                EmulatorCommand::PeekSP => {
                    self.debugtx.send(EmulatorResponse::SP(self.sp)).unwrap();
                },

                // Peek at the whole stack
                EmulatorCommand::PeekStack => {
                    self.debugtx.send(EmulatorResponse::Stack(self.stack.clone().to_vec())).unwrap();
                },

                // Break from the BRK loop
                EmulatorCommand::ResumeExecution => break,

                // Exit the emulator thread
                EmulatorCommand::Exit => { self.debug_should_exit = true; break },
            }
        }
        Ok(2)
    }

    /// Executes a SYS instruction.
    ///
    /// The SYS instruction jumps to a machine code routine at the given address.
    /// This instruction is only used on the old computers on which Chip-8 was originally
    /// implemented. It is ignored by modern interpreters.
    fn execute_sys(&mut self, _addr: Address) -> EmuResult {
        // Does nothing - NOP
        Ok(2)
    }

    /// Executes a CLS instruction.
    ///
    /// Clears the display.
    fn execute_cls(&mut self) -> EmuResult {
        self.user_interface.clear_chip8();

        Ok(2)
    }

    /// Executes a RET instruction.
    ///
    /// Sets the program counter to the address at the top of the stack,
    /// then subtracts one from the stack pointer.
    fn execute_ret(&mut self) -> EmuResult {
        if self.sp as usize >= self.stack.len() {
            let mut errmsg = String::new();
            write!(errmsg, "Stack pointer ({}) is too big.", self.sp).unwrap();
            Err(errmsg)
        } else {
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
            Ok(2)
        }
    }

    /// Executes a JP instruction.
    ///
    /// Sets the program counter to the given address.
    fn execute_jp(&mut self, addr: Address) -> EmuResult {
        if addr as usize >= self.memory.len() {
            let mut errmsg = String::new();
            write!(errmsg, "Address {} is larger than the memory.", addr).unwrap();
            Err(errmsg)
        } else {
            self.pc = addr;
            Ok(0)
        }
    }

    /// Executes a CALL instruction.
    ///
    /// Increments the stack pointer, puts the current program counter on top of the stack,
    /// then sets the program counter to the given address.
    fn execute_call(&mut self, addr: Address) -> EmuResult {
        if (self.sp + 1) as usize >= self.stack.len() {
            let mut errmsg = String::new();
            write!(errmsg, "Stack pointer ({}) is greater than the length of the stack.", self.sp).unwrap();
            Err(errmsg)
        } else {
            self.stack[self.sp as usize] = self.pc;
            self.sp += 1;
            self.pc = addr;
            Ok(0)
        }
    }

    /// Executes an SE instruction on register `x` and byte `byte`.
    ///
    /// If the contents of register Vx equal `byte`, the program counter is incremented
    /// by 2 (in other words, we skip the next instruction).
    fn execute_sevxbyte(&mut self, x: Register, byte: u8) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        if vx == byte {
            Ok(4)
        } else {
            Ok(2)
        }
    }

    /// Executes an SNE instruction on register `x` and byte `byte`.
    ///
    /// If the contents of register Vx do NOT equal `byte`, the program counter is incremented
    /// by 2 (in other words, we skip the next instruction).
    fn execute_snevxbyte(&mut self, x: Register, byte: u8) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        if vx != byte {
            Ok(4)
        } else {
            Ok(2)
        }
    }

    /// Executes an SNE instruction on registers `x` and `y`.
    ///
    /// The values of Vx and Vy are compared and if they are equal, the program counter is
    /// incremented by 2.
    fn execute_sevxvy(&mut self, x: Register, y: Register) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        if vx == vy {
            Ok(4)
        } else {
            Ok(2)
        }
    }

    /// Executes an LD instruction on register `x` and byte `byte`.
    ///
    /// Stores the given byte in register `x`.
    fn execute_ldvxbyte(&mut self, x: Register, byte: u8) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx = byte;

        Ok(2)
    }

    /// Executes an ADD instruction on register `x` and byte `byte`.
    ///
    /// Adds the value `byte` to the contents of register Vx, then stores the result in Vx.
    fn execute_addvxbyte(&mut self, x: Register, byte: u8) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx += byte;

        Ok(2)
    }

    /// Executes aLD instruction on registers `x` and `y`.
    ///
    /// Stores the value of register Vy in register Vx.
    fn execute_ldvxvy(&mut self, x: Register, y: Register) -> EmuResult {
        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx = vy;

        Ok(2)
    }

    /// Executes an OR instruction on registers `x` and `y`.
    ///
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    fn execute_orvxvy(&mut self, x: Register, y: Register) -> EmuResult {
        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx = *vx | vy;

        Ok(2)
    }

    /// Executes an AND instruction on registers `x` and `y`.
    ///
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    fn execute_andvxvy(&mut self, x: Register, y: Register) -> EmuResult {
        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx = *vx & vy;

        Ok(2)
    }

    /// Executes an XOR instruction on registers `x` and `y`.
    ///
    /// Performs a bitwise XOR on the values of Vx and Vy, then stores the result in Vx.
    fn execute_xorvxvy(&mut self, x: Register, y: Register) -> EmuResult {
        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx = *vx ^ vy;

        Ok(2)
    }

    /// Executes an ADD instruction on registers `x` and `y`.
    ///
    /// Adds the value in the register `y` to the value of register `x`, then stores the result in register `x`.
    /// If the result is greater than 255, VF is set to 1, otherwise it is set to 0.
    /// Only the lowest 8 bits are stored in Vx.
    fn execute_addvxvy(&mut self, x: Register, y: Register) -> EmuResult {
        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        let tmp: u16 = (*vx as u16) + (vy as u16);

        *vx = tmp as u8;

        if tmp > 0x00FF {
            self.registers.vf = 1;
        } else {
            self.registers.vf = 0;
        }

        Ok(2)
    }

    /// Executes a SUB instruction on registers `x` and `y`.
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise set it to 0. Then Vy is subtracted from Vx
    /// and the result is stored in Vx.
    fn execute_subvxvy(&mut self, x: Register, y: Register) -> EmuResult {
        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        let gt: bool = *vx > vy;

        if gt {
            *vx = *vx - vy;
            self.registers.vf = 1;
        } else {
            let tmp: i16 = (*vx).wrapping_sub(vy) as i16;  // Wrap around
            *vx = ((tmp ^ 0xFF) + 1) as u8;                // Get the magnitude, rather than 2's complement version
            self.registers.vf = 0;
        }

        Ok(2)
    }

    /// Executes a SHR instruction on register `x`.
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is
    /// bit shifted right by one (in other words, Vx is divided by 2).
    fn execute_shrvx(&mut self, x: Register) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        let lsb_is_one: bool = (*vx & 0x01) == 1;
        *vx = *vx >> 1;
        if lsb_is_one {
            self.registers.vf = 1;
        } else {
            self.registers.vf = 0;
        }

        Ok(2)
    }

    /// Executes a SUBN instruction on registers `x` and `y`.
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy
    /// and the result is stored in Vx.
    fn execute_subnvxvy(&mut self, x: Register, y: Register) -> EmuResult {
        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        let lt: bool = *vx < vy;

        if lt {
            *vx = vy - *vx;
            self.registers.vf = 1;
        } else {
            let tmp: i16 = (vy).wrapping_sub(*vx) as i16;  // Wrap around
            *vx = ((tmp ^ 0xFF) + 1) as u8;                // Get the magnitude, rather than 2's complement version
            self.registers.vf = 0;
        }

        Ok(2)
    }

    /// Executes a SHL instruction on register `x`.
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is shifted
    /// left by one bit (in other words, Vx is multiplied by 2).
    fn execute_shlvx(&mut self, x: Register) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        let msb_is_one: bool = (*vx & 0x80) == 0x80;
        *vx = *vx << 1;
        if msb_is_one {
            self.registers.vf = 1;
        } else {
            self.registers.vf = 0;
        }

        Ok(2)
    }

    /// Executes an SNE instruction on registers `x` and `y`.
    ///
    /// The values of Vx and Vy are compared and if they are NOT equal, the program counter is
    /// incremented by 2.
    fn execute_snevxvy(&mut self, x: Register, y: Register) -> EmuResult {
        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        if vx != vy {
            self.pc += 2;
        }

        Ok(2)
    }

    /// Executes a LD instruction on register I and `addr`.
    ///
    /// The value of regsiter I is set to the value `addr`.
    fn execute_ldiaddr(&mut self, addr: Address) -> EmuResult {
        self.index = addr;
        Ok(2)
    }

    /// Executes a JP instruction on V0 and `addr`.
    ///
    /// The program counter is set to `addr` plus the value of V0.
    fn execute_jpv0addr(&mut self, addr: Address) -> EmuResult {
        let mut msg = String::new();
        if addr as usize > MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} is out of range of the RAM.", addr).unwrap();
            Err(msg)
        } else if (addr + self.registers.v0 as u16) as usize > MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} plus {} (the contents of V0) is out of range of the RAM.", addr, self.registers.v0).unwrap();
            Err(msg)
        } else {
            self.pc = addr + self.registers.v0 as u16;
            Ok(0)
        }
    }

    /// Executes a RND instruction on `x` and byte `byte`.
    ///
    /// Generate a random number in the interval [0, 255], which is then ANDed with the value
    /// `byte`. The results are stored in Vx.
    fn execute_rndvxbyte(&mut self, x: Register, byte: u8) -> EmuResult {
        let mut rng = thread_rng();
        let result = byte & rng.gen_range(0, 256) as u8;

        let vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        *vx = result;

        Ok(2)
    }

    /// Executes a DRW instruction on registers `x` and `y` and nibble `byte`.
    ///
    /// Read `byte` bytes from memory, starting at the address stored in I. These bytes are then
    /// displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing
    /// screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
    /// If the sprite is positioned so part of it is outside the coordinates of the display, it wraps
    /// around to the opposite side of the screen.
    fn execute_drwvxvynibble(&mut self, x: Register, y: Register, byte: u8) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let vy = match self.get_register(y) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let mut msg = String::new();
        if self.index as usize >= MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} is too large for RAM.", self.index).unwrap();
            return Err(msg);
        } else if (self.index + byte as u16) as usize >= MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} plus {} (the number of bytes we need to read) is too large for RAM.", self.index, byte).unwrap();
            return Err(msg);
        }

        let mut combined_sprite = Vec::<u8>::new();
        for i in 0..byte {
            let idx: usize = (self.index + i as u16) as usize;
            let sprite = self.memory[idx];
            combined_sprite.push(sprite);
        }

        let pixsprite = sprite::Sprite::new(&combined_sprite, vx as u32, vy as u32);
        let collision = self.user_interface.buffer_sprite(pixsprite.clone());

        if collision {
            self.registers.vf = if collision { 0x01 } else { 0x00 };
        }

        Ok(2)
    }

    /// Executes a SKP instruction on register `x`.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently
    /// in the down position, the program counter is increased by 2.
    fn execute_skpvx(&mut self, x: Register) -> EmuResult {
        let _vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        // TODO

        Err("Not yet implemented.".to_string())
    }

    /// Executes a SKNP instruction on register `x`.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently
    /// in the up position, the program counter is increased by 2.
    fn execute_sknpvx(&mut self, x: Register) -> EmuResult {
        let _vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        // TODO

        Err("Not yet implemented.".to_string())
    }

    /// Executes a LD instruction on register `x` from the delay timer.
    ///
    /// The value of the delay timer is placed into Vx.
    fn execute_ldvxdt(&mut self, x: Register) -> EmuResult {
        let _vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        // TODO

        Err("Not yet implemented.".to_string())
    }

    /// Executes a LD instruction on register `x` from a key press.
    ///
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in Vx.
    fn execute_ldvxk(&mut self, x: Register) -> EmuResult {
        let _vx = match self.get_register(x) {
            Ok(r) => r,
            Err(msg) => return Err(msg),
        };

        // TODO

        Err("Not yet implemented.".to_string())
    }

    /// Executes a LD instruction on the delay timer and register `x`.
    ///
    /// The delay timer is set equal to the value of Vx.
    fn execute_lddtvx(&mut self, x: Register) -> EmuResult {
        let _vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        // TODO

        Err("Not yet implemented.".to_string())
    }

    /// Executes a LD instruction on the sound timer and register `x`.
    ///
    /// The sound timer is set to the value of Vx.
    fn execute_ldstvx(&mut self, x: Register) -> EmuResult {
        let _vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        // TODO

        Err("Not yet implemented.".to_string())
    }

    /// Executes an ADD instruction on I and `x`.
    ///
    /// The values of I and Vx are added, and the result is stored in I.
    fn execute_addivx(&mut self, x: Register) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        self.index += vx as u16;

        Ok(2)
    }

    /// Executes a sprite LD instruction.
    ///
    /// The value of I is set to the location of the hexadecimal sprite
    /// corresponding to the value of Vx.
    fn execute_ldfvx(&mut self, x: Register) -> EmuResult {
        let _vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        // TODO

        Err("Not yet implemented.".to_string())
    }

    /// Executes a BCD LD instruction.
    ///
    /// Takes the decimal value of Vx and places the hundreds digit in
    /// memory at location I, the tens digit at location I+1, and the
    /// ones digit at location I+2.
    fn execute_ldbvx(&mut self, x: Register) -> EmuResult {
        let vx = match self.get_register(x) {
            Ok(r) => *r,
            Err(msg) => return Err(msg),
        };

        let mut msg = String::new();
        if self.index as usize >= MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} is too large for RAM.", self.index).unwrap();
            return Err(msg);
        } else if (self.index + 2u16) as usize >= MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} plus 2 (where we will store the BCD ones digit) is too large for RAM.", self.index).unwrap();
            return Err(msg);
        }

        let hundreds = vx / 100;
        let vx = vx % 100;

        let tens = vx / 10;
        let vx = vx %10;

        let ones = vx;

        self.memory[self.index as usize] = hundreds;
        self.memory[(self.index + 1) as usize] = tens;
        self.memory[(self.index + 2) as usize] = ones;

        Ok(2)
    }

    /// Executes an array LD instruction for writing.
    ///
    /// Copies the values of registers V0 through Vx into memory,
    /// starting at the address in I.
    fn execute_ldivx(&mut self, regx_index: Register) -> EmuResult {
        let mut msg = String::new();
        if self.index as usize >= MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} is too large for RAM.", self.index).unwrap();
            return Err(msg);
        } else if (self.index + regx_index as u16) as usize >= MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} plus {} (where we will read to) is too large for RAM.", self.index, regx_index).unwrap();
            return Err(msg);
        } else if regx_index as usize >= self.registers.len() {
            write!(msg, "Requested register index {} is too large. We have {} registers.", regx_index, self.registers.len()).unwrap();
            return Err(msg);
        }

        for idx in 0..regx_index {
            let reg = match self.get_register(idx) {
                Ok(r) => r,
                Err(e) => {
                    write!(msg, "Could not get a register corresponding to index {}: {}", idx, e).unwrap();
                    return Err(msg);
                }
            };
            self.memory[(self.index + idx as u16) as usize] = *reg;
        }

        Ok(2)
    }

    /// Executes an array LD instruction for reading.
    ///
    /// Reads values from memory starting at location I into
    /// registers V0 through Vx.
    fn execute_ldvxi(&mut self, regx_index: Register) -> EmuResult {
        let mut msg = String::new();
        if self.index as usize >= MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} is too large for RAM.", self.index).unwrap();
            return Err(msg);
        } else if (self.index + regx_index as u16) as usize >= MEMORY_LENGTH_NBYTES {
            write!(msg, "Address {} plus {} (where we will read to) is too large for RAM.", self.index, regx_index).unwrap();
            return Err(msg);
        } else if regx_index as usize >= self.registers.len() {
            write!(msg, "Requested register index {} is too large. We have {} registers.", regx_index, self.registers.len()).unwrap();
            return Err(msg);
        }

        for idx in 0..regx_index {
            let tmp = self.memory[(self.index + idx as u16) as usize];
            let reg = match self.get_register(idx) {
                Ok(r) => r,
                Err(e) => {
                    write!(msg, "Could not get a register corresponding to index {}: {}", idx, e).unwrap();
                    return Err(msg);
                }
            };
            *reg = tmp;
        }

        Ok(2)
    }

    /// Execute the given instruction and return failure message or success and program counter increment.
    fn execute(&mut self, op: Opcode) -> EmuResult {
        match op {
            Opcode::BRK => self.execute_brk(),
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
    fn get_register<'b>(&'b mut self, v: Register) -> Result<&'b mut Register, String> {
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
