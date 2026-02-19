use pmtiles::{AsyncPmTilesReader, MmapBackend};
use std::path::PathBuf;
use tauri::Manager;

use super::map_state::MapState;
use super::map_types::{PmtilesInfo, PmtilesMetadata};

const PMTILES_RESOURCE_DIR: &str = "pmtiles";

pub fn get_pmtiles_resource_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let resource_dir = app
        .path()
        .resolve(PMTILES_RESOURCE_DIR, tauri::path::BaseDirectory::Resource);

    match resource_dir {
        Ok(path) if path.exists() => Ok(path),
        Ok(path) => {
            let cwd = std::env::current_dir().map_err(|e| format!("Failed to get cwd: {}", e))?;
            let dev_path = cwd.join("src-tauri").join(PMTILES_RESOURCE_DIR);

            if dev_path.exists() {
                return Ok(dev_path);
            }

            Err(format!(
                "PMTiles resource directory not found. Tried:\n  - Resource: '{}'\n  - Dev: '{}'\nEnsure 'pmtiles/' directory exists in src-tauri/",
                path.display(),
                dev_path.display()
            ))
        }
        Err(e) => Err(format!(
            "Failed to resolve PMTiles resource directory: {}. Ensure 'pmtiles/' is configured in tauri.conf.json bundle.resources",
            e
        )),
    }
}

pub async fn find_first_pmtiles_file(app: &tauri::AppHandle) -> Result<(String, PathBuf), String> {
    let resource_dir = get_pmtiles_resource_dir(app)?;

    let mut entries = std::fs::read_dir(&resource_dir).map_err(|e| {
        format!(
            "Failed to read PMTiles resource directory '{}': {}",
            resource_dir.display(),
            e
        )
    })?;

    while let Some(entry) = entries.next() {
        let entry = entry.map_err(|e| {
            format!(
                "Failed to read directory entry in '{}': {}",
                resource_dir.display(),
                e
            )
        })?;

        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "pmtiles") {
            if let Some(filename) = path.file_name() {
                return Ok((filename.to_string_lossy().to_string(), path));
            }
        }
    }

    Err(format!(
        "No PMTiles files found in '{}'. Add .pmtiles files to the directory.",
        resource_dir.display()
    ))
}

pub async fn init_reader(
    app: &tauri::AppHandle,
    state: &tauri::State<'_, MapState>,
) -> Result<PmtilesInfo, String> {
    let (filename, file_path) = find_first_pmtiles_file(app).await?;

    let backend = MmapBackend::try_from(&file_path)
        .await
        .map_err(|e| format!("Failed to open PMTiles file '{}': {}", filename, e))?;

    let reader = AsyncPmTilesReader::try_from_source(backend)
        .await
        .map_err(|e| format!("Failed to initialize PMTiles reader: {}", e))?;

    let metadata = extract_metadata(&reader);

    {
        let mut guard = state.reader.write().await;
        *guard = Some(reader);
    }
    {
        let mut guard = state.current_file.write().await;
        *guard = Some(filename.clone());
    }

    Ok(PmtilesInfo { filename, metadata })
}

pub fn extract_metadata(reader: &AsyncPmTilesReader<MmapBackend>) -> PmtilesMetadata {
    let header = reader.get_header();
    let bounds = header.get_bounds();

    PmtilesMetadata {
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
    }
}
