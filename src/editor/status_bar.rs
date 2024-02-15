use crate::gui;

use super::Document;

struct MessageBar {
    width: usize,
}

impl MessageBar {
    fn draw(
        &self,
        document: Document,
        line: usize,
        gui: Box<dyn gui::Gui>,
        bg_color: gui::gui::Rgb,
        fg_color: gui::gui::Rgb,
    ) {
        let modified_indicator = if document.is_dirty() {
            "(modified)"
        } else {
            ""
        };
        let filename = if let Some(name) = document.filename {
            name.as_str()
        } else {
            "[No Name]"
        };
        let mut status = format!(
            "{} - {} lines {}",
            filename,
            document.len(),
            modified_indicator
        );

        let line_indicator = format!("{}/{}", line, document.len());

        let len = status.len() + line_indicator.len();
        if self.width > len {
            status.push_str(&" ".repeat(self.width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(self.width);
        gui.color().set_bg_color(bg_color);
        gui.color().set_fg_color(fg_color);
        println!("{status}\r");
        gui.color().reset_bg_color();
        gui.color().reset_fg_color();
    }
}
