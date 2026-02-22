use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    pub min_lon: f64,
    pub min_lat: f64,
    pub max_lon: f64,
    pub max_lat: f64,
}

impl BoundingBox {
    pub fn new(min_lon: f64, min_lat: f64, max_lon: f64, max_lat: f64) -> Self {
        Self {
            min_lon,
            min_lat,
            max_lon,
            max_lat,
        }
    }

    #[allow(dead_code)]
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min_lon <= other.max_lon
            && self.max_lon >= other.min_lon
            && self.min_lat <= other.max_lat
            && self.max_lat >= other.min_lat
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CenterPoint {
    pub longitude: f64,
    pub latitude: f64,
    pub zoom: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalityMetadata {
    pub id: String,
    pub filename: String,
    pub name: String,
    pub description: Option<String>,
    pub bounds: BoundingBox,
    pub center: CenterPoint,
    pub min_zoom: u8,
    pub max_zoom: u8,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LocalityInfo {
    pub id: String,
    pub bounds: BoundingBox,
    pub file_path: PathBuf,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiPmtilesInfo {
    pub localities: Vec<LocalityMetadata>,
    pub combined_bounds: BoundingBox,
    pub combined_center: CenterPoint,
    pub min_zoom: u8,
    pub max_zoom: u8,
}
