use crate::map::map_state::{MapState, SpatialIndexEntry};
use crate::map::map_types::{
    BoundingBox, CenterPoint, LocalityInfo, LocalityMetadata, MultiPmtilesInfo,
};
use pmtiles::{AsyncPmTilesReader, MmapBackend, TileCoord};
use rstar::AABB;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager;

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

pub async fn discover_all_pmtiles_files(
    app: &tauri::AppHandle,
) -> Result<Vec<(String, PathBuf)>, String> {
    let resource_dir = get_pmtiles_resource_dir(app)?;

    let mut entries = std::fs::read_dir(&resource_dir).map_err(|e| {
        format!(
            "Failed to read PMTiles resource directory '{}': {}",
            resource_dir.display(),
            e
        )
    })?;

    let mut files = Vec::new();

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
                files.push((filename.to_string_lossy().to_string(), path));
            }
        }
    }

    if files.is_empty() {
        return Err(format!(
            "No PMTiles files found in '{}'. Add .pmtiles files to the directory.",
            resource_dir.display()
        ));
    }

    Ok(files)
}

async fn extract_locality_metadata(
    filename: &str,
    file_path: &PathBuf,
) -> Result<LocalityMetadata, String> {
    let backend = MmapBackend::try_from(file_path)
        .await
        .map_err(|e| format!("Failed to open PMTiles file '{}': {}", filename, e))?;

    let reader = AsyncPmTilesReader::try_from_source(backend)
        .await
        .map_err(|e| format!("Failed to initialize PMTiles reader for '{}': {}", filename, e))?;

    let header = reader.get_header();

    let (name, description) = match reader.get_metadata().await {
        Ok(metadata_str) => {
            // Parse JSON metadata
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(&metadata_str);
            match parsed {
                Ok(json) => {
                    let name = json
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| filename_to_locality_name(filename));
                    let description = json.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
                    (name, description)
                }
                Err(_) => (filename_to_locality_name(filename), None),
            }
        }
        Err(_) => (filename_to_locality_name(filename), None),
    };

    let id = filename
        .strip_suffix(".pmtiles")
        .unwrap_or(filename)
        .to_string();

    Ok(LocalityMetadata {
        id: id.clone(),
        filename: filename.to_string(),
        name,
        description,
        bounds: BoundingBox::new(
            header.min_longitude,
            header.min_latitude,
            header.max_longitude,
            header.max_latitude,
        ),
        center: CenterPoint {
            longitude: header.center_longitude,
            latitude: header.center_latitude,
            zoom: header.center_zoom,
        },
        min_zoom: header.min_zoom,
        max_zoom: header.max_zoom,
    })
}

