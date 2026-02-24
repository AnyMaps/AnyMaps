import { useStore } from '@nanostores/react';
import { useEffect, useRef } from 'react';
import 'maplibre-gl/dist/maplibre-gl.css';
import { cleanupMap, initializeMap } from '../services/map-service';
import { initializeStorage } from '../services/storage-service';
import { $mapError, $mapLoadingState } from '../states/map-state';

const MapView = () => {
  const mapContainer = useRef<HTMLDivElement>(null);
  const mapInitialized = useRef(false);

  const loadingState = useStore($mapLoadingState);
  const error = useStore($mapError);

  const isReady = loadingState === 'ready';
  const hasError = loadingState === 'error';

  useEffect(() => {
    const container = mapContainer.current;
    if (!container || mapInitialized.current) return;
    $mapLoadingState.set('loading');

    mapInitialized.current = true;

    initializeStorage().then(() => {
      initializeMap(container).catch((err) => {
        $mapLoadingState.set('error');
        console.error('Failed to initialize map:', err);
      });
    });

    return () => {
      cleanupMap();
      mapInitialized.current = false;
    };
  }, []);

  if (hasError && error) {
    return (
      // todo: create an actual splash screen use LSD's Typography component to display text as error
      <div className="flex items-center justify-center w-full h-full text-red-400">
        <p>{error}</p>
      </div>
    );
  }

  return (
    <div className="relative size-full">
      {!isReady && (
        // todo: create an actual splash screen (can't access app if map's unable to load)
        <div className="absolute size-full flex justify-center items-center lsd:bg-lsd-surface z-10">
          <p>Loading map...</p>
        </div>
      )}
      <div ref={mapContainer} className="size-full" />
    </div>
  );
};

export default MapView;
