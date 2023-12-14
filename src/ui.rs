use crate::app::App;
use ratatui::{Frame, widgets::{Paragraph, Block, Borders, BorderType, block::Position}, layout::Rect};

pub fn ui(app: &App, frame: &mut Frame<'_>) {
    let block = Block::new().borders(Borders::ALL).border_type(BorderType::Rounded).title("Dinesh").title_position(Position::Top);
    let block_pos = frame.size();
    let text_pos = Rect {
        x: block_pos.x + 2,
        y: block_pos.y + 2,
        width: block_pos.width - 2,
        height: block_pos.height - 2,
    };
    frame.render_widget(block, block_pos);
    frame.render_widget(Paragraph::new(app.buf.as_str()), text_pos);
}
