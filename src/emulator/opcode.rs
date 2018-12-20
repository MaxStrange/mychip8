use std::fmt;

#[derive(Debug, Clone, Copy)]
/// Opcodes
pub enum Opcode {
    /// 0x0nnn: Jump to a machine code routine at nnn.
    SYS(u16),
    /// 0x00E0: Clear the display.
    CLS,
    /// 0x00EE: Return from subroutine.
    RET,
    /// 0x1nnn: Jump to location nnn.
    JP(u16),
    /// 0x2nnn: Call subroutine at nnn.
    CALL(u16),
    /// 0x3xkk: Skip next instruction if Vx == kk; compare register Vx to kk. If equal, increment PC by 2.
    SEVxByte(u8, u8),
    /// 0x4xkk: Skip next instruction if Vx != kk.
    SNEVxByte(u8, u8),
    /// 0x5xy0: Skip next instruction if Vx == Vy; compare register Vx to register Vy. If equal, increment PC by 2.
    SEVxVy(u8, u8),
    /// 0x6xkk: Put kk into register Vx.
    LDVxByte(u8, u8),
    /// 0x7xkk: Add the value kk to the value of regsiter Vx, then store the result in Vx.
    ADDVxByte(u8, u8),
    /// 0x8xy0: Store the value of register Vy in register Vx.
    LDVxVy(u8, u8),
    /// 0x8xy1: Bitwise OR Vx and Vy, then store the result in Vx.
    ORVxVy(u8, u8),
    /// 0x8xy2: Bitwise AND Vx and Vy, then store the result in Vx.
    ANDVxVy(u8, u8),
    /// 0x8xy3: Bitwise XOR Vx and Vy, then store the result in Vx.
    XORVxVy(u8, u8),
    /// 0x8xy4: Add Vx and Vy. Store the result in Vx, store any carry in VF.
    ADDVxVy(u8, u8),
    /// 0x8xy5: Subtract Vy from Vx. Store the result in Vx. If Vx > Vy, set VF to 1, otherwise set it to 0.
    SUBVxVy(u8, u8),
    /// 0x8xy6: Shift Vx right 1 (ignore Vy). Store result in Vx. If 0x01 & Vx is 1 before shift, VF is set to 1, oetherwise 0.
    SHRVx(u8),
    /// 0x8xy7: Set Vx to Vx - Vy. If Vy > Vx, VF is set to 1 oetherwise 0.
    SUBNVxVy(u8, u8),
    /// 0x8xyE: Shift Vx left 1 (ignore Vy). Store result in Vx. If 0x80 & Vx is 1 before shift, VF is set to 1, oetherwise 0.
    SHLVx(u8),
    /// 0x9xy0: Skip next instruction if Vx != Vy. If Vx does not equal Vy, increment PC by 2.
    SNEVxVy(u8, u8),
    /// 0xAnnn: Set I to nnn.
    LDIAddr(u16),
    /// 0xBnnn: Jump to location nnn + V0. The PC is set to nnn plus the value of V0.
    JPV0Addr(u16),
    /// 0xCxkk: Generate a random number between 0 and 255, AND it with the value kk. Store the result in Vx.
    RNDVxByte(u8, u8),
    /// 0xDxyn: Display n-byte sprite starting at memory location I at coordinate (Vx, Vy). Set VF equal to collision.
    DRWVxVyNibble(u8, u8, u8),
    /// 0xEx9E: Skip the next instruction if the key with the value of Vx is pressed.
    SKPVx(u8),
    /// 0xExA1: Skip next instruction if key with the value of Vx is NOT pressed.
    SKNPVx(u8),
    /// 0xFx07: The value of the delay timer is stored in Vx.
    LDVxDT(u8),
    /// 0xFx0A: Stop execution until a key is pressed. Then store the value of that key in Vx.
    LDVxK(u8),
    /// 0xFx15: Set the delay timer to Vx.
    LDDTVx(u8),
    /// 0xFx18: Set the sound timer to Vx.
    LDSTVx(u8),
    /// 0xFx1E: Set I equal to I + Vx.
    ADDIVx(u8),
    /// 0xFx29: Set I equal to the location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
    LDFVx(u8),
    /// 0xF33: Store the binary-coded-decimal representation of Vx in memory locations I, I+1, and I+2.
    /// The hundreds digit is I, tens at I+1, then ones at I+2.
    LDBVx(u8),
    /// 0xFx55: Store registers V0 through Vx in memory starting at location I.
    LDIVx(u8),
    /// 0xFx65: Loads registers V0 through Vx with memory starting at location I.
    LDVxI(u8),
}

