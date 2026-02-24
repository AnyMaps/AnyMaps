mod map;
mod storage;

use map::{map_cmd, MapState};
use storage::{storage_cmd, StorageState};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let storage_state = StorageState::new(app.handle())
                .expect("Failed to initialize storage state");
            app.manage(storage_state);

            app.manage(MapState::new());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            map_cmd::init_pmtiles_reader,
            map_cmd::get_pmtiles_tile,
            map_cmd::get_localities,
            storage_cmd::start_storage_node,
            storage_cmd::stop_storage_node,
            storage_cmd::connect_to_peer,
            storage_cmd::connect_to_peers,
            storage_cmd::download_pmtiles_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
