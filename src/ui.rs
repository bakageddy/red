use crate::app::App;
use ratatui::{Frame, widgets::Paragraph};

pub fn ui(app: &App, frame: &mut Frame<'_>) {
    frame.render_widget(Paragraph::new(app.buf.as_str()), frame.size());
}
