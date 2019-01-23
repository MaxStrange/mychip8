//! Panel that can display stuff

use super::piston_window as pwindow;
use super::{Point32, Rectangle, DrawingContext};

/// Which direction an arrow is pointing
#[allow(dead_code)]
pub enum ArrowDirection {
    Left,
    Right,
}

/// A Panel on the GUI. Each Panel should be responsible for displaying stuff inside of itself.
pub trait Panel {

    /// Create a new instance of a Panel.
    fn new(origin: Point32, height: u32, width: u32) -> Self;

    /// Clear the Panel, setting it back to its default display state.
    fn clear(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event) {
        let state = self.get_state();
        let (origin, width_npixels, height_npixels) = (state.origin, state.width_npixels, state.height_npixels);
        let rect = [origin.x as f64, origin.y as f64, (origin.x + width_npixels) as f64, (origin.y + height_npixels) as f64];
        window.draw_2d(event, |context, graphics| {
            pwindow::rectangle(pwindow::color::WHITE, rect, context.transform, graphics);
        });
    }

    /// Get our state. Mostly useful for the accessing our internal state in default trait method implementations.
    fn get_state(&self) -> PanelData;

    /// Take whatever you need from the provided DrawingContext and draw on this Panel.
    fn draw(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, args: DrawingContext);

    /// Draws a small arrow at the given location pointing the given direction.
    fn draw_arrow(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, direction: ArrowDirection, topleft: Point32) {
        let arrow_height = 12;
        let arrow_width = 6;
        let arrow_length = 24;
        let line_start = Point32{x: topleft.x, y: topleft.y + (arrow_height / 2)};
        let line_end = Point32{x: topleft.x + arrow_length, y: line_start.y};
        let radius = (arrow_width / 3) as f64;
        let line: [f64; 4] = [line_start.x as f64, line_start.y as f64, line_end.x as f64, line_end.y as f64];
        window.draw_2d(event, |context, graphics| {
            pwindow::line(pwindow::color::BLACK, radius, line, context.transform, graphics);
            let (upline, downline) = match direction {
                ArrowDirection::Left => {
                    let up = [line_start.x as f64, line_start.y as f64, (line_start.x + arrow_width) as f64, (line_start.y - (arrow_height / 2)) as f64];
                    let down = [line_start.x as f64, line_start.y as f64, (line_start.x + arrow_width) as f64, (line_start.y + (arrow_height / 2)) as f64];
                    (up, down)
                },
                ArrowDirection::Right => {
                    let up = [line_end.x as f64, line_end.y as f64, (line_end.x - arrow_width) as f64, (line_end.y - (arrow_height / 2)) as f64];
                    let down = [line_end.x as f64, line_end.y as f64, (line_end.x - arrow_width) as f64, (line_end.y + (arrow_height / 2)) as f64];
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
    /// Returns the rectangles that are used.
    fn draw_rows(&mut self, window: &mut pwindow::PistonWindow, event: &pwindow::Event, nrows: usize) -> Vec<Rectangle> {
        let state = self.get_state();
        let (height_npixels, width_npixels, origin) = (state.height_npixels, state.width_npixels, state.origin);
        let mut recs = Vec::<Rectangle>::new();
        let back_color_off = pwindow::color::hex("666699");
        let row_height = height_npixels / (nrows as u32);
        let row_width = width_npixels;

        for row_idx in 0..nrows {
            let color = if row_idx % 2 == 0 { back_color_off } else { pwindow::color::WHITE };
            let row_origin = Point32{x: origin.x, y: origin.y + (row_height * row_idx as u32)};
            let row_end = Point32{x: row_origin.x + row_width, y: row_origin.y + row_height};
            let rectangle = [row_origin.x as f64, row_origin.y as f64, row_end.x as f64, row_end.y as f64];
            window.draw_2d(event, |context, graphics| {
                pwindow::rectangle(color, rectangle, context.transform, graphics);
            });
            recs.push(Rectangle{topleft: row_origin, bottomright: row_end});
        }

        recs
    }
}

/// A Panel is a rectangular portion of the GUI window. This is the data that is required by one.
#[derive(Debug, Clone)]
pub struct PanelData {
    /// Height of the Panel in pixels
    pub height_npixels: u32,
    /// Width of the Panel in pixels
    pub width_npixels: u32,
    /// Top left of the Panel
    pub origin: Point32,
}

impl PanelData {
    pub fn new(origin: Point32, height: u32, width: u32) -> Self {
        PanelData {
            height_npixels: height,
            width_npixels: width,
            origin: origin,
        }
    }
}
