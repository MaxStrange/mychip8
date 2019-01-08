//! The GUI

use super::panel::Panel;
use super::chip8panel::Chip8Panel;
use super::rampanel::RamPanel;
use super::stackpanel::StackPanel;
use super::piston_window as pwindow;
use super::sprite;
use super::{Point32, DrawingContext};

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

/// The whole GUI for the Chip8, including several Panels.
pub struct Gui {
    chip8_panel: Chip8Panel,
    ram_panel: RamPanel,
    stack_panel: StackPanel,
    window: pwindow::PistonWindow,
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            chip8_panel: Chip8Panel::new(Point32{x: 0, y: 0}, CHIP8_HEIGHT_AFTER_SF, CHIP8_WIDTH_AFTER_SF),
            ram_panel: RamPanel::new(Point32{x: 0, y: CHIP8_HEIGHT_AFTER_SF + BORDER_RADIUS as u32}, BOTTOM_PANEL_HEIGHT_NPIXELS, BOTTOM_PANEL_WIDTH_NPIXELS),
            stack_panel: StackPanel::new(Point32{x: CHIP8_WIDTH_AFTER_SF + BORDER_RADIUS as u32, y: 0}, RIGHT_PANEL_HEIGHT_NPIXELS, RIGHT_PANEL_WIDTH_NPIXELS),
            window: pwindow::WindowSettings::new("CHIP-8", [WIDTH_NPIXELS, HEIGHT_NPIXELS]).exit_on_esc(true).build().unwrap(),
        }
    }

    pub fn next(&mut self) -> Option<pwindow::Event> {
        self.window.next()
    }

    pub fn clear_chip8(&mut self, event: &pwindow::Event) {
        self.chip8_panel.clear(&mut self.window, event);
    }

    #[allow(dead_code)]
    pub fn clear_ram(&mut self, event: &pwindow::Event) {
        self.ram_panel.clear(&mut self.window, event);
    }

    #[allow(dead_code)]
    pub fn clear_stack(&mut self, event: &pwindow::Event) {
        self.stack_panel.clear(&mut self.window, event);
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

    /// Returns true if any pixel in the given sprite overwrites any pixels of any sprites already in the panel.
    pub fn draw_sprite(&mut self, spr: sprite::Sprite) -> bool {
        // TODO
        false
    }

    /// Draw the video game display
    ///
    /// Draws the pixels in this object's internal representation of the game display.
    /// Emulated instructions should change the internal representation, and then this
    /// function should get called once per emulation cycle, or perhaps only whenever
    /// anything has changed in the display.
    pub fn draw_chip8(&mut self, event: &pwindow::Event) {
        self.chip8_panel.draw(&mut self.window, event, DrawingContext {
            pc: None,
            sp: None,
            ram: None,
            stack: None,
        })
    }

    /// Draw the RAM around where the program counter is currently.
    ///
    /// Includes disassembly of instructions... if I ever get around to that.
    pub fn draw_ram(&mut self, event: &pwindow::Event, pc: u16, ram: &[u8]) {
        self.ram_panel.draw(&mut self.window, event, DrawingContext {
            pc: Some(pc),
            sp: None,
            ram: Some(ram.to_vec()),
            stack: None,
        });
    }

    /// Draw the stack, including an indication of where the stack pointer is.
    pub fn draw_stack(&mut self, event: &pwindow::Event, sp: u8, stack: &[u16]) {
        self.stack_panel.draw(&mut self.window, event, DrawingContext {
            pc: None,
            sp: Some(sp),
            ram: None,
            stack: Some(stack.to_vec()),
        });
    }
}
