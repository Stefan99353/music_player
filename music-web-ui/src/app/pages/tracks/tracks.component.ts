import {Component, OnInit} from '@angular/core';
import {environment} from '../../../environments/environment';
import {ActivatedRoute} from '@angular/router';
import {QueueService} from '../../core/io/queue/queue.service';

@Component({
  selector: 'app-tracks',
  templateUrl: './tracks.component.html',
  styleUrls: ['./tracks.component.scss']
})
export class TracksComponent implements OnInit {

  imageUrl = environment.baseUrl + 'images/';

  artistId?: number;
  albumId?: number;
  playlistId?: number;

  constructor(
    private queueService: QueueService,
    private route: ActivatedRoute
  ) {
  }

  ngOnInit(): void {
    this.route.params.subscribe(value => {
      this.artistId = !isNaN(parseInt(value.artistId, 10)) ? parseInt(value.artistId, 10) : undefined;
      this.albumId = !isNaN(parseInt(value.albumId, 10)) ? parseInt(value.albumId, 10) : undefined;
      this.playlistId = !isNaN(parseInt(value.playlistId, 10)) ? parseInt(value.playlistId, 10) : undefined;
    });
  }

  addTrack(id: number): void {
    this.queueService.addTracks([id]).subscribe();
  }
}
