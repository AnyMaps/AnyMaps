use pmtiles::{AsyncPmTilesReader, HttpBackend};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MapState {
    pub reader: Arc<RwLock<Option<AsyncPmTilesReader<HttpBackend>>>>,
}

impl MapState {
    pub fn new() -> Self {
        Self {
            reader: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for MapState {
    fn default() -> Self {
        Self::new()
    }
}
