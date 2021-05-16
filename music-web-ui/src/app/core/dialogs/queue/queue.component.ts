import {Component, OnInit} from '@angular/core';
import {MatDialogRef} from '@angular/material/dialog';
import {QueueService} from '../../io/queue/queue.service';
import {Track} from '../../../models/track';

@Component({
  selector: 'app-queue',
  templateUrl: './queue.component.html',
  styleUrls: ['./queue.component.scss']
})
export class QueueComponent implements OnInit {

  displayedColumns: string[] = ['title', 'artist_name', 'album_title', 'duration'];

  queue: Track[] = [];

  constructor(
    public dialogRef: MatDialogRef<QueueComponent>,
    private queueService: QueueService,
  ) {
  }

  ngOnInit(): void {
    this.queueService.getQueue().subscribe(tracks => {
      this.queue = tracks;
    });
  }

  close(): void {
    this.dialogRef.close();
  }

  getTotalDuration(): number {
    if (this.queue.length > 0) {
      return this.queue
        .map((x) => x.duration)
        .reduce((acc, currentValue) => acc + currentValue);
    } else {
      return 0;
    }
  }

  clear(): void {
    this.queueService.clear().subscribe();
  }
}
