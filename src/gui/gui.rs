pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub enum Key {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Backward Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,
    __IsNotComplete,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub trait Cursor {
    fn hide(&self);
    fn show(&self);
    fn goto(&self, x: usize, y: usize);
}

pub trait Color {
    fn set_bg_color(&self, color: Rgb);
    fn reset_bg_color(&self);
    fn set_fg_color(&self, color: Rgb);
    fn reset_fg_color(&self);
}

pub trait Screen {
    fn size(&self) -> &Size;
    fn clear_screen(&self);
    fn flush(&self) -> Result<(), std::io::Error>;
    fn clear_current_line(&self);
    fn cursor(&self) -> &Box<dyn Cursor>;
    fn color(&self) -> &Box<dyn Color>;
}

pub trait Keyboard {
    fn read_key(&self) -> Result<Key, std::io::Error>;
}

pub trait Gui {
    fn default() -> Result<Self, std::io::Error>
    where
        Self: Sized;
    fn screen(&self) -> &Box<dyn Screen>;
    fn keyboard(&self) -> &Box<dyn Keyboard>;
}
