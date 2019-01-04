//! Module to hold the PixelGrid Struct.

use super::sprite;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pxcolor {
    /// Sprite color
    Black,
    /// Background color
    White,
}

/// A PixelGrid is simply that: a 2D grid of pixels and associated methods.
///
/// We ignore the scale factors for the pixels and simply treat them as if they are truly pixels.
/// It is the Panel's responsibility for drawing the Pixels correctly based on the scale factors.
#[derive(Debug, Clone)]
pub struct PixelGrid {
    /// Number of rows of pixels
    pub nrows: u32,
    /// Number of columns of pixels
    pub ncols: u32,
    /// The Pixels contained in this grid.
    pub pixels: Vec<Pixel>,
}

/// A Pixel is a virtual pixel - a solid black or solid white block at the appropriate scale factor.
#[derive(Debug, Clone)]
pub struct Pixel {
    /// The x-location of this pixel in the scaled pixel screen
    pub x: u32,
    /// The y-location of this pixel in the scaled pixel screen
    pub y: u32,
    /// The number of rows that this pixel occupies
    pub nrows: u32,
    /// The number of columns that this pixel occupies
    pub ncols: u32,
    /// The color of this pixel.
    pub value: Pxcolor,
    /// The scale factor for this pixel - i.e., how big it appears on screen.
    /// A scalefactor of 4.0 would mean that every virtual pixel is actually 4x4 real pixels.
    scalefactor: f64,
}

impl Pixel {
    pub fn new(prescaled_x: u32, prescaled_y: u32, sf: f64) -> Self {
        Pixel {
            x: (prescaled_x as f64 * sf) as u32,
            y: (prescaled_y as f64 * sf) as u32,
            nrows: sf as u32,
            ncols: sf as u32,
            value: Pxcolor::White,
            scalefactor: sf,
        }
    }

    /// Returns:
    ///
    /// ```
    /// a       b       return
    /// BLACK   BLACK   WHITE
    /// BLACK   WHITE   BLACK
    /// WHITE   BLACK   BLACK
    /// WHITE   WHITE   WHITE
    /// ```
    ///
    pub fn xor(a: Pxcolor, b: Pxcolor) -> Pxcolor {
        match a {
            Pxcolor::Black => match b {
                Pxcolor::Black => Pxcolor::White,
                Pxcolor::White => Pxcolor::Black,
            },
            Pxcolor::White => match b {
                Pxcolor::Black => Pxcolor::Black,
                Pxcolor::White => Pxcolor::White,
            },
        }
    }
}

impl PixelGrid {
    pub fn new(nrows: u32, ncols: u32, scalefactor: f64) -> Self {
        // Create the grid from a bunch of pixels
        let mut pixels = Vec::<Pixel>::new();
        for r in 0..nrows {
            for c in 0..ncols {
                pixels.push(Pixel::new(c, r, scalefactor));
            }
        }

        // Construct and return it
        PixelGrid {
            nrows: nrows,
            ncols: ncols,
            pixels: pixels,
        }
    }

    /// Adds the given sprite to the grid of pixels. If any part of this sprite overwrites any
    /// part of any other sprite, true is returned.
    pub fn add_sprite(&mut self, s: &sprite::Sprite) -> bool {
        let mut collision = false;
        let start = s.y;
        let end = s.y + s.rows.len() as u32;

        // Iterate from the top of the sprite downwards over however many rows the sprite contains
        for (byte, y) in s.rows.iter().zip(start..end) {

            // Each row in the sprite is a byte.
            // Iterate over that byte from left to right.
            for (bitidx, xadd) in (0..8).rev().zip(0..8) {
                // Determine the x coordinate of this bit
                let x = s.x + xadd as u32;

                // Check the value of the sprite at that bit
                let bit = if byte & (1 << bitidx) != 0 { 1 } else { 0 };

                // We now have an x and a y for our pixel and whether or not the incoming pixel is occupied
                let incoming_pixel_value = if bit == 1 { Pxcolor::Black } else { Pxcolor::White };
                let our_pixel_value = self.get_pixel_at(x as usize, y as usize).value;

                // Check if incoming pixel value is BLACK and our pixel value is also BLACK. If so, that's a collision.
                if (incoming_pixel_value == our_pixel_value) && (incoming_pixel_value == Pxcolor::Black) {
                    collision = true;
                }

                // XOR the two pixels.
                let xored_value = Pixel::xor(incoming_pixel_value, our_pixel_value);
                self.set_pixel_at(xored_value, x as usize, y as usize);
            }
        }
        collision
    }

    /// Get the pixel at the given x and y
    fn get_pixel_at(&self, x: usize, y: usize) -> Pixel {
        self.pixels[(y * self.ncols as usize) + x].clone()
    }

    /// Set the value of the given pixel
    fn set_pixel_at(&mut self, val: Pxcolor, x: usize, y: usize) {
        self.pixels[(y * self.ncols as usize) + x].value = val;
    }
}
