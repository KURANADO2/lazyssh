use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Widget};

pub fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Use j/k to move, Enter to execute SSH login, q to quit.").render(area, buf);
}
