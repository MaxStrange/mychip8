use super::piston_window as pwindow;
use super::panel::{self, ArrowDirection, PanelData};
use super::rusttype;
use self::pwindow::Transformed;
use super::{DrawingContext, Point32};

/// This Panel is the display window for the stack. It shows the stack to the user for debug purposes.
pub struct StackPanel {
    /// The state that is common to each type of Panel (height, width, origin, etc.)
    data: panel::PanelData,
}

impl panel::Panel for StackPanel {
    fn new(origin: Point32, height: u32, width: u32) -> Self {
        StackPanel {
            data: panel::PanelData::new(origin, height, width),
        }
    }

    fn draw(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, args: DrawingContext) {
        let sp = args.sp.expect("Stack Panel's draw() method requires a SP as part of its context, but none was provided.");
        let stack = args.stack.expect("Stack Panel's draw() method requires a stack as part of its context, but none was provided.");

        // Construct the glyph cache
        let font_data: &[u8] = include_bytes!("../../assets/fonts/roboto/Roboto-Regular.ttf");
        let font: rusttype::Font<'static> = rusttype::Font::from_bytes(font_data).expect("Fatal error: Corrupt font binary?");
        let mut glyphcache = pwindow::glyph_cache::rusttype::GlyphCache::from_font(font, window.factory.clone(), pwindow::TextureSettings::new());

        let rects = self.draw_rows(window, event, stack.len());
        for (idx, rect) in rects.iter().rev().enumerate() {
            let topleft = rect.topleft;
            window.draw_2d(event, |context, graphics| {
                let text = format!("0x{:04x}", stack[idx]);
                let color = pwindow::color::BLACK;
                let fontsize = 12;
                let transform = context.transform.trans(topleft.x as f64, (topleft.y + rect.height() - 2) as f64);
                if let Err(e) = pwindow::text(color, fontsize, &text, &mut glyphcache, transform, graphics) {
                    println!("Could not draw stack: {:?}", e);
                }
            });
            if idx == sp as usize {
                let three_fourths_over = rect.topleft.x + (3 * (rect.width() / 4));
                let halfway_down = rect.topleft.y + 1;
                self.draw_arrow(window, event, ArrowDirection::Left, Point32{x: three_fourths_over, y: halfway_down});
            }
        }
    }

    fn get_state(&self) -> PanelData {
        self.data.clone()
    }
}
