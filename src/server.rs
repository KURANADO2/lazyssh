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
    pub host: String,
    pub ip: String,
    pub username: String,
    pub port: u32,
    pub private_key: String,
}

impl ServerList {
    pub fn from_ssh_config() -> Self {
        let path = dirs::home_dir()
            .map(|p| p.join(".ssh/config"))
            .unwrap_or_else(|| "/dev/null".into());

        let content = fs::read_to_string(path).unwrap_or_default();
        let mut items = Vec::new();
        let mut current_host = None;
        let mut current_ip = None;
        let mut current_user = None;
        let mut current_port = 22;
        let mut current_private_key = None;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            match parts[0] {
                "Host" => {
                    if let Some(host) = current_host {
                        items.push(ServerItem::new(
                            host,
                            current_ip.unwrap_or("Unknown ip"),
                            current_user.unwrap_or("Unknown user"),
                            current_port,
                            current_private_key.unwrap_or("Unknown private key"),
                        ));
                    }
                    if parts[1] != "*" {
                        current_host = Some(parts[1]);
                        current_ip = None;
                        current_user = None;
                        current_port = 22;
                        current_private_key = None;
                    } else {
                        current_host = None;
                    }
                }
                "HostName" => current_ip = Some(parts[1]),
                "User" => current_user = Some(parts[1]),
                "Port" => current_port = parts[1].parse().unwrap_or(22),
                "IdentityFile" => current_private_key = Some(parts[1]),
                _ => {}
            }
        }

        // Add the last server if exists
        if let Some(host) = current_host {
            items.push(ServerItem::new(
                host,
                current_ip.unwrap_or("unknown"),
                current_user.unwrap_or("jing"),
                current_port,
                current_private_key.unwrap_or("unknown"),
            ));
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
            .filter_map(|(idx, item)| {
                best_match(query, &item.to_string()).map(|m| (idx, m.score()))
            })
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
        self.state
            .selected()
            .and_then(|i| self.filtered_items.get(i))
            .map(|&idx| &self.items[idx])
    }

    pub fn get_index_at_y(&self, y: usize) -> Option<usize> {
        if y < self.filtered_items.len() {
            Some(y)
        } else {
            None
        }
    }
}

impl ServerItem {
    pub fn new(host: &str, ip: &str, username: &str, port: u32, private_key: &str) -> Self {
        Self {
            host: host.to_string(),
            ip: ip.to_string(),
            username: username.to_string(),
            port,
            private_key: private_key.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.host, self.ip)
    }
}
