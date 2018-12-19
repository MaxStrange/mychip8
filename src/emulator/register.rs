/// The Chip 8 has 8-bit general purpose registers
pub type Register = u8;

/// The number of registers in this struct.
const NUM_REGISTERS: usize = 16;

/// All the registers in the system.
pub struct RegisterArray {
    /// V0 - a general purpose register
    pub v0: Register,
    /// V1 - a general purpose register
    pub v1: Register,
    /// V2 - a general purpose register
    pub v2: Register,
    /// V3 - a general purpose register
    pub v3: Register,
    /// V4 - a general purpose register
    pub v4: Register,
    /// V5 - a general purpose register
    pub v5: Register,
    /// V6 - a general purpose register
    pub v6: Register,
    /// V7 - a general purpose register
    pub v7: Register,
    /// V8 - a general purpose register
    pub v8: Register,
    /// V9 - a general purpose register
    pub v9: Register,
    /// VA - a general purpose register
    pub va: Register,
    /// VB - a general purpose register
    pub vb: Register,
    /// VC - a general purpose register
    pub vc: Register,
    /// VD - a general purpose register
    pub vd: Register,
    /// VE - a general purpose register
    pub ve: Register,
    /// VF - The only non-general purpose register in this struct. Used for carry bits mostly.
    pub vf: Register,
}

impl RegisterArray {
    /// Initializes all the registers in the system to zero and returns them as a RegisterArray.
    pub fn new() -> Self {
        RegisterArray {
            v0: 0u8,
            v1: 0u8,
            v2: 0u8,
            v3: 0u8,
            v4: 0u8,
            v5: 0u8,
            v6: 0u8,
            v7: 0u8,
            v8: 0u8,
            v9: 0u8,
            va: 0u8,
            vb: 0u8,
            vc: 0u8,
            vd: 0u8,
            ve: 0u8,
            vf: 0u8,
        }
    }

    pub fn len(&self) -> usize {
        NUM_REGISTERS
    }
}
