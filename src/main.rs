use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn get_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn handle_err(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} -> {}\r", b, c);
                }
                if b == get_ctrl_byte('c') {
                    break;
                }
            }
            Err(err) => handle_err(err),
        }
    }
}
