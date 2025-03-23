mod app;
mod event_handler;
mod render;
mod server;
mod ssh_login;

use crate::app::App;
use crate::event_handler::{handle_key, handle_mouse};
use crate::render::render;
use crate::ssh_login::ssh_login;
use color_eyre::Result;
use crossterm::event;
use crossterm::event::Event;
use crossterm::terminal;
use ratatui::DefaultTerminal;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Enable mouse support
    terminal::enable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;

    let terminal = ratatui::init();
    let app = App::new()?;
    let (result, app) = run(app, terminal);
    ratatui::restore();

    // Cleanup
    terminal::disable_raw_mode()?;
    crossterm::execute!(
        std::io::stdout(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;

    if app.has_selected {
        if let Some(server) = app.server_list.selected() {
            ssh_login(&server);
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
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) => handle_key(&mut app, key),
                Event::Mouse(mouse) => handle_mouse(&mut app, mouse),
                _ => {}
            }
        };
    }

    (result, app)
}
