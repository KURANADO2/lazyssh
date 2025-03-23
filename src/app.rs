use crate::server::ServerList;
use std::time::Instant;

#[derive(Debug)]
pub struct App {
    pub should_exit: bool,
    pub has_selected: bool,
    pub search_query: String,
    pub is_searching: bool,
    pub server_list: ServerList,
    pub last_click_time: Option<Instant>,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            should_exit: false,
            has_selected: false,
            search_query: String::new(),
            is_searching: false,
            server_list: ServerList::from_ssh_config(),
            last_click_time: None,
        })
    }

    pub fn update_search(&mut self) {
        self.server_list.filter_items(&self.search_query);
    }
}
