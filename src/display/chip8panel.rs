use super::panel::{self, PanelData};
use super::piston_window as pwindow;
use super::{DrawingContext, Point32};
use super::sprite;

/// How often to force refresh of the display.
const DRAW_INTERVAL: usize = 1;
/// The color of sprites (pixels that are on)
const SPRITE_COLOR: &str = "001a00";
/// The color of the Chip8 background (pixels that are off)
const BACKGROUND_COLOR: &str = "e6ffcc";
/// The pixel scale factor
pub const CHIP8_SCALE_FACTOR: f64 = 4.0;
/// The width of the Chip-8 display in pixels before applying the scale factor
pub const CHIP8_WIDTH_BEFORE_SF: u32 = 128;
/// The height of the Chip-8 display in pixels before applying the scale factor
pub const CHIP8_HEIGHT_BEFORE_SF: u32 = 64;

/// Possible pixel colors
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pxcolor {
    /// Sprite color
    Black,
    /// Background color
    White,
}

/// A Pixel is a virtual pixel - a solid black or solid white block at the appropriate scale factor.
#[derive(Debug, Clone)]
struct Pixel {
    /// The x-location of this pixel in the scaled pixel screen
    pub x: u32,
    /// The y-location of this pixel in the scaled pixel screen
    pub y: u32,
    /// The color of this pixel.
    pub value: Pxcolor,
}

impl Pixel {
    pub fn new(prescaled_x: u32, prescaled_y: u32) -> Self {
        Pixel {
            x: (prescaled_x as f64 * CHIP8_SCALE_FACTOR) as u32,
            y: (prescaled_y as f64 * CHIP8_SCALE_FACTOR) as u32,
            value: Pxcolor::White,
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

/// A PixelGrid is simply that: a 2D grid of pixels and associated methods.
///
/// We ignore the scale factors for the pixels and simply treat them as if they are truly pixels.
/// It is the Panel's responsibility for drawing the Pixels correctly based on the scale factors.
#[derive(Debug, Clone)]
struct PixelGrid {
    /// Number of rows of pixels
    pub nrows: u32,
    /// Number of columns of pixels
    pub ncols: u32,
    /// The Pixels contained in this grid.
    pub pixels: Vec<Pixel>,
    /// Has the grid changed since the last time we painted? We will set this to true
    /// internally whenever we know that it has, but it is the Pixel8Panel's responsibility
    /// to set it back to false upon painting.
    pub has_changed: bool,
}

impl PixelGrid {
    pub fn new(nrows: u32, ncols: u32) -> Self {
        // Create the grid from a bunch of pixels
        let mut pixels = Vec::<Pixel>::new();
        for r in 0..nrows {
            for c in 0..ncols {
                pixels.push(Pixel::new(c, r));
            }
        }

        PixelGrid {
            nrows: nrows,
            ncols: ncols,
            pixels: pixels,
            has_changed: true,
        }
    }

    pub fn clear(&mut self) {
        for p in self.pixels.iter_mut() {
            p.value = Pxcolor::White;
        }

        self.has_changed = true;
    }

    /// Adds the given sprite to the grid of pixels. If any part of this sprite overwrites any
    /// part of any other sprite, true is returned.
    pub fn add_sprite(&mut self, s: &sprite::Sprite) -> bool {
        let spritex = s.x % CHIP8_WIDTH_BEFORE_SF;
        let spritey = s.y % CHIP8_HEIGHT_BEFORE_SF;
        let mut collision = false;
        let start = spritey;
        let end = spritey + s.rows.len() as u32;

        // Iterate from the top of the sprite downwards over however many rows the sprite contains
        for (byte, prewrap_y) in s.rows.iter().zip(start..end) {
            let y = prewrap_y % CHIP8_HEIGHT_BEFORE_SF;

            // Each row in the sprite is a byte.
            // Iterate over that byte from left to right.
            for (bitidx, xadd) in (0..8).rev().zip(0..8) {
                // Determine the x coordinate of this bit
                let x = (spritex + xadd as u32) % CHIP8_WIDTH_BEFORE_SF;

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

        self.has_changed = true;
        collision
    }

    /// Get the pixel at the given x and y.
    fn get_pixel_at<'a>(&'a self, x: usize, y: usize) -> &'a Pixel {
        &self.pixels[(y * self.ncols as usize) + x]
    }

    /// Set the value of the given pixel.
    fn set_pixel_at(&mut self, val: Pxcolor, x: usize, y: usize) {
        let idx = (y * self.ncols as usize) + x;
        self.pixels[idx].value = val;
    }
}

/// This Panel is the Chip8 display.
pub struct Chip8Panel {
    /// The state that is common to each type of Panel (height, width, origin, etc.)
    data: panel::PanelData,
    /// The pixels to draw this iteration.
    pixelgrid: PixelGrid,
    /// Counter to keep track of force refresh periodically.
    draw_ticks: usize,
}

impl panel::Panel for Chip8Panel {
    fn new(origin: Point32, height: u32, width: u32) -> Self {
        Chip8Panel {
            data: panel::PanelData::new(origin, height, width),
            pixelgrid: PixelGrid::new(CHIP8_WIDTH_BEFORE_SF, CHIP8_HEIGHT_BEFORE_SF),
            draw_ticks: 0,
        }
    }

    fn clear(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event) {
        self.pixelgrid.clear();
        self.draw(window, event, DrawingContext {
            pc: None,
            sp: None,
            ram: None,
            stack: None,
        });
    }

    fn draw(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, _args: DrawingContext) {
        if self.pixelgrid.has_changed || self.draw_ticks % DRAW_INTERVAL == 0 {
            let spritecolor = pwindow::color::hex(SPRITE_COLOR);
            let backgroundcolor = pwindow::color::hex(BACKGROUND_COLOR);
            let pixwidth = CHIP8_SCALE_FACTOR as u32;

            window.draw_2d(event, |context, graphics| {
                for pixel in self.pixelgrid.pixels.iter() {
                    let xored_color = if pixel.value == Pxcolor::Black { spritecolor } else { backgroundcolor };
                    let x1 = pixel.x as f64;
                    let y1 = pixel.y as f64;
                    let x2 = ((pixel.x + pixwidth) % CHIP8_WIDTH_BEFORE_SF) as f64;
                    let y2 = ((pixel.y + pixwidth) % CHIP8_HEIGHT_BEFORE_SF) as f64;
                    let rect = [x1, y1, x2, y2];
                    pwindow::rectangle(xored_color, rect, context.transform, graphics);
                }
            });
        }

        self.pixelgrid.has_changed = false;
        self.draw_ticks = self.draw_ticks.wrapping_add(1);
    }

    fn get_state(&self) -> PanelData {
        self.data.clone()
    }
}

impl Chip8Panel {
    /// Adds the given Sprite to the display the next time draw() is called. Returns true if any part of
    /// this Sprite overwrites any part of any Sprite already in existence.
    pub fn add_sprite(&mut self, spr: sprite::Sprite) -> bool {
        self.pixelgrid.add_sprite(&spr)
    }
}

//    /// Draws the given grid of pixels. This is pretty slow, so only pass in which pixels have changed on the screen.
//    fn chip8_draw_grid(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, grid: &Vec<pixelgrid::Pixel>) {
//    }
//
