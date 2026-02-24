use tauri::State;

use super::map_service;
use super::map_state::MapState;
use super::map_types::MultiPmtilesInfo;
use crate::storage::StorageState;

#[tauri::command]
pub async fn init_pmtiles_reader(
    app: tauri::AppHandle,
    map_state: State<'_, MapState>,
    storage_state: State<'_, StorageState>,
) -> Result<MultiPmtilesInfo, String> {
    let storage_manager = storage_state.storage_manager();
    map_service::init_multi_reader(&app, &map_state, storage_manager.as_ref()).await
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
