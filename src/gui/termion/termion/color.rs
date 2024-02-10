use crate::gui::gui;

pub struct Termion {}

impl gui::Color for Termion {
    fn set_bg_color(&self, color: gui::Rgb) {
        let termion_rgb = termion::color::Rgb(color.red, color.green, color.blue);
        print!("{}", termion::color::Bg(termion_rgb));
    }

    fn reset_bg_color(&self) {
        print!("{}", termion::color::Bg(termion::color::Reset));
    }

    fn set_fg_color(&self, color: gui::Rgb) {
        let termion_rgb = termion::color::Rgb(color.red, color.green, color.blue);
        print!("{}", termion::color::Fg(termion_rgb));
    }

    fn reset_fg_color(&self) {
        print!("{}", termion::color::Fg(termion::color::Reset));
    }
}
