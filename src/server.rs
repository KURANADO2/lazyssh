use ratatui::widgets::ListState;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use sublime_fuzzy::best_match;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Default, Serialize)]
pub struct ServerList {
    pub items: Vec<ServerItem>,
    pub filtered_items: Vec<usize>,
    #[serde(skip_serializing)]
    pub state: ListState,
    #[serde(skip_serializing)]
    pub expanded_groups: HashMap<String, bool>,
}

#[derive(Debug, Serialize)]
pub struct ServerItem {
    pub group: String,
    pub is_group: bool,
    pub host: String,
    pub ip: String,
    pub username: String,
    pub port: u32,
    pub private_key: String,
    pub password: Option<String>,
}

#[derive(Default)]
struct SshConfigParser {
    current_group: Option<String>,
    current_is_group: Option<bool>,
    current_host: Option<String>,
    current_ip: Option<String>,
    current_user: Option<String>,
    current_port: u32,
    current_private_key: Option<String>,
    current_password: Option<String>,
    items: Vec<ServerItem>,
}

const OTHER_GROUP: &str = "other";

impl SshConfigParser {
    fn new() -> Self {
        Self::default()
    }

    fn parse_line(&mut self, line: &str) {
        let line = line.trim();
        if line.starts_with("#: Group") {
            self.flush_current_host();
            let group_name = line[8..].trim().to_string();
            self.items.push(ServerItem {
                group: group_name.clone(),
                is_group: true,
                host: String::new(),
                ip: String::new(),
                username: String::new(),
                port: 0,
                private_key: String::new(),
                password: None,
            });
            self.current_group = Some(group_name);
            self.current_is_group = Some(true);
            return;
        }

        if line.starts_with("#: Password") {
            let password = line[11..].trim().to_string();
            self.current_password = Some(password);
            return;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return;
        }

        match parts[0] {
            "Host" => {
                self.flush_current_host();
                if parts[1] != "*" {
                    let host_name = parts[1..].join(" ");
                    self.current_host = Some(host_name);
                    self.current_is_group = Some(false);
                    self.reset_current_values();
                }
            }
            "HostName" => self.current_ip = Some(parts[1].to_string()),
            "User" => self.current_user = Some(parts[1].to_string()),
            "Port" => self.current_port = parts[1].parse().unwrap_or(22),
            "IdentityFile" => self.current_private_key = Some(parts[1].to_string()),
            _ => {}
        }
    }

    fn flush_current_host(&mut self) {
        if let Some(host) = self.current_host.take() {
            self.items.push(ServerItem::new(
                self.current_group.as_deref().unwrap_or(OTHER_GROUP),
                self.current_is_group.unwrap_or(false),
                &host,
                self.current_ip.as_deref().unwrap_or("unknown"),
                self.current_user.as_deref().unwrap_or("jing"),
                self.current_port,
                self.current_private_key.as_deref().unwrap_or("unknown"),
                self.current_password.take(),
            ));
        }
    }

    fn reset_current_values(&mut self) {
        self.current_ip = None;
        self.current_user = None;
        self.current_port = 22;
        self.current_private_key = None;
        self.current_password = None;
    }
}

