pub mod storage;
pub mod uniform;
pub mod vertex;
use std::any::Any;

pub trait Component: Any + Send + Sync {}

impl<T: Any + Send + Sync> Component for T {}
