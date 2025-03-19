use crate::app::App;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, StatefulWidget, Style, Widget};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::widgets::{HighlightSpacing, List, ListItem, Paragraph};

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

pub fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Use j/k to move, Enter to execute SSH login, q to quit.").render(area, buf);
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
        let [list_area] = Layout::vertical([Constraint::Fill(1)]).areas(main_area);

        render_list(self, list_area, buf);
        render_footer(footer_area, buf);
    }
}
