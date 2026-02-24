use storage_bindings::node::config::RepoKind;
use storage_bindings::{LogLevel, StorageConfig};
use tauri::{AppHandle, Manager};

const BOOTSTRAP_NODES: &[&str] = &[
    "spr:CiUIAhIhAiJvIcA_ZwPZ9ugVKDbmqwhJZaig5zKyLiuaicRcCGqLEgIDARo8CicAJQgCEiECIm8hwD9nA9n26BUoNuarCEllqKDnMrIuK5qJxFwIaosQ3d6esAYaCwoJBJ_f8zKRAnU6KkYwRAIgM0MvWNJL296kJ9gWvfatfmVvT-A7O2s8Mxp8l9c8EW0CIC-h-H-jBVSgFjg3Eny2u33qF7BDnWFzo7fGfZ7_qc9P",
    "spr:CiUIAhIhAlNJ7ary8eOK5GcwQ6q4U8brR7iWjwhMwzHb8BzzmCEDEgIDARpJCicAJQgCEiECU0ntqvLx44rkZzBDqrhTxutHuJaPCEzDMdvwHPOYIQMQsZ67vgYaCwoJBK6Kf1-RAnVEGgsKCQSuin9fkQJ1RCpGMEQCIDxd6lXDvj1PcHgQYnNpHGfgCO5a7fejg3WhSjh2wTimAiB7YHsL1WZYU_zkHcNDWhRgMbkb3C5yRuvUhjBjGOYJYQ",
    "spr:CiUIAhIhAyUvcPkKoGE7-gh84RmKIPHJPdsX5Ugm_IHVJgF-Mmu_EgIDARo8CicAJQgCEiEDJS9w-QqgYTv6CHzhGYog8ck92xflSCb8gdUmAX4ya78QoemesAYaCwoJBES39Q2RAnVOKkYwRAIgLi3rouyaZFS_Uilx8k99ySdQCP1tsmLR21tDb9p8LcgCIG30o5YnEooQ1n6tgm9fCT7s53k6XlxyeSkD_uIO9mb3"
];


pub const PMTILES_CIDS: &[&str] = &[
    "zDvZRwzm7QgTjMsYxLzSgkDRgixkKKkjyiKPuEhhZoRRLvW8NXzb"
];

pub fn create_storage_config(app_handle: &AppHandle) -> StorageConfig {
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory")
        .join("storage_data");

    if let Err(e) = std::fs::create_dir_all(&data_dir) {
        panic!(
            "Failed to create storage data directory {}: {}",
            data_dir.display(),
            e
        );
    }

    let mut config = StorageConfig::new()
        .log_level(LogLevel::Info)
        .data_dir(&data_dir)
        .storage_quota(10 * 1024 * 1024 * 1024) // 1 GB
        .max_peers(50)
        .discovery_port(8089)
        .repo_kind(RepoKind::LevelDb);

    for node in BOOTSTRAP_NODES {
        config = config.add_bootstrap_node(*node);
    }

    config
}
