use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
mod terminal;
use terminal::{Position, Size, Terminal};

const LINE_INDICATOR: &str = ">";
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    fn get_welcome_message() -> Result<(), Error> {
        let welcome_message = format!("{NAME} {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);

        let mut welcome_message_line = format!("{LINE_INDICATOR}{spaces}{welcome_message}");
        welcome_message_line.truncate(width);

        Terminal::print(welcome_message_line)?;
        Ok(())
    }

    fn get_empty_row() -> Result<(), Error> {
        Terminal::print(LINE_INDICATOR)?;
        Ok(())
    }

    fn get_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::get_welcome_message()?;
            } else {
                Self::get_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Bye!\r\n")?;
        } else {
            Self::get_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('c') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    pub fn start(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
}
