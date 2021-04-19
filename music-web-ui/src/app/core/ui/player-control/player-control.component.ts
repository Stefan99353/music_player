import {Component, OnDestroy, OnInit, ViewChild} from '@angular/core';
import {RodioPlayerState} from '../../../models/rodio-player-state';
import {environment} from '../../../../environments/environment';
import {MatDialog} from '@angular/material/dialog';
import {QueueComponent} from '../../dialogs/queue/queue.component';
import {webSocket, WebSocketSubject} from 'rxjs/webSocket';
import {ImageService} from '../../io/image/image.service';


@Component({
  selector: 'app-player-control',
  templateUrl: './player-control.component.html',
  styleUrls: ['./player-control.component.scss'],
})
export class PlayerControlComponent implements OnInit, OnDestroy {
  imageUrl = environment.baseUrl + 'images/';

  wsSubject?: WebSocketSubject<any>;
  progressLoop?: number;

  rodioPlayerState: RodioPlayerState = new RodioPlayerState(undefined, 0, false, false, 0.5, 0);
  imageId?: number;
  private previousVolume?: number;

  @ViewChild('audioPlayer') audioPlayer!: HTMLAudioElement;

  constructor(
    public dialog: MatDialog,
    private imageService: ImageService) {
  }

  ngOnInit(): void {
    this.wsSubject = webSocket(environment.wsUrl);

    this.wsSubject.subscribe(value => {
      this.rodioPlayerState = value;

      clearInterval(this.progressLoop);
      if (this.rodioPlayerState && this.rodioPlayerState.currentlyPlaying && !this.rodioPlayerState.paused) {
        // @ts-ignore
        this.progressLoop = setInterval(() => {
          if (this.rodioPlayerState) {
            this.rodioPlayerState.time += 250;
          }
        }, 250);
      }

      // Check image
      if (this.rodioPlayerState.currentTrack) {
        this.imageId = this.rodioPlayerState.currentTrack.imageId;

        if (this.imageId === null || this.imageId === undefined) {
          this.imageService.get_album_image_id(this.rodioPlayerState.currentTrack.albumId).subscribe(imgId => {
            this.imageId = imgId !== null ? imgId : undefined;
          });
        }
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
    if (!this.rodioPlayerState) {
      return;
    }

    if (this.rodioPlayerState?.paused) {
      this.wsSubject?.next({command: 'Resume'});
    } else {
      this.wsSubject?.next({command: 'Pause'});
    }
  }

  stop(): void {
    this.wsSubject?.next({command: 'Stop'});
  }

  prev(): void {
    this.wsSubject?.next({command: 'Prev'});
  }

  next(): void {
    this.wsSubject?.next({command: 'Next'});
  }

  setVolume(volume?: number | null): void {
    if (volume !== undefined && volume !== null) {
      this.wsSubject?.next({command: {Volume: volume}});
    }
  }

  toggleVolume(): void {
    if (this.rodioPlayerState?.volume === 0) {
      this.setVolume(this.previousVolume);
    } else {
      this.previousVolume = this.rodioPlayerState?.volume;
      this.setVolume(0);
    }
  }

  openQueue(): void {
    const dialogRef = this.dialog.open(QueueComponent, {
      width: '100%',
      data: this.rodioPlayerState?.currentIndex
    });
  }

  ngOnDestroy(): void {
    this.wsSubject?.complete();
  }

  seek_to(to: number | null): void {
    if (to !== null) {
      this.wsSubject?.next({command: {Seek: to}});
    }
  }
}
