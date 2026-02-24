//! Storage module for downloading pmtiles files from Logos Storage network
//! 
//! This module provides:
//! - Storage node lifecycle management (on-demand start/stop)
//! - File download from Storage network using CIDs
//! - Configuration for bootstrap nodes and pmtiles CIDs
//! - Peer connection management

pub mod storage_cmd;
mod storage_config;
mod storage_lifecycle;
mod storage_service;
mod storage_state;
pub mod storage_types;

pub use storage_lifecycle::StorageManager;
pub use storage_service::{parse_peer, parse_peers};
pub use storage_state::StorageState;
