use super::state::ApplicationState;
use crate::{input::handler::InputHandler, rupyLogger::LogFactory};

pub struct Rupy {
    pub(crate) state: Option<ApplicationState>,
    #[cfg(feature = "logging")]
    pub logger: LogFactory,
    pub input: InputHandler,
}

impl Rupy {
    pub fn new() -> Self {
        Rupy {
            #[cfg(feature = "logging")]
            logger: Default::default(),
            input: InputHandler::new(),
            state: None,
        }
    }
}
