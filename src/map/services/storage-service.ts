import { invoke } from '@tauri-apps/api/core';

export async function startStorageNode(): Promise<void> {
  await invoke('start_storage_node');
}

export async function stopStorageNode(): Promise<void> {
  await invoke('stop_storage_node');
}

export async function connectToPeer(): Promise<void> {
  await invoke('connect_to_peer');
}

export async function downloadPmtilesFiles(): Promise<number> {
  const count = await invoke<number>('download_pmtiles_files');
  console.log(`Downloaded ${count} PMTiles file(s)`);
  return count;
}

export async function initializeStorage(): Promise<void> {
  try {
    console.log('Starting storage node...');
    await startStorageNode();
    console.log('Storage node started');

    console.log('Connecting to peer...');
    await connectToPeer();
    console.log('Successfully connected to peer');

    console.log('Downloading PMTiles files...');
    const fileCount = await downloadPmtilesFiles();
    console.log(`Successfully downloaded ${fileCount} PMTiles file(s)`);
  } catch (error) {
    console.error('Failed to initialize storage:', error);
    throw error;
  }
}
