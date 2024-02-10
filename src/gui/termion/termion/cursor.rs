use crate::gui::gui;

pub struct TermionCursor {}

impl gui::Cursor for TermionCursor {
    fn hide(&self) {
        print!("{}", termion::cursor::Hide);
    }

    fn show(&self) {
        print!("{}", termion::cursor::Show);
    }

    fn goto(&self, x: usize, y: usize) {
        let x = x.saturating_add(1) as u16;
        let y = y.saturating_add(1) as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }
}
