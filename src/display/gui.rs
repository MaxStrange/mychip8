//! The GUI

use super::piston_window as pwindow;

pub struct Gui {
}

impl Gui {
    pub fn new() -> Self {
        Gui {

        }
    }

    pub fn new_window(&mut self) -> pwindow::PistonWindow {
        let window: pwindow::PistonWindow = pwindow::WindowSettings::new("CHIP-8", [640, 480])
                            .exit_on_esc(true)
                            .build()
                            .unwrap();
        window
    }

    pub fn draw_red_rectangle(&self, window: &mut pwindow::PistonWindow, event: &pwindow::Event) {
        window.draw_2d(event, |context, graphics| {
            let red = [1.0, 0.0, 0.0, 1.0];
            let rectangle = [0.0, 0.0, 100.0, 100.0];
            pwindow::clear([1.0; 4], graphics);
            pwindow::rectangle(red, rectangle, context.transform, graphics);
        });
    }

    pub fn draw_blue_rectangle(&self, window: &mut pwindow::PistonWindow, event: &pwindow::Event) {
        window.draw_2d(event, |context, graphics| {
            let blue = [0.0, 0.0, 1.0, 1.0];
            let rectangle = [100.0, 100.0, 200.0, 200.0];
            pwindow::rectangle(blue, rectangle, context.transform, graphics);
        });
    }
}
