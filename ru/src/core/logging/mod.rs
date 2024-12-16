pub mod factory;
pub mod level_filter;
pub struct Logger {
    env_logger: env_logger::Logger,
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.env_logger.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        self.env_logger.log(record)
    }

    fn flush(&self) {}
}
