use termion::raw::{IntoRawMode, RawTerminal};

use crate::gui::gui;

mod keyboard;
mod screen;

pub struct Termion {
    keyboard: Box<dyn gui::Keyboard>,
    screen: Box<dyn gui::Screen>,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl gui::Gui for Termion {
    fn default() -> Result<Self, std::io::Error> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let keyboard = Box::new(keyboard::Termion {});
        let screen = Box::new(screen::Termion::default().expect("Failed getting terminal size"));
        Ok(Self {
            keyboard,
            screen,
            _stdout: stdout,
        })
    }

    fn screen(&self) -> &Box<dyn gui::Screen> {
        &self.screen
    }

    fn keyboard(&self) -> &Box<dyn gui::Keyboard> {
        &self.keyboard
    }
}
