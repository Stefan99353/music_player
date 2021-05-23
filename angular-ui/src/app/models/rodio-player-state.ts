import {Track} from './track';

export interface RodioPlayerState {
  currentTrack: Track | null;
  paused: boolean;
  shuffle: boolean;
  repeat: 'Not' | 'Single' | 'Endless';
  volume: number;
  time: number;
}
