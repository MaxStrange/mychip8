use super::piston_window as pwindow;
use super::panel::{self, ArrowDirection, PanelData, Panel};
use super::rusttype;
use self::pwindow::Transformed;
use super::{DrawingContext, Point32};

/// Draw every this many iterations regardless of whether anything has changed on canvas.
/// This is useful to force a redraw periodically.
const DRAW_INTERVAL: usize = 1000;

/// This Panel is the display window for the stack. It shows the stack to the user for debug purposes.
pub struct StackPanel {
    /// The state that is common to each type of Panel (height, width, origin, etc.)
    data: panel::PanelData,
    /// The most recently drawn SP.
    cached_sp: Option<u8>,
    /// The most recently drawn stack.
    cached_stack: Option<Vec<u16>>,
    /// A counter to keep track of forcing drawing.
    draw_ticks: usize,
}

impl panel::Panel for StackPanel {
    fn new(origin: Point32, height: u32, width: u32) -> Self {
        StackPanel {
            data: panel::PanelData::new(origin, height, width),
            cached_sp: None,
            cached_stack: None,
            draw_ticks: 0,
        }
    }

    fn draw(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, args: DrawingContext) {
        let sp = args.sp.expect("Stack Panel's draw() method requires a SP as part of its context, but none was provided.");
        let stack = args.stack.expect("Stack Panel's draw() method requires a stack as part of its context, but none was provided.");

        if !self.args_already_cached(sp, &stack) || self.draw_ticks % DRAW_INTERVAL == 0 {
            // Do the drawing, since we haven't done this one yet; then update the cache.
            self.draw_no_cache_check(window, event, sp, &stack);
            self.update_cache(sp, &stack);
        }

        self.draw_ticks.wrapping_add(1);
    }

    fn get_state(&self) -> PanelData {
        self.data.clone()
    }
}

impl StackPanel {
    /// Checks if the given args are already in the cache and returns the result.
    fn args_already_cached(&self, sp: u8, stack: &Vec<u16>) -> bool {
        let cached_sp;
        if let Some(s) = self.cached_sp {
            cached_sp = s;
        } else {
            // If cached_sp is None, we don't match
            return false;
        }

        let cached_stack;
        if let Some(s) = &self.cached_stack {
            cached_stack = s;
        } else {
            // If cached_stack is None, we don't match
            return false;
        }

        let mut stacks_are_equal = true;
        for (a, b) in cached_stack.iter().zip(stack) {
            if a != b {
                stacks_are_equal = false;
                break;
            }
        }

        cached_sp == sp && stacks_are_equal
    }

    /// Updates the cache with the given args.
    fn update_cache(&mut self, sp: u8, stack: &Vec<u16>) {
        self.cached_sp = Some(sp);
        self.cached_stack = Some(stack.clone());
    }

    /// Don't check the cache, just draw the Panel.
    fn draw_no_cache_check(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, sp: u8, stack: &Vec<u16>) {
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
}
