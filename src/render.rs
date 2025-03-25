use crate::app::App;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::text::Line;
use ratatui::widgets::{HighlightSpacing, List, ListItem, Paragraph};
use ratatui::Frame;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.area());

    // render server list
    let max_host_len = app.server_list.max_host_len();
    let items: Vec<ListItem> = app
        .server_list
        .visible_items()
        .iter()
        .map(|server| ListItem::new(Line::styled(server.to_string_aligned(max_host_len), TEXT_FG_COLOR)))
        .collect();
    let list = List::new(items)
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol("→ ")
        .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(list, chunks[0], &mut app.server_list.state);

    // render footer
    let footer_text = if app.is_searching {
        format!(
            "Search: {} (Press Esc to cancel, Ctrl+j/k or ↑/↓ to navigate)",
            app.search_query
        )
    } else {
        "j/↓: down | k/↑: up | g/Home: top | G/End: bottom | / or f: search | Enter: login"
            .to_string()
    };
    let footer = Paragraph::new(footer_text);

    frame.render_widget(footer, chunks[1]);
}
