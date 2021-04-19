import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {StorageField, StorageService} from '../../core/io/storage/storage.service';
import {QueueService} from "../../core/io/queue/queue.service";

@Component({
  selector: 'app-albums',
  templateUrl: './albums.component.html',
  styleUrls: ['./albums.component.scss']
})
export class AlbumsComponent implements OnInit {
  artistId?: number;
  grid = true;

  constructor(
    private route: ActivatedRoute,
    private router: Router,
    private storageService: StorageService,
    private queueService: QueueService,
  ) {
  }

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      this.artistId = params.artistId;
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
      if (this.artistId && this.artistId >= 0) {
        this.router.navigate(['artists', this.artistId, 'albums', albumId, 'tracks']);
      } else {
        this.router.navigate(['albums', albumId, 'tracks']);
      }
    } else {
      if (this.artistId && this.artistId >= 0) {
        this.router.navigate(['artists', this.artistId, 'tracks']);
      } else {
        this.router.navigate(['tracks']);
      }
    }
  }

  addAlbum(albumId: number, shuffle: boolean): void {
    this.queueService.addAlbum(albumId, shuffle).subscribe();
  }
}
