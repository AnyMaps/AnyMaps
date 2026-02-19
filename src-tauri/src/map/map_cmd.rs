use pmtiles::{AsyncPmTilesReader, TileCoord};
use reqwest::Client;

use super::map_state::MapState;
use super::map_types::PmtilesMetadata;

#[tauri::command]
pub async fn init_pmtiles_reader(
    url: String,
    state: tauri::State<'_, MapState>,
) -> Result<(), String> {
    let client = Client::new();
    let reader = AsyncPmTilesReader::new_with_url(client, &url)
        .await
        .map_err(|e| format!("Failed to initialize PMTiles reader: {}", e))?;

    let mut guard = state.reader.write().await;
    *guard = Some(reader);

    Ok(())
}

#[tauri::command]
pub async fn get_pmtiles_header(
    state: tauri::State<'_, MapState>,
) -> Result<PmtilesMetadata, String> {
    let guard = state.reader.read().await;

    let reader = guard
        .as_ref()
        .ok_or_else(|| "PMTiles reader not initialized".to_string())?;

    let header = reader.get_header();
    let bounds = header.get_bounds();

    Ok(PmtilesMetadata {
        tile_type: format!("{:?}", header.tile_type),
        min_zoom: header.min_zoom,
        max_zoom: header.max_zoom,
        min_longitude: header.min_longitude as f64,
        max_longitude: header.max_longitude as f64,
        min_latitude: header.min_latitude as f64,
        max_latitude: header.max_latitude as f64,
        bounds: vec![
            bounds.left as f64,
            bounds.bottom as f64,
            bounds.right as f64,
            bounds.top as f64,
        ],
    })
}

#[tauri::command]
pub async fn get_pmtiles_tile(
    z: u8,
    x: u32,
    y: u32,
    state: tauri::State<'_, MapState>,
) -> Result<Option<Vec<u8>>, String> {
    let guard = state.reader.read().await;

    let reader = guard
        .as_ref()
        .ok_or_else(|| "PMTiles reader not initialized".to_string())?;

    let coord = TileCoord::new(z, x, y).map_err(|e| e.to_string())?;

    match reader.get_tile_decompressed(coord).await {
        Ok(Some(tile)) => Ok(Some(tile.to_vec())),
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Failed to get tile: {}", e)),
    }
}
