import type { Map as MapLibreMap } from 'maplibre-gl';

export type MapInstance = MapLibreMap | null;

export type MapLoadingState = 'idle' | 'loading' | 'ready' | 'error';

export interface BoundingBox {
  minLon: number;
  minLat: number;
  maxLon: number;
  maxLat: number;
}

export interface CenterPoint {
  longitude: number;
  latitude: number;
  zoom: number;
}

export interface LocalityMetadata {
  id: string;
  filename: string;
  name: string;
  description?: string;
  bounds: BoundingBox;
  center: CenterPoint;
  minZoom: number;
  maxZoom: number;
}

export interface MultiPmtilesInfo {
  localities: LocalityMetadata[];
  combinedBounds: BoundingBox;
  combinedCenter: CenterPoint;
  minZoom: number;
  maxZoom: number;
}
