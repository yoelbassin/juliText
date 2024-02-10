use std::io::Write;

use crate::gui::gui;

pub struct Termion {
    size: gui::Size,
}

impl gui::Screen for Termion {
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
        let size = termion::terminal_size()?;
        Ok(Self {
            size: gui::Size {
                height: size.1.saturating_sub(2),
                width: size.0,
            },
        })
    }
}
