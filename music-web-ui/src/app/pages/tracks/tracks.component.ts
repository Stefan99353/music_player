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

  artistId: number | null = null;
  albumId: number | null = null;
  playlistId: number | null = null;

  constructor(
    private queueService: QueueService,
    private route: ActivatedRoute
  ) {
  }

  ngOnInit(): void {
    this.route.params.subscribe(value => {
      this.artistId = !isNaN(parseInt(value.artistId, 10)) ? parseInt(value.artistId, 10) : null;
      this.albumId = !isNaN(parseInt(value.albumId, 10)) ? parseInt(value.albumId, 10) : null;
      this.playlistId = !isNaN(parseInt(value.playlistId, 10)) ? parseInt(value.playlistId, 10) : null;
    });
  }

  addTrack(id: number): void {
    this.queueService.addTracks([id]).subscribe();
  }
}
