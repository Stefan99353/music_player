import {Component, OnInit} from '@angular/core';
import {Router} from '@angular/router';
import {StorageField, StorageService} from '../../core/io/storage/storage.service';
import {QueueService} from "../../core/io/queue/queue.service";

@Component({
  selector: 'app-artists',
  templateUrl: './artists.component.html',
  styleUrls: ['./artists.component.scss']
})
export class ArtistsComponent implements OnInit {
  grid = true;

  constructor(
    private router: Router,
    private storageService: StorageService,
    private queueService: QueueService,
  ) {
  }

  ngOnInit(): void {
    this.storageService.storageObservable().subscribe(value => {
      if (value.key === StorageField.GRID) {
        this.grid = JSON.parse(value.value);
      }
    });
    this.storageService.next(StorageField.GRID);
  }

  artistClicked(artistId: number): void {
    if (artistId >= 0) {
      this.router.navigate(['artists', artistId, 'albums']);
    } else {
      this.router.navigate(['albums']);
    }
  }

  addArtist(artistId: number, shuffle: boolean): void {
    this.queueService.addArtist(artistId, shuffle).subscribe();
  }
}
