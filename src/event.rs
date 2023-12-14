use crate::app::App;

use std::time::Duration;

use crossterm::event::{self, Event, KeyEventKind, KeyModifiers, KeyCode};

pub fn update(app: &mut App) -> () {
    if event::poll(Duration::from_millis(16)).unwrap_or(false) {
        if let Ok(Event::Key(e)) = event::read() {
            match e.kind {
                KeyEventKind::Press => {
                    if e.modifiers == KeyModifiers::CONTROL && e.code == KeyCode::Char('c') {
                        app.quit = true;
                    }

                    if e.code == KeyCode::Backspace {
                        app.buf.pop();
                    }
                    
                    if e.code == KeyCode::Enter {
                        app.buf.push('\n');
                    }

                    if let KeyCode::Char(x) = e.code {
                        app.buf.push(x);
                    }
                },
                _ => (),
            }
        }
    }
}
