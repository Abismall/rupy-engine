pub mod material;
pub mod model;
pub mod storage;
pub mod transform;
pub mod uniform;
pub mod vertices;

use std::any::Any;

pub trait Component: Any + Send + Sync {}

impl<T: Any + Send + Sync> Component for T {}
