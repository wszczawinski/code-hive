use termion::event::Key;

use crate::Document;
use crate::Row;
use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LINE_INDICATOR: &str = ">";

#[derive(Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}

fn handle_err(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to start terminal"),
            document: Document::open(),
            cursor_position: Position { x: 1, y: 0 },
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1);
        let width = size.width.saturating_sub(1);
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('c') => self.should_quit = true,
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn get_welcome_message(&self) {
        let mut welcome_message = format!("Code hive - v{}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("{}{}{}", LINE_INDICATOR, spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_rows(&self) {
        let terminal_height = self.terminal.size().height;
        for terminal_row in 1..terminal_height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if terminal_row == terminal_height / 3 {
                self.get_welcome_message();
            } else {
                println!("{}\r", LINE_INDICATOR)
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::set_cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            println!("Bye!\r");
        } else {
            self.draw_rows();
            Terminal::set_cursor_position(&self.cursor_position)
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    pub fn start(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                handle_err(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                handle_err(error);
            }
        }
    }
}
