use ratatui::widgets::ListState;
use std::fs;
use sublime_fuzzy::best_match;

#[derive(Debug, Default)]
pub struct ServerList {
    pub items: Vec<ServerItem>,
    pub filtered_items: Vec<usize>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct ServerItem {
    pub username: String,
    pub hostname: String,
    pub port: u32,
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

        let mut result = Self {
            items,
            state,
            filtered_items: Vec::new(),
        };

        result.reset_filter();

        result
    }

    pub fn filter_items(&mut self, query: &str) {
        if query.is_empty() {
            self.reset_filter();
            return;
        }

        self.filtered_items = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| best_match(query, &item.hostname).map(|m| (idx, m.score())))
            .filter(|(_, score)| *score > 0)
            .map(|(idx, _)| idx)
            .collect();

        if !self.filtered_items.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn reset_filter(&mut self) {
        self.filtered_items = (0..self.items.len()).collect();
        self.state.select(Some(0));
    }

    pub fn visible_items(&self) -> Vec<&ServerItem> {
        self.filtered_items
            .iter()
            .map(|&idx| &self.items[idx])
            .collect()
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
