use ratatui::widgets::ListState;
use std::fs;

#[derive(Debug)]
pub struct App {
    pub should_exit: bool,
    pub has_selected: bool,
    pub server_list: ServerList,
}

#[derive(Debug, Default)]
pub struct ServerList {
    pub items: Vec<ServerItem>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct ServerItem {
    pub username: String,
    pub hostname: String,
    pub port: u32,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            should_exit: false,
            has_selected: false,
            server_list: ServerList::from_ssh_config(),
        })
    }
}

impl ServerList {
    pub fn from_ssh_config() -> Self {
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

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }

    pub fn selected(&self) -> Option<&ServerItem> {
        self.state.selected().map(|i| &self.items[i])
    }
}

impl ServerItem {
    pub fn new(username: &str, hostname: &str, port: u32) -> Self {
        Self {
            username: username.to_string(),
            hostname: hostname.to_string(),
            port,
        }
    }
}
