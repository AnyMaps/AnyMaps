use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PmtilesMetadata {
    pub tile_type: String,
    pub min_zoom: u8,
    pub max_zoom: u8,
    pub min_longitude: f64,
    pub max_longitude: f64,
    pub min_latitude: f64,
    pub max_latitude: f64,
    pub bounds: Vec<f64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PmtilesInfo {
    pub filename: String,
    pub metadata: PmtilesMetadata,
}
