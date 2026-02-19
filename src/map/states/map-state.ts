import { atom } from 'nanostores';
import type { MapInstance, MapLoadingState } from '../types/map-types';

export const $mapLoadingState = atom<MapLoadingState>('idle');

export const $mapError = atom<string | null>(null);

export const $mapCenter = atom<[number, number]>([0, 0]);

export const $mapZoom = atom<number>(2);

export const $mapInstance = atom<MapInstance>(null);

export const $pmtilesInitialized = atom<boolean>(false);

export function resetMapState(): void {
  $mapLoadingState.set('idle');
  $mapError.set(null);
  $mapCenter.set([0, 0]);
  $mapZoom.set(2);
  $mapInstance.set(null);
  $pmtilesInitialized.set(false);
}
