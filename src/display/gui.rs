//! The GUI

use super::piston_window as pwindow;

/// Width of the whole GUI in pixels
const WIDTH_NPIXELS: u32 = 640;
/// Height of the whole GUI in pixels
const HEIGHT_NPIXELS: u32 = 480;
/// The pixel scale factor
const CHIP8_SCALE_FACTOR: f64 = 4.0;
/// The width of the Chip-8 display in pixels before applying the scale factor
const CHIP8_WIDTH_BEFORE_SF: u32 = 128;
/// The width of the Chip-8 display in pixels after applying the scale factor
const CHIP8_WIDTH_AFTER_SF: u32 = (CHIP8_WIDTH_BEFORE_SF as f64 * CHIP8_SCALE_FACTOR) as u32;
/// The width of the right GUI panel in pixels
const RIGHT_PANEL_WIDTH_NPIXELS: u32 = WIDTH_NPIXELS - CHIP8_WIDTH_AFTER_SF;
/// The height of the Chip-8 display in pixels before applying the scale factor
const CHIP8_HEIGHT_BEFORE_SF: u32 = 64;
/// The height of the Chip-8 display in pixels after applying the scale factor
const CHIP8_HEIGHT_AFTER_SF: u32 = (CHIP8_HEIGHT_BEFORE_SF as f64 * CHIP8_SCALE_FACTOR) as u32;
/// The height of the bottom panel in pixels
const BOTTOM_PANEL_HEIGHT_NPIXELS: u32 = HEIGHT_NPIXELS - CHIP8_HEIGHT_AFTER_SF;
/// The height of the right panel in pixels
const RIGHT_PANEL_HEIGHT_NPIXELS: u32 = HEIGHT_NPIXELS - BOTTOM_PANEL_HEIGHT_NPIXELS;
/// The width of the bottom panel in pixels
const BOTTOM_PANEL_WIDTH_NPIXELS: u32 = WIDTH_NPIXELS;


pub struct Gui {
    window: pwindow::PistonWindow,
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            window: pwindow::WindowSettings::new("CHIP-8", [WIDTH_NPIXELS, HEIGHT_NPIXELS]).exit_on_esc(true).build().unwrap(),
        }
    }

    pub fn next(&mut self) -> Option<pwindow::Event> {
        self.window.next()
    }

    // Examples of how to draw stuff
    //pub fn draw_red_rectangle(&mut self, event: &pwindow::Event) {
    //    self.window.draw_2d(event, |context, graphics| {
    //        let red = [1.0, 0.0, 0.0, 1.0];
    //        let rectangle = [0.0, 0.0, 100.0, 100.0];
    //        pwindow::clear([1.0; 4], graphics);
    //        pwindow::rectangle(red, rectangle, context.transform, graphics);
    //    });
    //}

    //pub fn draw_blue_rectangle(&mut self, event: &pwindow::Event) {
    //    self.window.draw_2d(event, |context, graphics| {
    //        let blue = [0.0, 0.0, 1.0, 1.0];
    //        let rectangle = [100.0, 100.0, 200.0, 200.0];
    //        pwindow::rectangle(blue, rectangle, context.transform, graphics);
    //    });
    //}

    /// Clears the whole display
    pub fn clear(&mut self, event: &pwindow::Event) {
        self.window.draw_2d(event, |_context, graphics| {
            pwindow::clear([1.0; 4], graphics);
        });
    }

    /// Draws the borders between the panels
    pub fn draw_paneling(&mut self, event: &pwindow::Event) {
        let border_radius = 3.0;

        let vert_x0 = CHIP8_WIDTH_AFTER_SF as f64;
        let vert_y0 = 0.0f64;
        let vert_x1 = vert_x0;
        let vert_y1 = CHIP8_HEIGHT_AFTER_SF as f64;
        let vertical_line = [vert_x0, vert_y0, vert_x1, vert_y1];

        let horz_x0 = 0.0f64;
        let horz_y0 = CHIP8_HEIGHT_AFTER_SF as f64;
        let horz_x1 = BOTTOM_PANEL_WIDTH_NPIXELS as f64;
        let horz_y1 = horz_y0;
        let horizontal_line = [horz_x0, horz_y0, horz_x1, horz_y1];

        self.window.draw_2d(event, |context, graphics| {
            pwindow::line(pwindow::color::BLACK, border_radius, vertical_line, context.transform, graphics);
            pwindow::line(pwindow::color::BLACK, border_radius, horizontal_line, context.transform, graphics);
        });
    }

    /// Draw the video game display
    ///
    /// Draws the pixels in this object's internal representation of the game display.
    /// Emulated instructions should change the internal representation, and then this
    /// function should get called once per emulation cycle, or perhaps only whenever
    /// anything has changed in the display.
    pub fn draw_chip8(&mut self, _event: &pwindow::Event) {
        // TODO
    }

    /// Draw the RAM around where the program counter is currently.
    ///
    /// Includes disassembly of instructions... if I ever get around to that.
    pub fn draw_ram(&mut self, _event: &pwindow::Event, _pc: u16, _ram: &[u8]) {
        // TODO: alternate rectangles for text backgrounds rows
        // TODO: place text in rows
    }

    /// Draw the stack, including an indication of where the stack pointer is.
    pub fn draw_stack(&mut self, _event: &pwindow::Event, _sp: u8, _stack: &[u16]) {
        // TODO: alternate rectangles for text backgrounds rows
        // TODO: place text in rows
    }

    /// Draw the contents of the registers.
    pub fn draw_registers(&mut self, _event: &pwindow::Event, _vregs: &[u8], _idxreg: u16) {
        // TODO
    }
}
