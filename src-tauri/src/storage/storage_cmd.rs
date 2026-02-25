use tauri::{Manager, State};

use super::storage_config::{PEER_ADDRESS, PEER_ID};
use super::storage_state::StorageState;
use super::{parse_peer, parse_peers};

#[tauri::command]
pub async fn start_storage_node(
    state: State<'_, StorageState>,
) -> Result<(), String> {
    state
        .storage_manager()
        .initialize()
        .await
        .map_err(|e| e.to_string())?;
    
    state
        .storage_manager()
        .start_node()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_storage_node(
    state: State<'_, StorageState>,
) -> Result<(), String> {
    state
        .storage_manager()
        .stop_node()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn connect_to_peer(
    state: State<'_, StorageState>,
) -> Result<(), String> {
    let peer_spec = format!("{}:{}", PEER_ID, PEER_ADDRESS);
    let (peer_id, address) = parse_peer(&peer_spec)
        .map_err(|e| e.to_string())?;
    
    state
        .storage_manager()
        .connect_to_peer(peer_id, vec![address])
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn connect_to_peers(
    peers: String,
    state: State<'_, StorageState>,
) -> Result<usize, String> {
    let peer_list = parse_peers(&peers)
        .map_err(|e| e.to_string())?;
    
    let mut connected_count = 0;
    
    for (peer_id, address) in &peer_list {
        match state
            .storage_manager()
            .connect_to_peer(peer_id.clone(), vec![address.clone()])
            .await
        {
            Ok(_) => connected_count += 1,
            Err(e) => {
                eprintln!("Failed to connect to peer: {}", e);
            }
        }
    }
    
    Ok(connected_count)
}

#[tauri::command]
pub async fn download_pmtiles_files(
    state: State<'_, StorageState>,
) -> Result<usize, String> {
    let app = state.app_handle();
    let pmtiles_dir = app.path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?
        .join("pmtiles");
    
    let storage_manager = state.storage_manager();
    
    storage_manager.initialize()
        .await
        .map_err(|e| format!("Failed to initialize storage node: {}", e))?;
    
    storage_manager.start_node()
        .await
        .map_err(|e| format!("Failed to start storage node: {}", e))?;
    
    let files = super::storage_service::ensure_pmtiles_files(pmtiles_dir, storage_manager.as_ref())
        .await
        .map_err(|e| format!("Failed to download pmtiles files: {}", e))?;
    
    Ok(files.len())
}
