use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    if key.kind != KeyEventKind::Press {
        return;
    }
    match key.code {
        KeyCode::Char('q') => app.should_exit = true,
        KeyCode::Char('j') | KeyCode::Down => app.server_list.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.server_list.select_previous(),
        KeyCode::Char('g') | KeyCode::Home => app.server_list.select_first(),
        KeyCode::Char('G') | KeyCode::End => app.server_list.select_last(),
        KeyCode::Enter => {
            app.should_exit = true;
            app.has_selected = true;
        }
        _ => {}
    }
}
