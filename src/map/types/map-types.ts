import type { Map as MapLibreMap } from 'maplibre-gl';

export interface PmtilesMetadata {
  tileType: string;
  minZoom: number;
  maxZoom: number;
  minLongitude: number;
  maxLongitude: number;
  minLatitude: number;
  maxLatitude: number;
  bounds: number[];
}

export type MapLoadingState = 'idle' | 'loading' | 'ready' | 'error';

export type MapInstance = MapLibreMap | null;
