use std::sync::Arc;

use super::storage_config::create_storage_config;
use super::storage_lifecycle::StorageManager;
use super::storage_types::StorageError;

pub struct StorageState {
    storage_manager: Arc<StorageManager>,
    app_handle: tauri::AppHandle,
}

impl StorageState {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, StorageError> {
        let config = create_storage_config(app_handle);
        let storage_manager = Arc::new(StorageManager::new(config));
        
        Ok(Self { 
            storage_manager,
            app_handle: app_handle.clone(),
        })
    }

    pub fn storage_manager(&self) -> &Arc<StorageManager> {
        &self.storage_manager
    }

    pub fn app_handle(&self) -> &tauri::AppHandle {
        &self.app_handle
    }
}
