import { invoke } from '@tauri-apps/api/core';
import type { GetResourceResponse, RequestParameters } from 'maplibre-gl';
import { $pmtilesInitialized } from '../states/map-state';
import type { LocalityMetadata, MultiPmtilesInfo } from '../types/map-types';

let cachedMetadata: MultiPmtilesInfo | null = null;

export async function initPmtilesReader(): Promise<MultiPmtilesInfo> {
  const info = await invoke<MultiPmtilesInfo>('init_pmtiles_reader');
  cachedMetadata = info;
  $pmtilesInitialized.set(true);
  return info;
}

export async function getPmtilesTile(
  z: number,
  x: number,
  y: number,
): Promise<number[] | null> {
  return await invoke<number[] | null>('get_pmtiles_tile', { z, x, y });
}

export async function getLocalities(): Promise<LocalityMetadata[]> {
  return await invoke<LocalityMetadata[]>('get_localities');
}

export function createPmtilesProtocol() {
  return async (
    request: RequestParameters,
    abortController: AbortController,
  ): Promise<GetResourceResponse<unknown>> => {
    if (abortController.signal.aborted) {
      throw new DOMException('Aborted', 'AbortError');
    }

    const isInitialized = $pmtilesInitialized.get();
    if (!isInitialized) {
      throw new Error('PMTiles not initialized. Call initPmtilesReader first.');
    }

    if (request.type === 'json') {
      const metadata = cachedMetadata;
      if (!metadata) {
        throw new Error('PMTiles metadata not available');
      }

      return {
        data: {
          tiles: [`${request.url}/{z}/{x}/{y}`],
          minzoom: metadata.minZoom,
          maxzoom: metadata.maxZoom,
          bounds: [
            metadata.combinedBounds.minLon,
            metadata.combinedBounds.minLat,
            metadata.combinedBounds.maxLon,
            metadata.combinedBounds.maxLat,
          ],
        },
      };
    }

    const re = new RegExp(/pmtiles:\/\/.+\/(\d+)\/(\d+)\/(\d+)/);
    const result = request.url.match(re);

    if (!result) {
      throw new Error('Invalid PMTiles protocol URL');
    }

    const z = parseInt(result[1], 10);
    const x = parseInt(result[2], 10);
    const y = parseInt(result[3], 10);

    const data = await getPmtilesTile(z, x, y);

    if (data) {
      return {
        data: new Uint8Array(data),
      };
    }

    return {
      data: new Uint8Array(),
    };
  };
}
