import { layers, namedFlavor } from '@protomaps/basemaps';
import maplibregl from 'maplibre-gl';
import {
  $mapError,
  $mapInstance,
  $mapLoadingState,
  resetMapState,
} from '../states/map-state';
import type { MapInstance } from '../types/map-types';
import { createPmtilesProtocol, initPmtilesReader } from './pmtiles-service';

const InitZoomLevel = 10;

export async function initializeMap(
  container: HTMLElement,
): Promise<MapInstance> {
  try {
    $mapLoadingState.set('loading');

    const pmtilesInfo = await initPmtilesReader();

    const protocol = createPmtilesProtocol();
    maplibregl.addProtocol('pmtiles', protocol);

    const [west, south, east, north] = pmtilesInfo.metadata.bounds;
    const centerLng = (west + east) / 2;
    const centerLat = (south + north) / 2;

    const map = new maplibregl.Map({
      container,
      style: {
        version: 8,
        sources: {
          protomaps: {
            type: 'vector',
            url: 'pmtiles://local',
            minzoom: pmtilesInfo.metadata.minZoom,
            maxzoom: pmtilesInfo.metadata.maxZoom,
            bounds: pmtilesInfo.metadata.bounds as [
              number,
              number,
              number,
              number,
            ],
          },
        },
        layers: layers('protomaps', namedFlavor('dark'), { lang: 'en' }),
        sprite: 'https://protomaps.github.io/basemaps-assets/sprites/v4/dark',
        glyphs:
          'https://protomaps.github.io/basemaps-assets/fonts/{fontstack}/{range}.pbf',
      },
      center: [centerLng, centerLat],
      zoom: InitZoomLevel,
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
