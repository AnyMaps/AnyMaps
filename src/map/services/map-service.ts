import { layers, namedFlavor } from '@protomaps/basemaps';
import maplibregl from 'maplibre-gl';
import {
  $mapCenter,
  $mapError,
  $mapInstance,
  $mapLoadingState,
  $mapZoom,
  resetMapState,
} from '../states/map-state';
import type { MapInstance } from '../types/map-types';
import { createPmtilesProtocol, initPmtilesReader } from './pmtiles-service';

// todo: temporary, to replace with loading of local pmtiles
const PMTILES_URL = 'https://demo-bucket.protomaps.com/v4.pmtiles';

export async function initializeMap(
  container: HTMLElement,
): Promise<MapInstance> {
  try {
    $mapLoadingState.set('loading');

    await initPmtilesReader(PMTILES_URL);

    const protocol = createPmtilesProtocol();
    maplibregl.addProtocol('pmtiles', protocol);

    const map = new maplibregl.Map({
      container,
      style: {
        version: 8,
        sources: {
          protomaps: {
            type: 'vector',
            url: `pmtiles://${PMTILES_URL}`,
          },
        },
        layers: layers('protomaps', namedFlavor('dark'), { lang: 'en' }),
        sprite: 'https://protomaps.github.io/basemaps-assets/sprites/v4/dark',
        glyphs:
          'https://protomaps.github.io/basemaps-assets/fonts/{fontstack}/{range}.pbf',
      },
      center: $mapCenter.get(),
      zoom: $mapZoom.get(),
      attributionControl: false,
    });

    $mapInstance.set(map);

    setupMapEventListeners(map);

    return map;
  } catch (error) {
    const errorMessage =
      error instanceof Error ? error.message : 'Failed to initialize map';
    $mapLoadingState.set('error');
    $mapError.set(errorMessage);
    throw error;
  }
}

function setupMapEventListeners(map: maplibregl.Map): void {
  map.on('load', () => {
    $mapLoadingState.set('ready');
    $mapError.set(null);
  });

  map.on('error', (e) => {
    console.error('Map error:', e);
    $mapLoadingState.set('error');
    $mapError.set('Failed to load map');
  });

  map.on('moveend', () => {
    const center = map.getCenter();
    $mapCenter.set([center.lng, center.lat]);
    $mapZoom.set(map.getZoom());
  });
}

export function cleanupMap(): void {
  const map = $mapInstance.get();
  if (map) {
    map.remove();
    $mapInstance.set(null);
    maplibregl.removeProtocol('pmtiles');
  }
  resetMapState();
}
