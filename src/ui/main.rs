use crate::app::App;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::text::Line;
use ratatui::widgets::{HighlightSpacing, List, ListItem, StatefulWidget};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;

pub fn render_list(app: &mut App, area: Rect, buf: &mut Buffer) {
    let items: Vec<ListItem> = app
        .server_list
        .items
        .iter()
        .map(|server| ListItem::new(Line::styled(server.hostname.clone(), TEXT_FG_COLOR)))
        .collect();

    let list = List::new(items)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol("→ ")
        .highlight_spacing(HighlightSpacing::Always);

    StatefulWidget::render(list, area, buf, &mut app.server_list.state);
}
