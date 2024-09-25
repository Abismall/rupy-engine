use env_logger;
use log::{LevelFilter, Log, SetLoggerError};
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
        let mut env_logger_builder = env_logger::Builder::new();
        env_logger_builder
            .filter_level(level)
            .filter_module("wgpu_hal", LevelFilter::Error)
            .filter_module("naga", LevelFilter::Error)
            .filter_module("wgpu_core", LevelFilter::Error)
            .filter_module("cosmic_text", LevelFilter::Error);

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

struct Logger {
    env_logger: env_logger::Logger,
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.env_logger.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        self.env_logger.log(record)
    }

    fn flush(&self) {}
}
