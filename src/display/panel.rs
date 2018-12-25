//! Panel that can display stuff

use super::piston_window as pwindow;
use self::pwindow::Transformed;
use super::rusttype;

#[derive(Debug, Clone, Copy)]
/// X, Y values
struct Point(u32, u32);

#[derive(Debug, Clone)]
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

#[allow(dead_code)]
/// Which direction an arrow is pointing
enum ArrowDirection {
    Left,
    Right,
}

/// The different instructions that can be executed as part of drawing stuff in the Chip8 panel.
pub enum Chip8Instruction {
    /// Clear the Chip8 panel of anything the user has drawn.
    ClearScreen,
}

/// The different possible argument combinations that we might pass
/// to a Panel's draw method.
pub enum Context<'a> {
    Chip8{window: &'a mut pwindow::PistonWindow, event: &'a pwindow::Event, instructions: &'a Vec<Chip8Instruction>},
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
            Context::Chip8{window, event, instructions} => self.draw_chip8(window, event, instructions),
            Context::Ram{window, event, pc, ram} => self.draw_ram(window, event, pc, ram),
            Context::Stack{window, event, sp, stack} => self.draw_stack(window, event, sp, stack),
        }
    }

    fn chip8_clear_screen(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event) {
        let rect = [self.origin.0 as f64, self.origin.1 as f64, (self.origin.0 + self.width_npixels) as f64, (self.origin.1 + self.height_npixels) as f64];
        window.draw_2d(event, |context, graphics| {
            pwindow::rectangle(pwindow::color::WHITE, rect, context.transform, graphics);
        });
    }

    fn draw_chip8(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, instructions: &Vec<Chip8Instruction>) {
        for instr in instructions {
            match instr {
                Chip8Instruction::ClearScreen => self.chip8_clear_screen(window, event),
            }
        }
    }

    fn draw_ram(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, pc: u16, ram: &[u8]) {
        // Construct the glyph cache
        let font_data: &[u8] = include_bytes!("../../assets/fonts/roboto/Roboto-Regular.ttf");
        let font: rusttype::Font<'static> = rusttype::Font::from_bytes(font_data).expect("Fatal error: Corrupt font binary?");
        let mut glyphcache = pwindow::glyph_cache::rusttype::GlyphCache::from_font(font, window.factory.clone(), pwindow::TextureSettings::new());

        let nrows = 12;
        let rects = self.draw_rows(window, event, nrows);

        if (pc as usize) < (nrows / 2) {
            return;  // Program counter is in a crazy place
        }
        for (idx, rect) in rects.iter().rev().enumerate() {
            let topleft = rect.0;
            let ramaddr = (pc as usize - (nrows / 2)) + idx;
            window.draw_2d(event, |context, graphics| {
                let text = format!("0x{:02x}", ram[ramaddr]);
                let color = pwindow::color::BLACK;
                let fontsize = 12;
                let transform = context.transform.trans(topleft.0 as f64, (topleft.1 + rect.height() - 2) as f64);
                if let Err(e) = pwindow::text(color, fontsize, &text, &mut glyphcache, transform, graphics) {
                    println!("Could not draw RAM: {:?}", e);
                }
            });
            if ramaddr == pc as usize {
                let halfway_over = (rect.0).0 + (rect.width() / 2);
                let five_eighths_over = (rect.0).0 + (5 * (rect.width() / 8));
                let halfway_down = (rect.0).1 + 1;
                self.draw_arrow(window, event, ArrowDirection::Left, Point(halfway_over, halfway_down));

                window.draw_2d(event, |context, graphics| {
                    let text = format!("PC Loc: 0x{:04x}", pc);
                    let transform = context.transform.trans(five_eighths_over as f64, (topleft.1 + rect.height() - 2) as f64);
                    let fontsize = 14;
                    let color = pwindow::color::BLACK;
                    if let Err(e) = pwindow::text(color, fontsize, &text, &mut glyphcache, transform, graphics) {
                        println!("Could not draw PC: {:?}", e);
                    }
                });
            }
        }
    }

    fn draw_stack(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, sp: u8, stack: &[u16]) {
        // Construct the glyph cache
        let font_data: &[u8] = include_bytes!("../../assets/fonts/roboto/Roboto-Regular.ttf");
        let font: rusttype::Font<'static> = rusttype::Font::from_bytes(font_data).expect("Fatal error: Corrupt font binary?");
        let mut glyphcache = pwindow::glyph_cache::rusttype::GlyphCache::from_font(font, window.factory.clone(), pwindow::TextureSettings::new());

        let rects = self.draw_rows(window, event, stack.len());
        for (idx, rect) in rects.iter().rev().enumerate() {
            let topleft = rect.0;
            window.draw_2d(event, |context, graphics| {
                let text = format!("0x{:04x}", stack[idx]);
                let color = pwindow::color::BLACK;
                let fontsize = 12;
                let transform = context.transform.trans(topleft.0 as f64, (topleft.1 + rect.height() - 2) as f64);
                if let Err(e) = pwindow::text(color, fontsize, &text, &mut glyphcache, transform, graphics) {
                    println!("Could not draw stack: {:?}", e);
                }
            });
            if idx == sp as usize {
                let three_fourths_over = (rect.0).0 + (3 * (rect.width() / 4));
                let halfway_down = (rect.0).1 + 1;
                self.draw_arrow(window, event, ArrowDirection::Left, Point(three_fourths_over, halfway_down));
            }
        }
    }

    /// Draws a small arrow at the given location pointing the given direction.
    fn draw_arrow(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, direction: ArrowDirection, topleft: Point) {
        let arrow_height = 12;
        let arrow_width = 6;
        let arrow_length = 24;
        let line_start = Point(topleft.0, topleft.1 + (arrow_height / 2));
        let line_end = Point(topleft.0 + arrow_length, line_start.1);
        let radius = (arrow_width / 3) as f64;
        let line: [f64; 4] = [line_start.0 as f64, line_start.1 as f64, line_end.0 as f64, line_end.1 as f64];
        window.draw_2d(event, |context, graphics| {
            pwindow::line(pwindow::color::BLACK, radius, line, context.transform, graphics);
            let (upline, downline) = match direction {
                ArrowDirection::Left => {
                    let up = [line_start.0 as f64, line_start.1 as f64, (line_start.0 + arrow_width) as f64, (line_start.1 - (arrow_height / 2)) as f64];
                    let down = [line_start.0 as f64, line_start.1 as f64, (line_start.0 + arrow_width) as f64, (line_start.1 + (arrow_height / 2)) as f64];
                    (up, down)
                },
                ArrowDirection::Right => {
                    let up = [line_end.0 as f64, line_end.1 as f64, (line_end.0 - arrow_width) as f64, (line_end.1 - (arrow_height / 2)) as f64];
                    let down = [line_end.0 as f64, line_end.1 as f64, (line_end.0 - arrow_width) as f64, (line_end.1 + (arrow_height / 2)) as f64];
                    (up, down)
                },
            };
            pwindow::line(pwindow::color::BLACK, radius, upline, context.transform, graphics);
            pwindow::line(pwindow::color::BLACK, radius, downline, context.transform, graphics);
        });
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
            let row_end = Point(row_origin.0 + row_width, row_origin.1 + row_height);
            let rectangle = [row_origin.0 as f64, row_origin.1 as f64, row_end.0 as f64, row_end.1 as f64];
            window.draw_2d(event, |context, graphics| {
                pwindow::rectangle(color, rectangle, context.transform, graphics);
            });
            recs.push(Rectangle(row_origin, row_end));
        }

        recs
    }
}
