use crate::map::map_types::{LocalityInfo, LocalityMetadata};
use lru::LruCache;
use nonzero_ext::nonzero;
use pmtiles::{AsyncPmTilesReader, MmapBackend};
use rstar::{RTree, AABB};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

const MAX_CACHED_READERS: NonZeroUsize = nonzero!(10usize);

#[derive(Debug, Clone)]
pub struct SpatialIndexEntry {
    pub locality_id: String,
    pub min_lon: f64,
    pub min_lat: f64,
    pub max_lon: f64,
    pub max_lat: f64,
}

impl rstar::RTreeObject for SpatialIndexEntry {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            [self.min_lon, self.min_lat],
            [self.max_lon, self.max_lat],
        )
    }
}

impl SpatialIndexEntry {
    pub fn from_locality(info: &LocalityInfo) -> Self {
        Self {
            locality_id: info.id.clone(),
            min_lon: info.bounds.min_lon,
            min_lat: info.bounds.min_lat,
            max_lon: info.bounds.max_lon,
            max_lat: info.bounds.max_lat,
        }
    }
}

pub struct MapState {
    pub spatial_index: Arc<RwLock<RTree<SpatialIndexEntry>>>,

    pub reader_cache: Arc<RwLock<LruCache<String, Arc<AsyncPmTilesReader<MmapBackend>>>>>,

    pub locality_metadata: Arc<RwLock<HashMap<String, LocalityMetadata>>>,

    pub pmtiles_dir: Arc<RwLock<Option<PathBuf>>>,
}

impl MapState {
    pub fn new() -> Self {
        Self {
            spatial_index: Arc::new(RwLock::new(RTree::new())),
            reader_cache: Arc::new(RwLock::new(LruCache::new(MAX_CACHED_READERS))),
            locality_metadata: Arc::new(RwLock::new(HashMap::new())),
            pmtiles_dir: Arc::new(RwLock::new(None)),
        }
    }

    #[allow(dead_code)]
    pub async fn is_initialized(&self) -> bool {
        let metadata = self.locality_metadata.read().await;
        !metadata.is_empty()
    }

    #[allow(dead_code)]
    pub async fn locality_count(&self) -> usize {
        let metadata = self.locality_metadata.read().await;
        metadata.len()
    }
}

impl Default for MapState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::map_types::BoundingBox;

    #[test]
    fn spatial_index_entry_from_locality() {
        let info = LocalityInfo {
            id: "test-locality".to_string(),
            bounds: BoundingBox::new(-75.0, 45.0, -74.0, 46.0),
            file_path: PathBuf::from("/path/to/test.pmtiles"),
        };

        let entry = SpatialIndexEntry::from_locality(&info);

        assert_eq!(entry.locality_id, "test-locality");
        assert_eq!(entry.min_lon, -75.0);
        assert_eq!(entry.min_lat, 45.0);
        assert_eq!(entry.max_lon, -74.0);
        assert_eq!(entry.max_lat, 46.0);
    }

    #[test]
    fn spatial_index_query() {
        let entries = vec![
            SpatialIndexEntry {
                locality_id: "ottawa".to_string(),
                min_lon: -76.5,
                min_lat: 44.9,
                max_lon: -75.0,
                max_lat: 45.6,
            },
            SpatialIndexEntry {
                locality_id: "gatineau".to_string(),
                min_lon: -76.0,
                min_lat: 45.3,
                max_lon: -75.3,
                max_lat: 45.7,
            },
        ];

        let tree = RTree::bulk_load(entries);

        let query_bounds = AABB::from_corners([-75.5, 45.4], [-75.4, 45.5]);
        let results: Vec<_> = tree.locate_in_envelope_intersecting(&query_bounds).collect();

        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn map_state_initialization() {
        let state = MapState::new();

        assert!(!state.is_initialized().await);
        assert_eq!(state.locality_count().await, 0);
    }
}
