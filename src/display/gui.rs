//! The GUI

use super::panel;
use super::pixelgrid;
use super::piston_window as pwindow;
use super::sprite;

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
/// The radius of the boarder
const BORDER_RADIUS: f64 = 3.0;

pub struct Gui {
    chip8_instruction_buffer: Vec<panel::Chip8Instruction>,
    chip8_panel: panel::Panel,
    pxgrid: pixelgrid::PixelGrid,
    ram_panel: panel::Panel,
    stack_panel: panel::Panel,
    window: pwindow::PistonWindow,
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            chip8_instruction_buffer: Vec::<panel::Chip8Instruction>::new(),
            chip8_panel: panel::Panel::new(0, 0, CHIP8_HEIGHT_AFTER_SF, CHIP8_WIDTH_AFTER_SF),
            pxgrid: pixelgrid::PixelGrid::new(CHIP8_HEIGHT_AFTER_SF, CHIP8_WIDTH_AFTER_SF, CHIP8_SCALE_FACTOR),
            ram_panel: panel::Panel::new(0, CHIP8_HEIGHT_AFTER_SF + BORDER_RADIUS as u32, BOTTOM_PANEL_HEIGHT_NPIXELS, BOTTOM_PANEL_WIDTH_NPIXELS),
            stack_panel: panel::Panel::new(CHIP8_WIDTH_AFTER_SF + BORDER_RADIUS as u32, 0, RIGHT_PANEL_HEIGHT_NPIXELS, RIGHT_PANEL_WIDTH_NPIXELS),
            window: pwindow::WindowSettings::new("CHIP-8", [WIDTH_NPIXELS, HEIGHT_NPIXELS]).exit_on_esc(true).build().unwrap(),
        }
    }

    pub fn next(&mut self) -> Option<pwindow::Event> {
        self.window.next()
    }

    /// Buffers a sprite to be drawn at the next chip8 drawing method call.
    ///
    /// Returns true if any part of the given sprite overwrites any sprites currently
    /// in existence.
    pub fn buffer_sprite(&mut self, s: sprite::Sprite) -> bool {
        let overlap = self.pxgrid.add_sprite(&s);
        // TODO: This is super inneficient
        self.chip8_instruction_buffer.push(panel::Chip8Instruction::DrawPixGrid(self.pxgrid.clone()));
        overlap
    }

    /// Clears the whole display
    pub fn clear(&mut self, event: &pwindow::Event) {
        self.window.draw_2d(event, |_context, graphics| {
            pwindow::clear([1.0; 4], graphics);
        });
    }

    /// Adds a clear-chip8 instruction to the instruction buffer.
    ///
    /// The next time draw_chip8() is called, the screen will be cleared.
    pub fn clear_chip8(&mut self) {
        self.chip8_instruction_buffer.push(panel::Chip8Instruction::ClearScreen);
    }

    /// Draws the borders between the panels
    pub fn draw_paneling(&mut self, event: &pwindow::Event) {
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
            pwindow::line(pwindow::color::BLACK, BORDER_RADIUS, vertical_line, context.transform, graphics);
            pwindow::line(pwindow::color::BLACK, BORDER_RADIUS, horizontal_line, context.transform, graphics);
        });
    }

    /// Draw the video game display
    ///
    /// Draws the pixels in this object's internal representation of the game display.
    /// Emulated instructions should change the internal representation, and then this
    /// function should get called once per emulation cycle, or perhaps only whenever
    /// anything has changed in the display.
    pub fn draw_chip8(&mut self, event: &pwindow::Event) {
        self.chip8_panel.draw(panel::Context::Chip8{window: &mut self.window, event: event, instructions: &self.chip8_instruction_buffer});
    }

    /// Draw the RAM around where the program counter is currently.
    ///
    /// Includes disassembly of instructions... if I ever get around to that.
    pub fn draw_ram(&mut self, event: &pwindow::Event, pc: u16, ram: &[u8]) {
        self.ram_panel.draw(panel::Context::Ram{window: &mut self.window, event: event, pc: pc, ram: ram});
    }

    /// Draw the stack, including an indication of where the stack pointer is.
    pub fn draw_stack(&mut self, event: &pwindow::Event, sp: u8, stack: &[u16]) {
        self.stack_panel.draw(panel::Context::Stack{window: &mut self.window, event: event, sp: sp, stack: stack});
    }
}
