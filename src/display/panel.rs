//! Panel that can display stuff

use super::piston_window as pwindow;
use super::rusttype;

#[derive(Debug, Clone, Copy)]
/// X, Y values
struct Point(u32, u32);
/// Top-left and bottom-right points
struct Rectangle(Point, Point);

impl Rectangle {
    pub fn height(&self) -> u32 {
        (self.1).1 - (self.0).1
    }

    pub fn width(&self) -> u32 {
        (self.1).0 - (self.0).0
    }
}

/// The different possible argument combinations that we might pass
/// to a Panel's draw method.
pub enum Context<'a> {
    Chip8{window: &'a mut pwindow::PistonWindow, event: &'a pwindow::Event},
    Ram{window: &'a mut pwindow::PistonWindow, event: &'a pwindow::Event, pc: u16, ram: &'a [u8]},
    Stack{window: &'a mut pwindow::PistonWindow, event: &'a pwindow::Event, sp: u8, stack: &'a [u16]},
}

/// A Panel is a rectangular portion of the GUI
/// window.
pub struct Panel {
    /// Height of the Panel in pixels
    height_npixels: u32,
    /// Width of the Panel in pixels
    width_npixels: u32,
    /// Top left of the Panel
    origin: Point,
}

impl Panel {
    pub fn new(originx: u32, originy: u32, height: u32, width: u32) -> Self {
        Panel {
            height_npixels: height,
            width_npixels: width,
            origin: Point(originx, originy),
        }
    }

    pub fn draw(&mut self, context: Context) {
        match context {
            Context::Chip8{window, event} => self.draw_chip8(window, event),
            Context::Ram{window, event, pc, ram} => self.draw_ram(window, event, pc, ram),
            Context::Stack{window, event, sp, stack} => self.draw_stack(window, event, sp, stack),
        }
    }

    fn draw_chip8(&mut self, _window: &mut pwindow::PistonWindow, _event: &pwindow::Event) {
    }

    fn draw_ram(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, pc: u16, ram: &[u8]) {
        let nrows = 12;
        self.draw_rows(window, event, nrows);
    }

    fn draw_stack(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, sp: u8, stack: &[u16]) {
        // Construct the glyph cache
        let font_data: &[u8] = include_bytes!("../../assets/fonts/roboto/Roboto-Regular.ttf");
        let font: rusttype::Font<'static> = rusttype::Font::from_bytes(font_data).expect("Fatal error: Corrupt font binary?");
        let mut glyphcache = pwindow::glyph_cache::rusttype::GlyphCache::from_font(font, window.factory.clone(), pwindow::TextureSettings::new());

        let rects = self.draw_rows(window, event, stack.len());
        for (idx, rect) in rects.iter().enumerate() {
            let topleft = rect.0;
            let bottomright = rect.1;
            window.draw_2d(event, |context, graphics| {
                let text = format!("0x{:x}", stack[idx]);
                let color = pwindow::color::BLACK;
                let fontsize = rect.height() - ((0.25 * rect.height() as f64) as u32);
                pwindow::text(color, fontsize, &text, &mut glyphcache, context.transform, graphics);
            });
        }
    }

    /// Draws alternating dark/light rectangles on the screen, suitable for
    /// drawing text against in rows.
    ///
    /// Returns the rectangles are used.
    fn draw_rows(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, nrows: usize) -> Vec<Rectangle> {
        let mut recs = Vec::<Rectangle>::new();
        let back_color_off = pwindow::color::hex("666699");
        let row_height = self.height_npixels / (nrows as u32);
        let row_width = self.width_npixels;

        for row_idx in 0..nrows {
            let color = if row_idx % 2 == 0 { back_color_off } else { pwindow::color::WHITE };
            let row_origin = Point(self.origin.0, self.origin.1 + (row_height * row_idx as u32));
            let row_end = Point(row_origin.0 + row_width, row_origin.1 + self.height_npixels);
            let rectangle = [row_origin.0 as f64, row_origin.1 as f64, row_end.0 as f64, row_end.1 as f64];
            window.draw_2d(event, |context, graphics| {
                pwindow::rectangle(color, rectangle, context.transform, graphics);
            });
            recs.push(Rectangle(row_origin, row_end));
        }

        recs
    }
}
