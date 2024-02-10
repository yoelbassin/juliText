use termion::raw::{IntoRawMode, RawTerminal};

use crate::gui::gui;

mod color;
mod cursor;
mod keyboard;
mod screen;

pub struct Termion {
    cursor: Box<dyn gui::Cursor>,
    color: Box<dyn gui::Color>,
    keyboard: Box<dyn gui::Keyboard>,
    screen: Box<dyn gui::Screen>,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl gui::Gui for Termion {
    fn default() -> Result<Self, std::io::Error> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let cursor = Box::new(cursor::Termion {});
        let color = Box::new(color::Termion {});
        let keyboard = Box::new(keyboard::Termion {});
        let screen =
            Box::new(screen::Termion::default().expect("Failed getting terminal size"));
        Ok(Self {
            cursor,
            color,
            keyboard,
            screen,
            _stdout: stdout,
        })
    }
    fn cursor(&self) -> &Box<dyn gui::Cursor> {
        &self.cursor
    }

    fn color(&self) -> &Box<dyn gui::Color> {
        &self.color
    }

    fn screen(&self) -> &Box<dyn gui::Screen> {
        &self.screen
    }

    fn keyboard(&self) -> &Box<dyn gui::Keyboard> {
        &self.keyboard
    }
}
