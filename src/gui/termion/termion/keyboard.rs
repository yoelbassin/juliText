use termion::{event::Key, input::TermRead};

use crate::gui::gui;

pub struct Termion {}

impl gui::Keyboard for Termion {
    fn read_key(&self) -> Result<gui::Key, std::io::Error> {
        loop {
            if let Some(key) = std::io::stdin().lock().keys().next() {
                let unwrapped_key = key.unwrap();
                let key = match unwrapped_key {
                    Key::Backspace => gui::Key::Backspace,
                    Key::Left => gui::Key::Left,
                    Key::Right => gui::Key::Right,
                    Key::Up => gui::Key::Up,
                    Key::Down => gui::Key::Down,
                    Key::Home => gui::Key::Home,
                    Key::End => gui::Key::End,
                    Key::PageUp => gui::Key::PageUp,
                    Key::PageDown => gui::Key::PageDown,
                    Key::BackTab => gui::Key::BackTab,
                    Key::Delete => gui::Key::Delete,
                    Key::Insert => gui::Key::Insert,
                    Key::F(x) => gui::Key::F(x),
                    Key::Char(x) => gui::Key::Char(x),
                    Key::Alt(x) => gui::Key::Alt(x),
                    Key::Ctrl(x) => gui::Key::Ctrl(x),
                    Key::Null => gui::Key::Null,
                    Key::Esc => gui::Key::Esc,
                    Key::__IsNotComplete => gui::Key::__IsNotComplete,
                };
                return Ok(key);
            }
        }
    }
}
