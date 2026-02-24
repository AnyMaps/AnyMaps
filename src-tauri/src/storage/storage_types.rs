use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageError {
    NodeCreation(String),
    NodeNotInitialized,
    NodeNotStarted,
    NodeStart(String),
    NodeStop(String),
    InvalidCid(String),
    Download(String),
    Configuration(String),
    Io(String),
    Connection(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::NodeCreation(msg) => write!(f, "Failed to create storage node: {}", msg),
            StorageError::NodeNotInitialized => write!(f, "Storage node not initialized"),
            StorageError::NodeNotStarted => write!(f, "Storage node not started"),
            StorageError::NodeStart(msg) => write!(f, "Failed to start storage node: {}", msg),
            StorageError::NodeStop(msg) => write!(f, "Failed to stop storage node: {}", msg),
            StorageError::InvalidCid(msg) => write!(f, "Invalid CID: {}", msg),
            StorageError::Download(msg) => write!(f, "Download failed: {}", msg),
            StorageError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
            StorageError::Io(msg) => write!(f, "I/O error: {}", msg),
            StorageError::Connection(msg) => write!(f, "Connection error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::Io(err.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadResult {
    pub cid: String,
    pub size: usize,
    pub filepath: String,
}
