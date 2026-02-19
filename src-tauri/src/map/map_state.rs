use pmtiles::{AsyncPmTilesReader, MmapBackend};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MapState {
    pub reader: Arc<RwLock<Option<AsyncPmTilesReader<MmapBackend>>>>,
    pub current_file: Arc<RwLock<Option<String>>>,
}

impl MapState {
    pub fn new() -> Self {
        Self {
            reader: Arc::new(RwLock::new(None)),
            current_file: Arc::new(RwLock::new(None)),
        }
    }
}

impl Default for MapState {
    fn default() -> Self {
        Self::new()
    }
}
