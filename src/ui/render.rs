use crate::app::App;
use crate::ui::footer::render_footer;
use crate::ui::main::render_list;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Widget;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
        let [list_area] = Layout::vertical([Constraint::Fill(1)]).areas(main_area);

        render_list(self, list_area, buf);
        render_footer(footer_area, buf);
    }
}
