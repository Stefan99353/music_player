import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {StorageField, StorageService} from '../../core/io/storage/storage.service';
import {QueueService} from '../../core/io/queue/queue.service';

@Component({
  selector: 'app-albums',
  templateUrl: './albums.component.html',
  styleUrls: ['./albums.component.scss']
})
export class AlbumsComponent implements OnInit {
  artistId: number | null = null;
  grid = true;

  constructor(
    private route: ActivatedRoute,
    private router: Router,
    private storageService: StorageService,
    private queueService: QueueService,
  ) {
  }

  ngOnInit(): void {
    this.route.params.subscribe(value => {
      this.artistId = !isNaN(parseInt(value.artistId, 10)) ? parseInt(value.artistId, 10) : null;
    });

    this.storageService.storageObservable().subscribe(value => {
      if (value.key === StorageField.GRID) {
        this.grid = JSON.parse(value.value);
      }
    });
    this.storageService.next(StorageField.GRID);
  }

  albumClicked(albumId: number): void {
    if (albumId >= 0) {
      this.router.navigate(['albums', albumId, 'tracks']);
    } else {
      if (this.artistId !== null && this.artistId >= 0) {
        this.router.navigate(['artists', this.artistId, 'tracks']);
      } else {
        this.router.navigate(['tracks']);
      }
    }
  }

  addAlbum(albumId: number): void {
    this.queueService.addAlbum(albumId).subscribe();
  }
}
