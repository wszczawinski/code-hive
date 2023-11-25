use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

fn handle_err(e: std::io::Error) {
    panic!("{}", e);
}

impl Editor {
    pub fn start(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} -> {}\r", c as u8, c);
                        }
                    }
                    Key::Ctrl('c') => break,
                    _ => println!("{:?}\r", key),
                },
                Err(err) => handle_err(err),
            }
        }
    }
    pub fn default() -> Self {
        Editor {}
    }
}
