use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}
pub fn calculate_hashes<T: Hash>(item: &Vec<T>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for t in item.iter() {
        t.hash(&mut hasher);
    }
    hasher.finish()
}
pub fn string_to_u64(s: &str) -> u64 {
    calculate_hash(&s.to_string())
}
