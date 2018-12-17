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
    /// 0x5xy0: Skip next instruction if Vx == Vy; compare register Vx to register Vy. If equal, increment PC by 2.
    SEVxVy(u8, u8),
    /// 0x6xkk: Store the value of register Vy in register Vx.
    LDVxByte(u8, u8),
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
    pub fn new(instruction: u16) -> Self {
        match instruction & 0x8000 {
            0x0000 => {
                if instruction == 0x00E0 {
                    Opcode::CLS
                } else if instruction == 0x00EE {
                    Opcode::RET
                } else {
                    Opcode::SYS(instruction & 0x0FFF)
                }
            },
            0x1000 => {
                Opcode::JP(instruction & 0x0FFF)
            },
            0x2000 => {
                Opcode::CALL(instruction & 0x0FFF)
            },
            0x3000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                Opcode::SEVxByte(x, kk)
            },
            0x4000 => {
                panic!("Instruction {:x} is not valid!", instruction);
            },
            0x5000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let y: u8 = ((instruction & 0x00F0) >> 4) as u8;
                Opcode::SEVxVy(x, y)
            },
            0x6000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                Opcode::LDVxByte(x, kk)
            },
            0x7000 => {
                panic!("Instruction {:x} is not valid!", instruction);
            },
            0x8000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let y: u8 = ((instruction & 0x00F0) >> 4) as u8;
                match instruction & 0x000F {
                    0x0001 => Opcode::ORVxVy(x, y),
                    0x0002 => Opcode::ANDVxVy(x, y),
                    0x0003 => Opcode::XORVxVy(x, y),
                    0x0004 => Opcode::ADDVxVy(x, y),
                    0x0005 => Opcode::SUBVxVy(x, y),
                    0x0006 => Opcode::SHRVx(x),
                    0x0007 => Opcode::SUBNVxVy(x, y),
                    0x000E => Opcode::SHLVx(x),
                    _ => panic!("Instruction {:x} is not valid!", instruction),
                }
            },
            0x9000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let y: u8 = ((instruction & 0x00F0) >> 4) as u8;
                Opcode::SNEVxVy(x, y)
            },
            0xA000 => {
                Opcode::LDIAddr(instruction & 0x0FFF)
            },
            0xB000 => {
                Opcode::JPV0Addr(instruction & 0x0FFF)
            },
            0xC000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let kk: u8 = (instruction & 0x00FF) as u8;
                Opcode::RNDVxByte(x, kk)
            },
            0xD000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                let y: u8 = ((instruction & 0x00F0) >> 4) as u8;
                let n: u8 = (instruction & 0x000F) as u8;
                Opcode::DRWVxVyNibble(x, y, n)
            },
            0xE000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                match instruction & 0x00FF {
                    0x009E => Opcode::SKPVx(x),
                    0x00A1 => Opcode::SKNPVx(x),
                    _ => panic!("Instruction {:x} is not valid!", instruction),
                }
            },
            0xF000 => {
                let x: u8 = ((instruction & 0x0F00) >> 8) as u8;
                match instruction & 0x00FF {
                    0x0007 => Opcode::LDVxDT(x),
                    0x000A => Opcode::LDVxK(x),
                    0x0015 => Opcode::LDDTVx(x),
                    0x0018 => Opcode::LDSTVx(x),
                    0x001E => Opcode::ADDIVx(x),
                    0x0029 => Opcode::LDFVx(x),
                    0x0033 => Opcode::LDBVx(x),
                    0x0055 => Opcode::LDIVx(x),
                    0x0065 => Opcode::LDVxI(x),
                    _ => panic!("Instruction {:x} is not valid!", instruction),
                }
            },
            _ => panic!("It should be impossible to even get here..."),
        }
    }
}
