use pmtiles::TileCoord;
use tauri::State;

use super::map_service;
use super::map_state::MapState;
use super::map_types::{PmtilesInfo, PmtilesMetadata};

#[tauri::command]
pub async fn init_pmtiles_reader(
    app: tauri::AppHandle,
    state: State<'_, MapState>,
) -> Result<PmtilesInfo, String> {
    map_service::init_reader(&app, &state).await
}

#[tauri::command]
pub async fn get_pmtiles_header(
    state: State<'_, MapState>,
) -> Result<PmtilesMetadata, String> {
    let guard = state.reader.read().await;

    let reader = guard
        .as_ref()
        .ok_or_else(|| "PMTiles reader not initialized".to_string())?;

    Ok(map_service::extract_metadata(reader))
}

#[tauri::command]
pub async fn get_pmtiles_tile(
    z: u8,
    x: u32,
    y: u32,
    state: State<'_, MapState>,
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
