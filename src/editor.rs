use termion::event::Key;

use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LINE_INDICATOR: &str = ">";

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('c') => self.should_quit = true,
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

    fn draw_rows(&self) {
        let terminal_height = self.terminal.size().height;
        for row in 1..terminal_height {
            Terminal::clear_current_line();
            if row == terminal_height / 2 {
                self.get_welcome_message();
            } else {
                println!("{}\r", LINE_INDICATOR)
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::set_cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Bye!\r");
        } else {
            self.draw_rows();
            Terminal::set_cursor_position(1, 0)
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
