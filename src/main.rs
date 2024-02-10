#![warn(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else
)]
mod gui;
use gui::Gui;
use gui::termion::Termion;

mod document;
mod row;
mod editor;

use editor::Position;
use row::Row;

fn main() {
    let termion = Box::new(Termion::default().expect("failes"));
    termion.screen().clear_screen();
    let mut editor = editor::Editor::default(termion);
    editor.run();
}
