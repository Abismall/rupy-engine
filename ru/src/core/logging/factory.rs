use env_logger;
use log::{LevelFilter, SetLoggerError};

use super::{level_filter::LogLevelFilterFactory, Logger};
pub struct LogFactory {
    pub env_logger: env_logger::Builder,
}

impl Default for LogFactory {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        {
            Self::new(LevelFilter::Debug)
        }
        #[cfg(not(debug_assertions))]
        {
            Self::new(LevelFilter::Info)
        }
    }
}

impl LogFactory {
    pub fn new(level: LevelFilter) -> Self {
        let level_filters = LogLevelFilterFactory::new()
            .set_default_level(LevelFilter::Error)
            .add_filter("naga")
            .add_filter("cosmic_text")
            .add_filter("wgpu_hal")
            .set_default_level(LevelFilter::Error)
            .add_filter("wgpu_core");
        let mut env_logger_builder = env_logger::Builder::new();
        env_logger_builder.filter_level(level);
        level_filters.get_filters().into_iter().for_each(|v| {
            env_logger_builder.filter_module(v.0, v.1);
        });

        Self {
            env_logger: env_logger_builder,
        }
    }

    pub fn custom(env_logger_builder: env_logger::Builder) -> Self {
        Self {
            env_logger: env_logger_builder,
        }
    }

    pub fn init(mut self) -> Result<(), SetLoggerError> {
        let logger = Logger {
            env_logger: self.env_logger.build(),
        };

        let max_level = logger.env_logger.filter();
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(max_level);
        Ok(())
    }
}
