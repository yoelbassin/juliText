use termion::raw::{IntoRawMode, RawTerminal};

use crate::gui::gui;

mod color;
mod cursor;
mod keyboard;
mod screen;

pub struct Termion{
    _cursor: Box<dyn gui::Cursor>,
    _color: Box<dyn gui::Color>,
    _keyboard: Box<dyn gui::Keyboard>,
    _screen: Box<dyn gui::Screen>,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl gui::Gui for Termion {
    fn default() -> Result<Self, std::io::Error> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let cursor = Box::new(cursor::TermionCursor {});
        let color = Box::new(color::TermionColor {});
        let keyboard = Box::new(keyboard::TermionKeyboard {});
        let screen = Box::new(screen::TermionScreen::default().expect("Failed getting terminal size"));
        Ok(Self {
            _cursor: cursor,
            _color: color,
            _keyboard: keyboard,
            _screen: screen,
            _stdout: stdout,
        })
    }
    fn cursor(&self) -> &Box<dyn gui::Cursor> {
        &self._cursor
    }

    fn color(&self) -> &Box<dyn gui::Color> {
        &self._color
    }

    fn screen(&self) -> &Box<dyn gui::Screen> {
        &self._screen
    }

    fn keyboard(&self) -> &Box<dyn gui::Keyboard> {
        &self._keyboard
    }
}
