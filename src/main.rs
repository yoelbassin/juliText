#![warn(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
mod gui;
use gui::termion::Termion;
use gui::Gui;
mod editor;
// pub mod document;
// mod row;

// use editor::Position;
// use row::Row;

fn main() {
    let termion = Box::new(Termion::default().expect("failes"));
    termion.screen().clear_screen();
    // let mut editor = editorm::Editor::default(termion);
    // editor.run();
}