fn filename_to_locality_name(filename: &str) -> String {
    filename
        .strip_suffix(".pmtiles")
        .unwrap_or(filename)
        .replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub async fn init_multi_reader(
    app: &tauri::AppHandle,
    state: &tauri::State<'_, MapState>,
) -> Result<MultiPmtilesInfo, String> {
    let files = discover_all_pmtiles_files(app).await?;

    let pmtiles_dir = get_pmtiles_resource_dir(app)?;
    {
        let mut guard = state.pmtiles_dir.write().await;
        *guard = Some(pmtiles_dir.clone());
    }

    let mut localities = Vec::new();
    let mut spatial_entries = Vec::new();
    let mut metadata_map = std::collections::HashMap::new();

    for (filename, file_path) in files {
        match extract_locality_metadata(&filename, &file_path).await {
            Ok(metadata) => {
                let info = LocalityInfo {
                    id: metadata.id.clone(),
                    bounds: metadata.bounds,
                    file_path,
                };

                spatial_entries.push(SpatialIndexEntry::from_locality(&info));
                metadata_map.insert(metadata.id.clone(), metadata.clone());
                localities.push(metadata);
            }
            Err(e) => {
                eprintln!("Warning: Failed to extract metadata from {}: {}", filename, e);
            }
        }
    }

    if localities.is_empty() {
        return Err("No valid PMTiles files found".to_string());
    }

    let spatial_index = rstar::RTree::bulk_load(spatial_entries);

    let combined_bounds = calculate_combined_bounds(&localities);
    let combined_center = CenterPoint {
        longitude: (combined_bounds.min_lon + combined_bounds.max_lon) / 2.0,
        latitude: (combined_bounds.min_lat + combined_bounds.max_lat) / 2.0,
        zoom: localities.iter().map(|l| l.min_zoom).min().unwrap_or(0),
    };
    let min_zoom = localities.iter().map(|l| l.min_zoom).min().unwrap_or(0);
    let max_zoom = localities.iter().map(|l| l.max_zoom).max().unwrap_or(14);

    {
        let mut guard = state.spatial_index.write().await;
        *guard = spatial_index;
    }
    {
        let mut guard = state.locality_metadata.write().await;
        *guard = metadata_map;
    }

    Ok(MultiPmtilesInfo {
        localities,
        combined_bounds,
        combined_center,
        min_zoom,
        max_zoom,
    })
}

fn calculate_combined_bounds(localities: &[LocalityMetadata]) -> BoundingBox {
    let mut min_lon = f64::MAX;
    let mut min_lat = f64::MAX;
    let mut max_lon = f64::MIN;
    let mut max_lat = f64::MIN;

    for locality in localities {
        min_lon = min_lon.min(locality.bounds.min_lon);
        min_lat = min_lat.min(locality.bounds.min_lat);
        max_lon = max_lon.max(locality.bounds.max_lon);
        max_lat = max_lat.max(locality.bounds.max_lat);
    }

    BoundingBox::new(min_lon, min_lat, max_lon, max_lat)
}

pub fn tile_to_bounds(z: u8, x: u32, y: u32) -> BoundingBox {
    let n = 2u32.pow(z as u32) as f64;

    let min_lon = (x as f64 / n) * 360.0 - 180.0;
    let max_lon = ((x + 1) as f64 / n) * 360.0 - 180.0;

    let max_lat = (std::f64::consts::PI * (1.0 - 2.0 * (y as f64 / n))).sinh().atan().to_degrees();
    let min_lat = (std::f64::consts::PI * (1.0 - 2.0 * ((y + 1) as f64 / n)))
        .sinh()
        .atan()
        .to_degrees();

    BoundingBox::new(min_lon, min_lat, max_lon, max_lat)
}

pub async fn get_tile(
    z: u8,
    x: u32,
    y: u32,
    state: &tauri::State<'_, MapState>,
) -> Result<Option<Vec<u8>>, String> {
    let tile_bounds = tile_to_bounds(z, x, y);

    let candidates: Vec<String> = {
        let index = state.spatial_index.read().await;
        let query_envelope = AABB::from_corners(
            [tile_bounds.min_lon, tile_bounds.min_lat],
            [tile_bounds.max_lon, tile_bounds.max_lat],
        );

        index
            .locate_in_envelope_intersecting(&query_envelope)
            .map(|entry| entry.locality_id.clone())
            .collect()
    };

    for locality_id in candidates {
        match get_tile_from_locality(z, x, y, &locality_id, state).await {
            Ok(Some(tile)) => return Ok(Some(tile)),
            Ok(None) => continue,
            Err(e) => {
                eprintln!(
                    "Warning: Failed to get tile from locality {}: {}",
                    locality_id, e
                );
                continue;
            }
        }
    }

    Ok(None)
}

async fn get_tile_from_locality(
    z: u8,
    x: u32,
    y: u32,
    locality_id: &str,
    state: &tauri::State<'_, MapState>,
) -> Result<Option<Vec<u8>>, String> {
    let reader = get_or_load_reader(locality_id, state).await?;

    let coord = TileCoord::new(z, x, y).map_err(|e| e.to_string())?;

    match reader.get_tile_decompressed(coord).await {
        Ok(Some(tile)) => Ok(Some(tile.to_vec())),
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Failed to get tile: {}", e)),
    }
}

async fn get_or_load_reader(
    locality_id: &str,
    state: &tauri::State<'_, MapState>,
) -> Result<Arc<AsyncPmTilesReader<MmapBackend>>, String> {
    {
        let mut cache = state.reader_cache.write().await;
        if let Some(reader) = cache.get(locality_id) {
            return Ok(Arc::clone(reader));
        }
    }

    let (filename, file_path) = {
        let metadata = state.locality_metadata.read().await;
        let locality = metadata
            .get(locality_id)
            .ok_or_else(|| format!("Locality not found: {}", locality_id))?;

        let pmtiles_dir = state.pmtiles_dir.read().await;
        let pmtiles_dir = pmtiles_dir
            .as_ref()
            .ok_or_else(|| "PMTiles directory not set".to_string())?;

        (
            locality.filename.clone(),
            pmtiles_dir.join(&locality.filename),
        )
    };

    let backend = MmapBackend::try_from(&file_path)
        .await
        .map_err(|e| format!("Failed to open PMTiles file '{}': {}", filename, e))?;

    let reader = Arc::new(
        AsyncPmTilesReader::try_from_source(backend)
            .await
            .map_err(|e| format!("Failed to initialize reader for '{}': {}", filename, e))?,
    );

    {
        let mut cache = state.reader_cache.write().await;
        cache.put(locality_id.to_string(), Arc::clone(&reader));
    }

    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_to_locality_name() {
        assert_eq!(
            filename_to_locality_name("ottawa_locality.pmtiles"),
            "Ottawa Locality"
        );
        assert_eq!(filename_to_locality_name("gatineau.pmtiles"), "Gatineau");
        assert_eq!(
            filename_to_locality_name("new_york_city.pmtiles"),
            "New York City"
        );
    }

    #[test]
    fn test_tile_to_bounds() {
        let bounds = tile_to_bounds(0, 0, 0);
        assert!((bounds.min_lon - (-180.0)).abs() < 0.01);
        assert!((bounds.max_lon - 180.0).abs() < 0.01);

        let bounds = tile_to_bounds(1, 0, 0);
        assert!((bounds.min_lon - (-180.0)).abs() < 0.01);
        assert!((bounds.max_lon - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_combined_bounds() {
        let localities = vec![
            LocalityMetadata {
                id: "ottawa".to_string(),
                filename: "ottawa.pmtiles".to_string(),
                name: "Ottawa".to_string(),
                description: None,
                bounds: BoundingBox::new(-76.5, 45.0, -75.0, 45.6),
                center: CenterPoint {
                    longitude: -75.7,
                    latitude: 45.3,
                    zoom: 10,
                },
                min_zoom: 0,
                max_zoom: 14,
            },
            LocalityMetadata {
                id: "gatineau".to_string(),
                filename: "gatineau.pmtiles".to_string(),
                name: "Gatineau".to_string(),
                description: None,
                bounds: BoundingBox::new(-76.0, 45.3, -75.3, 45.7),
                center: CenterPoint {
                    longitude: -75.65,
                    latitude: 45.5,
                    zoom: 10,
                },
                min_zoom: 0,
                max_zoom: 14,
            },
        ];

        let combined = calculate_combined_bounds(&localities);

        assert!((combined.min_lon - (-76.5)).abs() < 0.01);
        assert!((combined.min_lat - 45.0).abs() < 0.01);
        assert!((combined.max_lon - (-75.0)).abs() < 0.01);
        assert!((combined.max_lat - 45.7).abs() < 0.01);
    }
}
