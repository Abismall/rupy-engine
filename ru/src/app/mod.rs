pub mod app;
pub mod flags;
pub mod handler;
pub mod state;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DebugMode {
    None,
    Minimal,
    Verbose,
}

impl DebugMode {
    pub fn next(self) -> DebugMode {
        match self {
            DebugMode::None => DebugMode::Minimal,
            DebugMode::Minimal => DebugMode::Verbose,
            DebugMode::Verbose => DebugMode::None,
        }
    }
}
