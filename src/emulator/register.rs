pub type Register = u8;

pub struct RegisterArray {
    v0: Register,
    v1: Register,
    v2: Register,
    v3: Register,
    v4: Register,
    v5: Register,
    v6: Register,
    v7: Register,
    v8: Register,
    v9: Register,
    va: Register,
    vb: Register,
    vc: Register,
    vd: Register,
    ve: Register,
    vf: Register,
}

impl RegisterArray {
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
}
