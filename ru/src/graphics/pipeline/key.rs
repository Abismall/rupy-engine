use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct CacheKey {
    id: String,
}

impl CacheKey {
    /// Creates a new CacheKey with the specified identifier.
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }

    /// Returns a unique hash value for the CacheKey, suitable for use as a hashmap key.
    pub fn as_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CacheKey {}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