impl Opcode {
    pub fn new(instruction: u16) -> Result<Self, String> {
        match instruction & 0xF000 {
            0x0000 => {
                if instruction == 0x00E0 {
                    Ok(Opcode::CLS)
                } else if instruction == 0x00EE {
                    Ok(Opcode::RET)
                } else {
                    Ok(Opcode::SYS(instruction & 0x0FFF))
                }
            },
            0x1000 => {
                Ok(Opcode::JP(instruction & 0x0FFF))
            },
            0x2000 => {
                Ok(Opcode::CALL(instruction & 0x0FFF))
            },
            0x3000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                Ok(Opcode::SEVxByte(x, kk))
            },
            0x4000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                Ok(Opcode::SNEVxByte(x, kk))
            },
            0x5000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let y: u8 = ((instruction & 0x00F0) >> 4) as u8;
                Ok(Opcode::SEVxVy(x, y))
            },
            0x6000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                Ok(Opcode::LDVxByte(x, kk))
            },
            0x7000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                Ok(Opcode::ADDVxByte(x, kk))
            },
            0x8000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let y: u8 = ((instruction & 0x00F0) >> 4) as u8;
                match instruction & 0x000F {
                    0x0000 => Ok(Opcode::LDVxVy(x, y)),
                    0x0001 => Ok(Opcode::ORVxVy(x, y)),
                    0x0002 => Ok(Opcode::ANDVxVy(x, y)),
                    0x0003 => Ok(Opcode::XORVxVy(x, y)),
                    0x0004 => Ok(Opcode::ADDVxVy(x, y)),
                    0x0005 => Ok(Opcode::SUBVxVy(x, y)),
                    0x0006 => Ok(Opcode::SHRVx(x)),
                    0x0007 => Ok(Opcode::SUBNVxVy(x, y)),
                    0x000E => Ok(Opcode::SHLVx(x)),
                    _ => Err("0x8 is a valid opcode but the submask is not.".to_string()),
                }
            },
            0x9000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let y: u8 = ((instruction & 0x00F0) >> 4) as u8;
                Ok(Opcode::SNEVxVy(x, y))
            },
            0xA000 => {
                Ok(Opcode::LDIAddr(instruction & 0x0FFF))
            },
            0xB000 => {
                Ok(Opcode::JPV0Addr(instruction & 0x0FFF))
            },
            0xC000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                Ok(Opcode::RNDVxByte(x, kk))
            },
            0xD000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let y: u8 = ((instruction & 0x00F0) >> 4) as u8;
                let n: u8 = (instruction & 0x000F) as u8;
                Ok(Opcode::DRWVxVyNibble(x, y, n))
            },
            0xE000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                match instruction & 0x00FF {
                    0x009E => Ok(Opcode::SKPVx(x)),
                    0x00A1 => Ok(Opcode::SKNPVx(x)),
                    _ => Err("0xE is a valid opcode, but the submask is not.".to_string()),
                }
            },
            0xF000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                match instruction & 0x00FF {
                    0x0007 => Ok(Opcode::LDVxDT(x)),
                    0x000A => Ok(Opcode::LDVxK(x)),
                    0x0015 => Ok(Opcode::LDDTVx(x)),
                    0x0018 => Ok(Opcode::LDSTVx(x)),
                    0x001E => Ok(Opcode::ADDIVx(x)),
                    0x0029 => Ok(Opcode::LDFVx(x)),
                    0x0033 => Ok(Opcode::LDBVx(x)),
                    0x0055 => Ok(Opcode::LDIVx(x)),
                    0x0065 => Ok(Opcode::LDVxI(x)),
                    _ => Err("0xF is a valid opcode, but the submask is not.".to_string()),
                }
            },
            _ => panic!("It should be impossible to even get here..."),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Op: ")?;

        match self {
            Opcode::SYS(addr) => write!(f, "SYS(0x{:04x})", addr),
            Opcode::CLS => write!(f, "CLS"),
            Opcode::RET => write!(f, "RET"),
            Opcode::JP(addr) => write!(f, "JP(0x{:04x})", addr),
            Opcode::CALL(addr) => write!(f, "CALL(0x{:04x})", addr),
            Opcode::SEVxByte(x, kk) => write!(f, "SEVxByte(V{}, {})", x, kk),
            Opcode::SNEVxByte(x, kk) => write!(f, "SNEVxByte(V{}, {})", x, kk),
            Opcode::SEVxVy(x, y) => write!(f, "SEVxVy(V{}, V{})", x, y),
            Opcode::LDVxByte(x, kk) => write!(f, "LDVxByte(V{}, {})", x, kk),
            Opcode::ADDVxByte(x, kk) => write!(f, "ADDVxByte(V{}, {})", x, kk),
            Opcode::LDVxVy(x, y) => write!(f, "LDVxVy(V{}, V{})", x, y),
            Opcode::ORVxVy(x, y) => write!(f, "ORVxVy(V{}, V{})", x, y),
            Opcode::ANDVxVy(x, y) => write!(f, "ANDVxVy(V{}, V{})", x, y),
            Opcode::XORVxVy(x, y) => write!(f, "XORVxVy(V{}, V{})", x, y),
            Opcode::ADDVxVy(x, y) => write!(f, "ADDVxVy(V{}, V{})", x, y),
            Opcode::SUBVxVy(x, y) => write!(f, "SUBVxVy(V{}, V{})", x, y),
            Opcode::SHRVx(x) => write!(f, "SHRVx(V{})", x),
            Opcode::SUBNVxVy(x, y) => write!(f, "SUBNVxVy(V{}, V{})", x, y),
            Opcode::SHLVx(x) => write!(f, "SHLVx({})", x),
            Opcode::SNEVxVy(x, y) => write!(f, "SNEVxVy(V{}, V{})", x, y),
            Opcode::LDIAddr(addr) => write!(f, "LDIAddr(0x{:04x})", addr),
            Opcode::JPV0Addr(addr) => write!(f, "JPV0Addr(0x{:04x})", addr),
            Opcode::RNDVxByte(x, kk) => write!(f, "RNDVxByte(V{}, 0x{:02x})", x, kk),
            Opcode::DRWVxVyNibble(x, y, n) => write!(f, "DRWVxVyNibble(V{}, V{}, {})", x, y, n),
            Opcode::SKPVx(x) => write!(f, "SKPVx(V{})", x),
            Opcode::SKNPVx(x) => write!(f, "SKNPVx(V{})", x),
            Opcode::LDVxDT(x) => write!(f, "LDVxDT(V{})", x),
            Opcode::LDVxK(x) => write!(f, "LDVxK(V{})", x),
            Opcode::LDDTVx(x) => write!(f, "LDDTVx(V{})", x),
            Opcode::LDSTVx(x) => write!(f, "LDSTVx(V{})", x),
            Opcode::ADDIVx(x) => write!(f, "ADDIVx(V{})", x),
            Opcode::LDFVx(x) => write!(f, "LDFVx(V{})", x),
            Opcode::LDBVx(x) => write!(f, "LDBVx(V{})", x),
            Opcode::LDIVx(x) => write!(f, "LDIVx(V{})", x),
            Opcode::LDVxI(x) => write!(f, "LDVxI(V{})", x),
        }
    }
}
