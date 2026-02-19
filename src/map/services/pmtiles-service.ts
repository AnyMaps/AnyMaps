import { invoke } from '@tauri-apps/api/core';
import type { GetResourceResponse, RequestParameters } from 'maplibre-gl';
import { $pmtilesInitialized } from '../states/map-state';
import type { PmtilesInfo, PmtilesMetadata } from '../types/map-types';

export async function initPmtilesReader(): Promise<PmtilesInfo> {
  const pmtilesInfo = await invoke<PmtilesInfo>('init_pmtiles_reader');
  $pmtilesInitialized.set(true);
  return pmtilesInfo;
}

export async function getPmtilesHeader(): Promise<PmtilesMetadata> {
  return await invoke<PmtilesMetadata>('get_pmtiles_header');
}

export async function getPmtilesTile(
  z: number,
  x: number,
  y: number,
): Promise<number[] | null> {
  return await invoke<number[] | null>('get_pmtiles_tile', { z, x, y });
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
      const header = await getPmtilesHeader();

      return {
        data: {
          tiles: [`${request.url}/{z}/{x}/{y}`],
          minzoom: header.minZoom,
          maxzoom: header.maxZoom,
          bounds: header.bounds,
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
