mod app;
mod event_handler;
mod render;
mod ssh_login;
mod server;

use crate::app::App;
use crate::event_handler::handle_key;
use crate::render::render;
use crate::ssh_login::ssh_login;
use color_eyre::Result;
use crossterm::event;
use crossterm::event::Event;
use ratatui::DefaultTerminal;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new()?;

    let (result, app) = run(app, terminal);

    ratatui::restore();

    if app.has_selected {
        if let Some(server) = app.server_list.selected() {
            ssh_login(&server.username, &server.hostname, server.port);
        }
    }

    result
}

fn run(mut app: App, mut terminal: DefaultTerminal) -> (Result<()>, App) {
    let mut result = Ok(());

    while !app.should_exit {
        if let Err(e) = terminal.draw(|frame| render(frame, &mut app)) {
            result = Err(e.into());
            break;
        }
        if let Ok(Event::Key(key)) = event::read() {
            handle_key(&mut app, key);
        };
    }

    (result, app)
}
