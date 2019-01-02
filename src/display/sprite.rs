//! Module to contain a Sprite struct.

/// A graphical sprite.
///
/// Each sprite consists of up to 15 rows of eight pixels each.
#[derive(Debug, Clone)]
pub struct Sprite {
    /// The x-coordinate of the top left pixel of this sprite.
    pub x: u32,
    /// The y-coordinate of the top left pixel of this sprite.
    pub y: u32,
    /// Vector of bytes. Each byte is interpreted as a row of eight pixels.
    pub rows: Vec<u8>,
}

impl std::fmt::Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for byte in self.rows.iter() {
            for i in (0..8).rev() {
                let c = if byte & (1 << i) != 0 { "#" } else { " " };
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Sprite {
    pub fn new(r: &Vec<u8>, locationx: u32, locationy: u32) -> Self {
        Sprite {
            x: locationx,
            y: locationy,
            rows: r.clone(),
        }
    }
}
