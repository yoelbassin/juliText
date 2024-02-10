use crate::Position;
use std::io::Write;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                height: size.1.saturating_sub(2),
                width: size.0,
            },
            _stdout: std::io::stdout().into_raw_mode()?,
        })
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn set_bg_color(color: termion::color::Rgb) {
        print!("{}", termion::color::Bg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", termion::color::Bg(termion::color::Reset));
    }

    pub fn set_fg_color(color: termion::color::Rgb) {
        print!("{}", termion::color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", termion::color::Fg(termion::color::Reset));
    }

    pub fn cursor_position(position: &Position) {
        let x = position.x.saturating_add(1) as u16;
        let y = position.y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn flush() -> Result<(), std::io::Error> {
        std::io::stdout().flush()
    }

    pub fn read_key() -> Result<termion::event::Key, std::io::Error> {
        loop {
            if let Some(key) = std::io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
