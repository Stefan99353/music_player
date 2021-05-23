import {Component, OnDestroy, OnInit, ViewChild} from '@angular/core';
import {RodioPlayerState} from '../../../models/rodio-player-state';
import {environment} from '../../../../environments/environment';
import {MatDialog} from '@angular/material/dialog';
import {QueueComponent} from '../../dialogs/queue/queue.component';
import {webSocket, WebSocketSubject} from 'rxjs/webSocket';

interface RodioCommandMessage {
  command: 'Resume' | 'Pause' | 'Stop' | 'Next' | 'Prev' | 'State' | { Volume: number } | { Seek: number };
}

@Component({
  selector: 'app-player-control',
  templateUrl: './player-control.component.html',
  styleUrls: ['./player-control.component.scss'],
})
export class PlayerControlComponent implements OnInit, OnDestroy {
  imageUrl = environment.baseUrl + 'images/';

  wsSubject: WebSocketSubject<RodioPlayerState | RodioCommandMessage> | null = null;
  progressLoop: number | null = null;

  rodioPlayerState: RodioPlayerState = {
    currentTrack: null,
    paused: false,
    volume: 0.5,
    time: 0,
  };

  imageId: number | null = null;
  private previousVolume: number | null = null;

  @ViewChild('audioPlayer') audioPlayer!: HTMLAudioElement;

  constructor(private dialog: MatDialog) {
  }

  ngOnInit(): void {
    this.wsSubject = webSocket(environment.wsPlayerUrl);

    this.wsSubject.subscribe(value => {
      this.rodioPlayerState = value as RodioPlayerState;

      if (this.progressLoop !== null) {
        clearInterval(this.progressLoop);
      }

      if (
        this.rodioPlayerState &&
        this.rodioPlayerState.currentTrack !== null &&
        !this.rodioPlayerState.paused
      ) {
        // @ts-ignore
        this.progressLoop = setInterval(() => {
          if (this.rodioPlayerState) {
            this.rodioPlayerState.time += 250;
          }
        }, 250);
      }

      // Check image
      if (this.rodioPlayerState.currentTrack !== null) {
        this.imageId = this.rodioPlayerState.currentTrack.imageId;
      }
    });
  }

  progressThumbLabel(value: number): string {
    const totalSecs = value / 1000;
    const mins = Math.floor(totalSecs / 60);
    const secs = Math.floor(totalSecs % 60);

    if (secs < 10) {
      return mins + ':0' + secs;
    } else {
      return mins + ':' + secs;
    }
  }

  resumePause(): void {
    if (this.wsSubject === null) {
      return;
    }

    if (this.rodioPlayerState.paused) {
      this.wsSubject.next({command: 'Resume'});
    } else {
      this.wsSubject.next({command: 'Pause'});
    }
  }

  stop(): void {
    if (this.wsSubject === null) {
      return;
    }

    this.wsSubject.next({command: 'Stop'});
  }

  prev(): void {
    if (this.wsSubject === null) {
      return;
    }

    this.wsSubject.next({command: 'Prev'});
  }

  next(): void {
    if (this.wsSubject === null) {
      return;
    }

    this.wsSubject.next({command: 'Next'});
  }

  setVolume(volume: number | null): void {
    if (this.wsSubject === null) {
      return;
    }

    if (volume !== null) {
      this.wsSubject.next({command: {Volume: volume}});
    }
  }

  toggleVolume(): void {
    if (this.rodioPlayerState.volume === 0) {
      this.setVolume(this.previousVolume);
    } else {
      this.previousVolume = this.rodioPlayerState.volume;
      this.setVolume(0);
    }
  }

  openQueue(): void {
    this.dialog.open(QueueComponent, {
      width: '100%'
    });
  }

  ngOnDestroy(): void {
    if (this.wsSubject !== null) {
      this.wsSubject.complete();
    }
  }

  seek_to(to: number | null): void {
    if (to !== null && this.wsSubject !== null) {
      this.wsSubject.next({command: {Seek: to}});
    }
  }
}
