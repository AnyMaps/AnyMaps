use std::sync::Arc;
use storage_bindings::{connect, StorageNode};
use tokio::sync::Mutex;

use super::storage_types::StorageError;

pub struct StorageManager {
    node: Arc<Mutex<Option<StorageNode>>>,
    config: storage_bindings::StorageConfig,
}

impl StorageManager {
    pub fn new(config: storage_bindings::StorageConfig) -> Self {
        Self {
            node: Arc::new(Mutex::new(None)),
            config,
        }
    }

    pub async fn initialize(&self) -> Result<(), StorageError> {
        let mut node_guard = self.node.lock().await;
        
        if node_guard.is_some() {
            // Already initialized
            return Ok(());
        }

        let node = StorageNode::new(self.config.clone())
            .await
            .map_err(|e| StorageError::NodeCreation(e.to_string()))?;

        *node_guard = Some(node);
        Ok(())
    }

    pub async fn start_node(&self) -> Result<(), StorageError> {
        let mut node_guard = self.node.lock().await;
        
        let node = node_guard
            .as_mut()
            .ok_or(StorageError::NodeNotInitialized)?;

        if node.is_started() {
            return Ok(());
        }

        node.start()
            .await
            .map_err(|e| StorageError::NodeStart(e.to_string()))?;

        Ok(())
    }

    pub async fn stop_node(&self) -> Result<(), StorageError> {
        let mut node_guard = self.node.lock().await;
        
        if let Some(node) = node_guard.as_mut() {
            if node.is_started() {
                node.stop()
                    .await
                    .map_err(|e| StorageError::NodeStop(e.to_string()))?;
            }
        }

        Ok(())
    }

    pub async fn get_node(&self) -> Result<StorageNode, StorageError> {
        let node_guard = self.node.lock().await;
        
        node_guard
            .as_ref()
            .ok_or(StorageError::NodeNotInitialized)
            .cloned()
    }

    pub async fn connect_to_peer(
        &self,
        peer_id: String,
        addresses: Vec<String>,
    ) -> Result<(), StorageError> {
        let node = {
            let node_guard = self.node.lock().await;
            node_guard
                .as_ref()
                .ok_or(StorageError::NodeNotInitialized)?
                .clone()
        };

        if !node.is_started() {
            return Err(StorageError::NodeNotStarted);
        }

        connect(&node, &peer_id, &addresses)
            .await
            .map_err(|e| StorageError::Connection(e.to_string()))?;

        Ok(())
    }

}

impl Clone for StorageManager {
    fn clone(&self) -> Self {
        Self {
            node: Arc::clone(&self.node),
            config: self.config.clone(),
        }
    }
}
