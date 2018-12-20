//! The GUI

use super::piston_window as pwindow;

/// Width of the whole GUI in pixels
const WIDTH_NPIXELS: u32 = 640;
/// Height of the whole GUI in pixels
const HEIGHT_NPIXELS: u32 = 480;

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

    /// Draws the boarders between the panels
    pub fn draw_paneling(&mut self, _event: &pwindow::Event) {
        // TODO
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
        // TODO
    }

    /// Draw the stack, including an indication of where the stack pointer is.
    pub fn draw_stack(&mut self, _event: &pwindow::Event, _sp: u8, _stack: &[u16]) {
        // TODO
    }

    /// Draw the contents of the registers.
    pub fn draw_registers(&mut self, _event: &pwindow::Event, _vregs: &[u8], _idxreg: u16) {
        // TODO
    }
}
