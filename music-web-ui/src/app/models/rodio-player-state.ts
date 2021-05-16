import {Track} from './track';

export interface RodioPlayerState {
  currentTrack: Track | null;
  paused: boolean;
  volume: number;
  time: number;
}
