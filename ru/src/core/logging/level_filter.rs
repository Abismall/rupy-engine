use log::LevelFilter;
use std::collections::HashMap;

pub struct LogLevelFilterFactory {
    filters: HashMap<&'static str, LevelFilter>,
    default_level: LevelFilter,
}

impl LogLevelFilterFactory {
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
            default_level: LevelFilter::Error,
        }
    }

    pub fn get_filters(self) -> HashMap<&'static str, LevelFilter> {
        self.filters
    }

    pub fn add_filter_with_level(mut self, module: &'static str, level: LevelFilter) -> Self {
        self.filters.insert(module, level);
        self
    }

    pub fn add_filter(mut self, module: &'static str) -> Self {
        self.filters.insert(module, self.default_level);
        self
    }

    pub fn set_default_level(mut self, level: LevelFilter) -> Self {
        self.default_level = level;
        self
    }

    pub fn build(self) -> HashMap<&'static str, LevelFilter> {
        self.filters
    }
}
