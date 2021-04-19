import {Track} from './track';

export class RodioPlayerState {
  currentTrack?: Track | undefined;
  currentIndex: number;
  currentlyPlaying: boolean;
  paused: boolean;
  volume: number;
  time: number;


  constructor(
    currentTrack: Track | undefined,
    currentIndex: number,
    currentlyPlaying: boolean,
    paused: boolean,
    volume: number,
    time: number
  ) {
    this.currentTrack = currentTrack;
    this.currentIndex = currentIndex;
    this.currentlyPlaying = currentlyPlaying;
    this.paused = paused;
    this.volume = volume;
    this.time = time;
  }
}
