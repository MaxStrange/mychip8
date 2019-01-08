use super::piston_window as pwindow;
use super::panel::{self, ArrowDirection, PanelData, Panel};
use super::rusttype;
use self::pwindow::Transformed;
use super::{DrawingContext, Point32};

/// Draw every this many iterations regardless of whether anything has changed on canvas.
/// This is useful to force a redraw periodically.
const DRAW_INTERVAL: usize = 1000;

/// This Panel is the RAM display portion of the GUI window. It shows the RAM around the instruction pointer for debug purposes.
pub struct RamPanel {
    /// The state that is common to each type of Panel (height, width, origin, etc.)
    data: panel::PanelData,
    /// The latest PC we have drawn.
    cached_pc: Option<u16>,
    /// The latest RAM we have drawn.
    cached_ram: Option<Vec<u8>>,
    /// A counter to keep track of forcing drawing.
    draw_ticks: usize,
}

impl panel::Panel for RamPanel {
    fn new(origin: Point32, height: u32, width: u32) -> Self {
        RamPanel {
            data: panel::PanelData::new(origin, height, width),
            cached_pc: None,
            cached_ram: None,
            draw_ticks: 0,
        }
    }

    fn draw(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, args: DrawingContext) {
        let pc = args.pc.expect("RAM Panel's draw() method requires a PC as part of its context, but none was found.");
        let ram = args.ram.expect("RAM Panel's draw() method requires a RAM as part of its context, but none was found.");

        if !self.args_already_cached(pc, &ram) || self.draw_ticks % DRAW_INTERVAL == 0 {
            // Do the drawing, since we haven't done this one yet; then update the cache.
            self.draw_no_cache_check(window, event, pc, &ram);
            self.update_cache(pc, &ram);
        }

        self.draw_ticks = self.draw_ticks.wrapping_add(1);
    }

    fn get_state(&self) -> PanelData {
        self.data.clone()
    }
}

impl RamPanel {
    /// Checks if the given args are already in the cache and returns the result.
    fn args_already_cached(&self, pc: u16, ram: &Vec<u8>) -> bool {
        let cached_pc;
        if let Some(p) = self.cached_pc {
            cached_pc = p;
        } else {
            // If cached_pc is None, we don't match
            return false;
        }

        let cached_ram;
        if let Some(r) = &self.cached_ram {
            cached_ram = r;
        } else {
            // If cached_ram is None, we don't match
            return false;
        }

        let mut rams_are_equal = true;
        for (a, b) in cached_ram.iter().zip(ram) {
            if a != b {
                rams_are_equal = false;
                break;
            }
        }

        cached_pc == pc && rams_are_equal
    }

    /// Updates the cache with the given args.
    fn update_cache(&mut self, pc: u16, ram: &Vec<u8>) {
        self.cached_pc = Some(pc);
        self.cached_ram = Some(ram.clone());
    }

    /// Don't check the cache, just draw the Panel.
    fn draw_no_cache_check(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, pc: u16, ram: &Vec<u8>) {
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
            let topleft = rect.topleft;
            let ramaddr = (pc as usize - (nrows / 2)) + idx;
            window.draw_2d(event, |context, graphics| {
                let text = format!("0x{:02x}", ram[ramaddr]);
                let color = pwindow::color::BLACK;
                let fontsize = 12;
                let transform = context.transform.trans(topleft.x as f64, (topleft.y + rect.height() - 2) as f64);
                if let Err(e) = pwindow::text(color, fontsize, &text, &mut glyphcache, transform, graphics) {
                    println!("Could not draw RAM: {:?}", e);
                }
            });
            if ramaddr == pc as usize {
                let halfway_over = rect.topleft.x + (rect.width() / 2);
                let five_eighths_over = rect.topleft.x + (5 * (rect.width() / 8));
                let halfway_down = rect.topleft.y + 1;
                self.draw_arrow(window, event, ArrowDirection::Left, Point32{x: halfway_over, y: halfway_down});

                window.draw_2d(event, |context, graphics| {
                    let text = format!("PC Loc: 0x{:04x}", pc);
                    let transform = context.transform.trans(five_eighths_over as f64, (topleft.y + rect.height() - 2) as f64);
                    let fontsize = 14;
                    let color = pwindow::color::BLACK;
                    if let Err(e) = pwindow::text(color, fontsize, &text, &mut glyphcache, transform, graphics) {
                        println!("Could not draw PC: {:?}", e);
                    }
                });
            }
        }
    }
}
