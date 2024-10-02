use winit::{event_loop::ActiveEventLoop, window::Window};

use super::state::ApplicationState;
use crate::{input::handler::InputHandler, rupyLogger::LogFactory, AppError};

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
    pub async fn rehydrate(&mut self, el: &ActiveEventLoop) -> Result<(), AppError> {
        self.state = Some(ApplicationState::new(el).await);
        if self.state.is_some() {
            Ok(())
        } else {
            Err(AppError::StateRehydrationError(
                "No state after rehydration".to_owned(),
            ))
        }
    }
}
