use anyhow::Result;
use ratatui::widgets::TableState;

use crate::data::{ network, port_usage::PortUsage, process::ProcessTable };

pub struct App {
    pub items: Vec<PortUsage>,
    pub filter: String,
    pub state: TableState,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut app = Self { items: Vec::new(), filter: String::new(), state: TableState::default() };

        app.refresh()?;

        Ok(app)
    }

    pub fn refresh(&mut self) -> Result<()> {
        let processes = ProcessTable::snapshot();

        self.items = network::scan()?
            .into_iter()
            .map(|listener| PortUsage::resolve(&processes, listener))
            .collect();

        self.reset_selection();

        Ok(())
    }

    pub fn filtered(&self) -> Vec<&PortUsage> {
        filtered_items(&self.items, &self.filter)
    }

    pub fn push_filter_char(&mut self, character: char) {
        self.filter.push(character);
        self.reset_selection();
    }

    pub fn pop_filter_char(&mut self) {
        self.filter.pop();
        self.reset_selection();
    }

    fn reset_selection(&mut self) {
        self.state.select(if self.filtered().is_empty() { None } else { Some(0) });
    }

    pub fn next(&mut self) {
        let len = self.filtered().len();

        if len == 0 {
            return;
        }

        let next = match self.state.selected() {
            Some(i) => (i + 1) % len,
            None => 0,
        };

        self.state.select(Some(next));
    }

    pub fn previous(&mut self) {
        let len = self.filtered().len();

        if len == 0 {
            return;
        }

        let previous = match self.state.selected() {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
        };

        self.state.select(Some(previous));
    }
}

pub fn filtered_items<'a>(items: &'a [PortUsage], filter: &str) -> Vec<&'a PortUsage> {
    if filter.is_empty() {
        return items.iter().collect();
    }

    let needle = filter.to_lowercase();

    items.iter().filter(|usage| usage.matches(&needle)).collect()
}
