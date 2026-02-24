use std::path::PathBuf;
use storage_bindings::{download_stream, DownloadStreamOptions};

use super::storage_lifecycle::StorageManager;
use super::storage_types::{DownloadResult, StorageError};

pub fn parse_peer(peer_str: &str) -> Result<(String, String), StorageError> {
    let parts: Vec<&str> = peer_str.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(StorageError::Configuration(format!(
            "Invalid peer format '{}', expected 'peerId:address'",
            peer_str
        )));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

pub fn parse_peers(peers_str: &str) -> Result<Vec<(String, String)>, StorageError> {
    if peers_str.trim().is_empty() {
        return Ok(Vec::new());
    }

    peers_str
        .split(',')
        .map(|s| parse_peer(s.trim()))
        .collect()
}

pub async fn download_pmtiles_file(
    cid: &str,
    save_path: PathBuf,
    storage_manager: &StorageManager,
) -> Result<DownloadResult, StorageError> {
    if cid.is_empty() {
        return Err(StorageError::InvalidCid("CID cannot be empty".to_string()));
    }

    let node = storage_manager.get_node().await?;

    if !node.is_started() {
        return Err(StorageError::NodeNotStarted);
    }

    let download_options = DownloadStreamOptions::new(cid)
        .filepath(&save_path);

    let result = download_stream(&node, cid, download_options)
        .await
        .map_err(|e| StorageError::Download(e.to_string()))?;

    Ok(DownloadResult {
        cid: cid.to_string(),
        size: result.size,
        filepath: save_path.to_string_lossy().to_string(),
    })
}

pub async fn ensure_pmtiles_files(
    pmtiles_dir: PathBuf,
    storage_manager: &StorageManager,
) -> Result<Vec<(String, PathBuf)>, StorageError> {
    std::fs::create_dir_all(&pmtiles_dir)?;

    let mut files = Vec::new();

    for cid in super::storage_config::PMTILES_CIDS {
        let filename = format!("{}.pmtiles", cid);
        let file_path = pmtiles_dir.join(&filename);

        if file_path.exists() {
            files.push((filename, file_path));
            continue;
        }

        download_pmtiles_file(cid, file_path.clone(), storage_manager).await?;

        files.push((filename, file_path));
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_peer_valid() {
        let result = parse_peer("16Uiu2HAmGxKj5uXvPvH8yqL5fQJzN3jKd8X9vR2tY1wZ4pL6mN7o:/ip4/194.60.86.122/tcp/4001");
        assert!(result.is_ok());
        let (peer_id, address) = result.unwrap();
        assert_eq!(peer_id, "16Uiu2HAmGxKj5uXvPvH8yqL5fQJzN3jKd8X9vR2tY1wZ4pL6mN7o");
        assert_eq!(address, "/ip4/194.60.86.122/tcp/4001");
    }

    #[test]
    fn parse_peer_invalid_no_colon() {
        let result = parse_peer("invalidpeer");
        assert!(result.is_err());
    }

    #[test]
    fn parse_peers_single() {
        let result = parse_peers("16Uiu2HAmGxKj5uXvPvH8yqL5fQJzN3jKd8X9vR2tY1wZ4pL6mN7o:/ip4/194.60.86.122/tcp/4001");
        assert!(result.is_ok());
        let peers = result.unwrap();
        assert_eq!(peers.len(), 1);
    }

    #[test]
    fn parse_peers_multiple() {
        let result = parse_peers(
            "peer1:/ip4/192.168.1.1/tcp/4001,peer2:/ip4/192.168.1.2/tcp/4001",
        );
        assert!(result.is_ok());
        let peers = result.unwrap();
        assert_eq!(peers.len(), 2);
        assert_eq!(peers[0].0, "peer1");
        assert_eq!(peers[1].0, "peer2");
    }

    #[test]
    fn parse_peers_empty() {
        let result = parse_peers("");
        assert!(result.is_ok());
        let peers = result.unwrap();
        assert_eq!(peers.len(), 0);
    }

    #[test]
    fn parse_peers_with_spaces() {
        let result = parse_peers("peer1:/ip4/192.168.1.1/tcp/4001 , peer2:/ip4/192.168.1.2/tcp/4001");
        assert!(result.is_ok());
        let peers = result.unwrap();
        assert_eq!(peers.len(), 2);
    }
}
