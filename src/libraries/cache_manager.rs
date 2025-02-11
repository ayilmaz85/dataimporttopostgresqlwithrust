use std::collections::HashMap;

pub struct CacheManager {
    pub genre_cache: HashMap<String, i32>,
}

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {
            genre_cache: HashMap::new(),
        }
    }
}
