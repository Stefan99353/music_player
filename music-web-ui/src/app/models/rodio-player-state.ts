import {Track} from './track';

export interface RodioPlayerState {
  currentTrack?: Track | undefined;
  currentIndex: number;
  currentlyPlaying: boolean;
  paused: boolean;
  volume: number;
  time: number;
}
