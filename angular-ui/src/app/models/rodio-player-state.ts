import {Track} from './track';

export interface RodioPlayerState {
  currentTrack: Track | null;
  paused: boolean;
  shuffle: boolean;
  repeat: 'not' | 'single' | 'endless';
  volume: number;
  time: number;
}
