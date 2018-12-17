/// The Chip 8 emulator
pub struct Chip8 {
}

impl Chip8 {
    /// Create a new instance of the emulator.
    pub fn new() -> Self {
        Chip8 {

        }
    }

    /// Attempts to load the given binary into RAM and run it.
    pub fn load(&mut self, binary: &Vec<u8>) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }

    /// Runs the emulator forever.
    pub fn run(&mut self) -> ! {
        loop {
        }
    }
}
