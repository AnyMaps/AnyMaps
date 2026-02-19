mod map;

use map::{map_cmd, MapState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(MapState::new())
        .invoke_handler(tauri::generate_handler![
            map_cmd::init_pmtiles_reader,
            map_cmd::get_pmtiles_header,
            map_cmd::get_pmtiles_tile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
