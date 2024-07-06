#![warn(clippy::all, clippy::pedantic)]
mod document;
pub use document::Document;

mod row;
pub use row::Row;

mod editor;
use editor::Editor;
pub use editor::Position;

mod terminal;
pub use terminal::Terminal;

fn main() {
    Editor::default().start();
}
