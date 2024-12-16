use crate::core::cache::HashCache;

use super::model::Material;

pub struct MaterialManager {
    pub materials: HashCache<Material>,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self {
            materials: HashCache::new(),
        }
    }
}
