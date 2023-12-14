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
}
