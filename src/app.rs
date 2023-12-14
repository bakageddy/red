use std::{fs::File, io::Read};
pub struct App {
    pub buf: String,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            buf: String::new(),
            quit: false,
        }
    }

    pub fn from_file(f: &mut File) -> Self {
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap_or_else(|_| 0);
        App {
            buf,
            quit: false,
        }
    }
}
