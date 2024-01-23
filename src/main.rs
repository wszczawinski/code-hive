#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;
pub use editor::Position;

mod terminal;
pub use terminal::Terminal;

fn main() {
    Editor::default().start();
}
