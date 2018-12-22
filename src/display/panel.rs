//! Panel that can display stuff

use super::piston_window as pwindow;

#[derive(Debug, Clone, Copy)]
/// X, Y values
struct Point(u32, u32);

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
    }

    fn draw_stack(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, sp: u8, stack: &[u16]) {
        self.draw_rows(window, event, stack.len());
    }

    /// Draws alternating dark/light rectangles on the screen, suitable for
    /// drawing text against in rows.
    fn draw_rows(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, nrows: usize) {
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
        }
    }
}
