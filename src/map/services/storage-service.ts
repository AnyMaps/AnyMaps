import { invoke } from '@tauri-apps/api/core';

// Hardcoded peer configuration (until discovery issues are solved)
const PEER_ID = '16Uiu2HAmKqXGniwogfJiWqbUA51eSkQyf7aQqJSXwDfNxjLvS7jm';
const PEER_ADDRESS = '/ip4/194.60.86.122/tcp/4001';

export async function startStorageNode(): Promise<void> {
  await invoke('start_storage_node');
}

export async function stopStorageNode(): Promise<void> {
  await invoke('stop_storage_node');
}

export async function connectToPeer(
  peerId: string,
  address: string,
): Promise<void> {
  const peerSpec = `${peerId}:${address}`;
  await invoke('connect_to_peer', { peerSpec });
}

export async function connectToHardcodedPeer(): Promise<void> {
  await connectToPeer(PEER_ID, PEER_ADDRESS);
  console.log(`Connected to peer: ${PEER_ID}`);
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
    await connectToHardcodedPeer();
    console.log('Successfully connected to peer');

    console.log('Downloading PMTiles files...');
    const fileCount = await downloadPmtilesFiles();
    console.log(`Successfully downloaded ${fileCount} PMTiles file(s)`);
  } catch (error) {
    console.error('Failed to initialize storage:', error);
    throw error;
  }
}