impl ServerList {
    pub fn from_ssh_config() -> Self {
        let path = dirs::home_dir()
            .map(|p| p.join(".ssh/config"))
            .unwrap_or_else(|| "/dev/null".into());

        let content = fs::read_to_string(path).unwrap_or_default();
        let mut parser = SshConfigParser::new();

        // Parse all lines
        content.lines().for_each(|line| parser.parse_line(line));

        // Flush the last host if exists
        parser.flush_current_host();

        let mut state = ListState::default();
        state.select(Some(0));

        let mut expanded_groups = HashMap::new();
        // Initialize all groups as expanded by default
        for item in &parser.items {
            if item.is_group {
                expanded_groups.insert(item.group.clone(), true);
            }
        }

        let mut result = Self {
            items: parser.items,
            state,
            filtered_items: Vec::new(),
            expanded_groups,
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

    pub fn toggle_group(&mut self) {
        if let Some(selected) = self.selected() {
            if selected.is_group {
                let group_name = selected.group.clone();
                let current_index = self.state.selected().unwrap_or(0);
                if let Some(is_expanded) = self.expanded_groups.get_mut(&group_name) {
                    *is_expanded = !*is_expanded;
                    // Rebuild filtered items without resetting selection
                    self.filtered_items = (0..self.items.len()).collect();
                    // Restore the selection
                    self.state.select(Some(current_index));
                }
            }
        }
    }

    pub fn is_group_expanded(&self, group: &str) -> bool {
        self.expanded_groups.get(group).copied().unwrap_or(true)
    }

    pub fn visible_items(&self) -> Vec<&ServerItem> {
        let mut visible = Vec::new();
        let mut current_group = None;
        let mut current_group_expanded = true;

        for &idx in &self.filtered_items {
            let item = &self.items[idx];
            if item.is_group {
                current_group = Some(&item.group);
                current_group_expanded = self.is_group_expanded(&item.group);
                visible.push(item);
            } else if current_group_expanded {
                visible.push(item);
            }
        }
        visible
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

    pub fn get_index_at_y(&self, y: usize) -> Option<usize> {
        let visible_items = self.visible_items();
        if y < visible_items.len() {
            // Find the corresponding index in the original items list
            let visible_item = &visible_items[y];
            self.items.iter().position(|item| {
                item.group == visible_item.group
                    && item.host == visible_item.host
                    && item.is_group == visible_item.is_group
            })
        } else {
            None
        }
    }

    pub fn selected(&self) -> Option<&ServerItem> {
        let visible_items = self.visible_items();
        self.state
            .selected()
            .and_then(|i| visible_items.get(i))
            .map(|item| {
                // Find the corresponding item in the original list
                self.items
                    .iter()
                    .find(|i| {
                        i.group == item.group && i.host == item.host && i.is_group == item.is_group
                    })
                    .unwrap()
            })
    }

    pub fn max_host_len(&self) -> usize {
        self.items
            .iter()
            .map(|item| item.host.width())
            .max()
            .unwrap_or(0)
    }

    pub fn toggle_all_groups(&mut self) {
        let current_index = self.state.selected().unwrap_or(0);
        let all_expanded = self.expanded_groups.values().all(|&expanded| expanded);

        // Toggle all groups to the opposite state
        for is_expanded in self.expanded_groups.values_mut() {
            *is_expanded = !all_expanded;
        }

        // Rebuild filtered items without resetting selection
        self.filtered_items = (0..self.items.len()).collect();
        // Restore the selection
        self.state.select(Some(current_index));
    }
}

impl ServerItem {
    pub fn new(
        group: &str,
        is_group: bool,
        host: &str,
        ip: &str,
        username: &str,
        port: u32,
        private_key: &str,
        password: Option<String>,
    ) -> Self {
        Self {
            group: group.to_string(),
            is_group,
            host: host.to_string(),
            ip: ip.to_string(),
            username: username.to_string(),
            port,
            private_key: private_key.to_string(),
            password,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.host)
    }

    pub fn to_string_aligned(&self, max_host_len: usize, is_expanded: bool) -> String {
        let host_width = self.host.width();
        let padding = " ".repeat(max_host_len - host_width);
        if self.is_group {
            let arrow = if is_expanded { "▼" } else { "▶" };
            format!("{} {}", arrow, self.group)
        } else if OTHER_GROUP.eq(&self.group) {
            format!("{}{} {}", self.host, padding, self.ip)
        } else {
            format!("  {}{} {}", self.host, padding, self.ip)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::server::ServerList;

    #[test]
    fn test() {
        let list = ServerList::from_ssh_config();
        println!("{}", serde_json::to_string(&list).unwrap());
    }
}
