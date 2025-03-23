use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent, MouseEventKind};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    if app.is_searching {
        match key.code {
            // cancel search
            KeyCode::Esc => {
                app.is_searching = false;
                app.search_query.clear();
                app.server_list.reset_filter();
            }
            // delete char
            KeyCode::Backspace => {
                app.search_query.pop();
                app.update_search();
            }
            // ctrl + J/K navigation
            KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.server_list.select_next()
            }
            KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.server_list.select_previous()
            }
            // input char
            KeyCode::Char(c) => {
                app.search_query.push(c);
                app.update_search();
            }
            // navigation in search results
            KeyCode::Down => app.server_list.select_next(),
            KeyCode::Up => app.server_list.select_previous(),
            // execute ssh login
            KeyCode::Enter => {
                app.is_searching = false;
                app.should_exit = true;
                app.has_selected = true;
            }
            _ => {}
        }
        return;
    }

    match key.code {
        KeyCode::Char('q') => app.should_exit = true,
        KeyCode::Char('j') | KeyCode::Down => app.server_list.select_next(),
        KeyCode::Char('k') | KeyCode::Up => app.server_list.select_previous(),
        KeyCode::Char('g') | KeyCode::Home => app.server_list.select_first(),
        KeyCode::Char('G') | KeyCode::End => app.server_list.select_last(),
        KeyCode::Char('/') | KeyCode::Char('f') => {
            app.is_searching = true;
            app.search_query.clear();
        }
        KeyCode::Enter => {
            app.should_exit = true;
            app.has_selected = true;
        }
        _ => {}
    }
}

pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(_) => {
            // Calculates the list item index corresponding to the clicked location
            if let Some(selected_index) = app.server_list.get_index_at_y(mouse.row as usize) {
                app.server_list.state.select(Some(selected_index));
            }
        }
        _ => {}
    }
}
