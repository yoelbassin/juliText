use crate::gui;

struct StatusBar {
    width: usize,
}

impl StatusBar {
    fn draw(
        &self,
        gui: Box<dyn gui::Gui>,
        bg_color: gui::gui::Rgb,
        fg_color: gui::gui::Rgb,
    ) {
        let status_message = format!("Hello test");
        let len = status_message.len();
        let mut status = status_message;
        if self.width > len {
            status.push_str(&" ".repeat(self.width - len));
        }
        status.truncate(self.width);
        gui.screen().color().set_bg_color(bg_color);
        gui.screen().color().set_fg_color(fg_color);
        println!("{status}\r");
        gui.screen().color().reset_bg_color();
        gui.screen().color().reset_fg_color();
    }
}
