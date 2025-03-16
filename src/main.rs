use color_eyre::Result;
use ratatui::widgets::ListState;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Color, Modifier, Style},
    text::Line,
    widgets::{HighlightSpacing, List, ListItem, Paragraph, StatefulWidget, Widget},
    DefaultTerminal,
};
use std::{fs, process::Command};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;

#[derive(Debug)]
struct App {
    should_exit: bool,
    selected: bool,
    server_list: ServerList,
}

#[derive(Debug, Default)]
struct ServerList {
    items: Vec<ServerItem>,
    state: ListState,
}

#[derive(Debug)]
struct ServerItem {
    username: String,
    hostname: String,
    port: u32,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new()?;
    let (app_result, app) = app.run(terminal);
    ratatui::restore();

    if app.selected {
        if let Some(server) = app.server_list.selected() {
            ssh_login(&server.username, &server.hostname, server.port);
        }
    }

    app_result
}

fn ssh_login(username: &str, hostname: &str, port: u32) {
    let ssh_cmd = format!("ssh {}@{} -p {}", username, hostname, port);
    println!("Executing: {}", ssh_cmd);

    Command::new("ssh")
        .arg(format!("-p {}", port))
        .arg(format!("{}@{}", username, hostname))
        .spawn()
        .expect("Failed to start SSH session")
        .wait()
        .expect("SSH process failed");
}

impl App {
    fn new() -> Result<Self> {
        let server_list = ServerList::from_ssh_config();
        Ok(Self {
            should_exit: false,
            selected: false,
            server_list,
        })
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> (Result<()>, Self) {
        let mut result = Ok(());
        while !self.should_exit {
            if let Err(e) = terminal.draw(|frame| frame.render_widget(&mut self, frame.area())) {
                result = Err(e.into());
                break;
            }
            if let Ok(Event::Key(key)) = event::read() {
                self.handle_key(key);
            };
        }
        (result, self)
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.should_exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.server_list.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.server_list.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.server_list.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.server_list.select_last(),
            KeyCode::Enter => {
                self.should_exit = true;
                self.selected = true;
            }
            _ => {}
        }
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .server_list
            .items
            .iter()
            .map(|server| ListItem::new(Line::styled(server.hostname.clone(), TEXT_FG_COLOR)))
            .collect();

        let list = List::new(items)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("→ ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.server_list.state);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use j/k to move, Enter to execute SSH login, q to quit.").render(area, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);
        let [list_area] = Layout::vertical([Constraint::Fill(1)]).areas(main_area);

        self.render_list(list_area, buf);
        App::render_footer(footer_area, buf);
    }
}

impl ServerList {
    fn from_ssh_config() -> Self {
        let path = dirs::home_dir()
            .map(|p| p.join(".ssh/config"))
            .unwrap_or_else(|| "/dev/null".into());

        let content = fs::read_to_string(path).unwrap_or_default();
        let mut items = Vec::new();
        let mut user = "root";
        let mut port = 22;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            match parts[0] {
                "Host" => {
                    if parts[1] != "*" {
                        items.push(ServerItem::new(user, parts[1], port));
                    }
                }
                "User" => user = parts[1],
                "Port" => port = parts[1].parse().unwrap_or(22),
                _ => {}
            }
        }
        let mut state = ListState::default();
        state.select(Some(0));
        Self { items, state }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn select_first(&mut self) {
        self.state.select_first();
    }

    fn select_last(&mut self) {
        self.state.select_last();
    }

    fn selected(&self) -> Option<&ServerItem> {
        self.state.selected().map(|i| &self.items[i])
    }
}

impl ServerItem {
    fn new(username: &str, hostname: &str, port: u32) -> Self {
        Self {
            username: username.to_string(),
            hostname: hostname.to_string(),
            port,
        }
    }
}
