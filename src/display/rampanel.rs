use super::piston_window as pwindow;
use super::panel::{self, ArrowDirection, PanelData};
use super::rusttype;
use self::pwindow::Transformed;
use super::{DrawingContext, Point32};

/// This Panel is the RAM display portion of the GUI window. It shows the RAM around the instruction pointer for debug purposes.
pub struct RamPanel {
    /// The state that is common to each type of Panel (height, width, origin, etc.)
    data: panel::PanelData,
}

impl panel::Panel for RamPanel {
    fn new(origin: Point32, height: u32, width: u32) -> Self {
        RamPanel {
            data: panel::PanelData::new(origin, height, width),
        }
    }

    fn draw(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, args: DrawingContext) {
        let pc = args.pc.expect("RAM Panel's draw() method requires a PC as part of its context, but none was found.");
        let ram = args.ram.expect("RAM Panel's draw() method requires a RAM as part of its context, but none was found.");

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

    fn get_state(&self) -> PanelData {
        self.data.clone()
    }
}
