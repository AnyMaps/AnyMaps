import { atom } from 'nanostores';
import type { MapInstance, MapLoadingState } from '../types/map-types';

export const $mapLoadingState = atom<MapLoadingState>('idle');

export const $mapError = atom<string | null>(null);

export const $mapInstance = atom<MapInstance>(null);

export const $pmtilesInitialized = atom<boolean>(false);

export function resetMapState(): void {
  $mapLoadingState.set('idle');
  $mapError.set(null);
  $mapInstance.set(null);
  $pmtilesInitialized.set(false);
}
