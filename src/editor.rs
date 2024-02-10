use crate::{document::Document, gui::gui::{self, Key}, row::Row, Gui};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const STATUS_BAR_FGCOLOR: gui::Rgb = gui::Rgb {
    red: 63,
    green: 63,
    blue: 63,
};
const STATUS_BAR_BGCOLOR: gui::Rgb = gui::Rgb {
    red: 239,
    green: 239,
    blue: 239,
};

struct StatusMessage {
    text: String,
    time: std::time::Instant,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<String> for StatusMessage {
    fn from(text: String) -> Self {
        Self {
            text: text.to_string(),
            time: std::time::Instant::now(),
        }
    }
}

pub struct Editor {
    should_quit: bool,
    gui: Box<dyn Gui>,
    cursor_position: Position,
    document: Document,
    offset: Position,
    status_message: StatusMessage,
}

impl Editor {
    fn die(&self, e: &std::io::Error) {
        self.gui.screen().clear_screen();
        panic!("{}", e);
    }
    pub fn default(gui: Box<dyn Gui>) -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-s = save | Ctrl-q = quit");
        let document = if args.len() > 1 {
            let doc = Document::open(&args[1]);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("Error opening file: {}", args[1]);
                Document::default()
            }
        } else {
            Document::default()
        };
        Editor {
            should_quit: false,
            gui: gui,
            cursor_position: Position::default(),
            document: document,
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(e) = self.refresh_screen() {
                self.die(&e);
            }
            if self.should_quit {
                break;
            }
            if let Err(e) = self.process_keypress() {
                self.die(&e);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        self.gui.cursor().hide();
        self.gui.screen().clear_screen();
        self.gui.cursor().goto(0, 0);
        if self.should_quit {
            self.gui.screen().clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows(); // Crate
            self.draw_status_bar(); // Crate
            self.draw_message_bar(); // Crate
            self.gui.cursor().goto(
                self.cursor_position.x.saturating_sub(self.offset.x),
                self.cursor_position.y.saturating_sub(self.offset.y),
            );
        }
        self.gui.cursor().show();
        self.gui.screen().flush()
    }

    fn draw_status_bar(&self) {
        let width = self.gui.screen().size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            "(modified)"
        } else {
            ""
        };
        let filename = if let Some(name) = &self.document.filename {
            name.as_str()
        } else {
            "[No Name]"
        };
        let mut status = format!(
            "{} - {} lines {}",
            filename,
            self.document.len(),
            modified_indicator
        );

        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        self.gui.color().set_bg_color(STATUS_BAR_BGCOLOR);
        self.gui.color().set_fg_color(STATUS_BAR_FGCOLOR);
        println!("{status}\r");
        self.gui.color().reset_bg_color();
        self.gui.color().reset_fg_color();
    }

    fn draw_message_bar(&self) {
        self.gui.screen().clear_current_line();
        let message = &self.status_message;
        if std::time::Instant::now() - message.time < std::time::Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.gui.screen().size().width as usize);
            print!("{text}");
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.gui.screen().size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message.truncate(width);
        welcome_message = format!("~{}{}", spaces, welcome_message);
        println!("{}\r", welcome_message); // TODO: use gui print
    }

    fn draw_raw(&self, row: &Row) {
        let width = self.gui.screen().size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.gui.screen().size().height;
        for terminal_row in 0..height {
            self.gui.screen().clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_raw(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
                // println!("JuliText editor -- version {VERSION}\r");
            } else {
                println!("~\r");
            }
        }
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.gui.screen().size().width as usize;
        let height = self.gui.screen().size().height as usize;

        if y < self.offset.y {
            self.offset.y = y;
        } else if y >= self.offset.y + height {
            self.offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < self.offset.x {
            self.offset.x = x;
        } else if x >= self.offset.x + width {
            self.offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }    

    fn move_cursor(&mut self, key: Key) {
        let terminal_height = self.gui.screen().size().height as usize;
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.document.len();
        let width = self.row_width(y);
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Left => {
                if x == 0 && y > 0 {
                    y = y.saturating_sub(1);
                    x = self.document.row(y).unwrap().len();
                } else {
                    x = x.saturating_sub(1);
                }
            }
            Key::Right => {
                if x == width {
                    x = 0;
                    y = y.saturating_add(1);
                } else {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = y.saturating_sub(terminal_height - 1),
            Key::PageDown => y = y.saturating_add(terminal_height - 1),
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        let width = self.row_width(y);
        if x > width {
            x = width;
        }
        if y > height {
            y = height;
        }
        self.cursor_position = Position { x, y };
    }

    fn save(&mut self) {
        if self.document.filename.is_none() {
            let new_name = self.prompt("Save as: ").unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted".to_string());
                return;
            }
            self.document.filename = new_name;
        }
        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully".to_string());
        } else {
            self.status_message = StatusMessage::from("Error saving file".to_string());
        }
    }

    fn row_width(&self, index: usize) -> usize {
        if let Some(row) = self.document.row(index) {
            row.len()
        } else {
            0
        }
    }

    fn prompt(&mut self, prompt: &str) -> Result<Option<String>, std::io::Error> {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            match self.gui.keyboard().read_key()? {
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Esc => {
                    result = String::new();
                    break;
                }
                _ => (),
            }
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = self.gui.keyboard().read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Ctrl('s') => self.save(),
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
            Key::Delete => {
                self.document.delete(&self.cursor_position);
            }
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

}
