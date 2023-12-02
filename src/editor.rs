use std::io::{self, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

fn handle_err(e: std::io::Error) {
    panic!("{}{}", termion::clear::All, e);
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('c') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!(
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::BlinkingUnderline
        );
        if self.should_quit {
            println!("Bye!\r");
        }
        io::stdout().flush()
    }

    pub fn start(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

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
