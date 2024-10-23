use log::LevelFilter;
use std::collections::HashMap;

/// `LogLevelFilterFactory` is a builder for creating log level filters for specific modules.
/// It allows setting a default log level and adding custom filters for individual modules.
pub struct LogLevelFilterFactory {
    filters: HashMap<&'static str, LevelFilter>,
    default_level: LevelFilter,
}

impl LogLevelFilterFactory {
    /// Creates a new instance of `LogLevelFilterFactory` with an initial default log level of `LevelFilter::Error`.
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
            default_level: LevelFilter::Error,
        }
    }

    pub fn get_filters(self) -> HashMap<&'static str, LevelFilter> {
        self.filters
    }

    /// Adds a filter for a specific module with a custom log level.
    ///
    /// # Arguments
    /// * `module` - The name of the module for which the log level filter will apply.
    /// * `level` - The log level to set for the specified module.
    pub fn add_filter_with_level(mut self, module: &'static str, level: LevelFilter) -> Self {
        self.filters.insert(module, level);
        self
    }

    /// Adds a filter for a specific module with the current default log level.
    ///
    /// # Arguments
    /// * `module` - The name of the module for which the log level filter will apply.
    pub fn add_filter(mut self, module: &'static str) -> Self {
        self.filters.insert(module, self.default_level);
        self
    }

    /// Sets a custom default log level that will be used for future filters added without a specified log level.
    ///
    /// # Arguments
    /// * `level` - The log level to use as the default for future module filters.
    pub fn set_default_level(mut self, level: LevelFilter) -> Self {
        self.default_level = level;
        self
    }

    /// Finalizes the filter configuration and returns a hash map of module name and log level pairs.
    pub fn build(self) -> HashMap<&'static str, LevelFilter> {
        self.filters
    }
}
