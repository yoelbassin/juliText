use std::io::Write;

mod cursor;
mod color;
use crate::gui::gui;

pub struct Termion {
    size: gui::Size,
    cursor: Box<dyn gui::Cursor>,
    color: Box<dyn gui::Color>
}

impl gui::Screen for Termion {
    fn cursor(&self) -> &Box<dyn gui::Cursor> {
        &self.cursor
    }
    fn color(&self) -> &Box<dyn gui::Color> {
        &self.color
    }
    fn size(&self) -> &gui::Size {
        &self.size
    }
    fn clear_screen(&self) {
        print!("{}", termion::clear::All);
    }
    fn flush(&self) -> Result<(), std::io::Error> {
        std::io::stdout().flush()
    }
    fn clear_current_line(&self) {
        print!("{}", termion::clear::CurrentLine);
    }
}

impl Termion {
    pub fn default() -> Result<Self, std::io::Error> {
        let cursor = Box::new(cursor::Termion {});
        let color = Box::new(color::Termion {});
        let size = termion::terminal_size()?;
        Ok(Self {
            size: gui::Size {
                height: size.1.saturating_sub(2),
                width: size.0,
            },
            cursor: cursor,
            color: color
        })
    }
}
