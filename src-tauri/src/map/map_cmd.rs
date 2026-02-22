use tauri::State;

use super::map_service;
use super::map_state::MapState;
use super::map_types::MultiPmtilesInfo;

#[tauri::command]
pub async fn init_pmtiles_reader(
    app: tauri::AppHandle,
    state: State<'_, MapState>,
) -> Result<MultiPmtilesInfo, String> {
    map_service::init_multi_reader(&app, &state).await
}

#[tauri::command]
pub async fn get_pmtiles_tile(
    z: u8,
    x: u32,
    y: u32,
    state: State<'_, MapState>,
) -> Result<Option<Vec<u8>>, String> {
    map_service::get_tile(z, x, y, &state).await
}

#[tauri::command]
pub async fn get_localities(
    state: State<'_, MapState>,
) -> Result<Vec<crate::map::map_types::LocalityMetadata>, String> {
    let metadata = state.locality_metadata.read().await;
    Ok(metadata.values().cloned().collect())
}
