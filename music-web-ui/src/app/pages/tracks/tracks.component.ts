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
  //
  // artistId?: number;
  // albumId?: number;

  constructor(
    private queueService: QueueService,
    private route: ActivatedRoute
  ) {
  }

  ngOnInit(): void {

  }

  addTrack(id: number): void {
    this.queueService.addTracks([id]).subscribe();
  }
}
