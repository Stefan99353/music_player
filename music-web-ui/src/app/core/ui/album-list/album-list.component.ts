import {Component, EventEmitter, Input, OnInit, Output} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {Album} from '../../../models/album';
import {AlbumService} from '../../io/album/album.service';
import {ActivatedRoute} from '@angular/router';

@Component({
  selector: 'app-album-list',
  templateUrl: './album-list.component.html',
  styleUrls: ['./album-list.component.scss']
})
export class AlbumListComponent implements OnInit {
  imageUrl = environment.baseUrl + 'images/';
  artistId?: number;
  albums: Album[] = [];

  @Output() albumClicked: EventEmitter<number> = new EventEmitter<number>();
  @Output() addAlbum: EventEmitter<number> = new EventEmitter<number>();
  @Output() shuffleAlbum: EventEmitter<number> = new EventEmitter<number>();
  @Input() displayAsGrid = true;

  constructor(
    private route: ActivatedRoute,
    private albumService: AlbumService
  ) {
  }

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      this.artistId = params.artistId;

      if (this.artistId && this.artistId >= 0) {
        this.albumService.get_albums(this.artistId).subscribe(value => {
          this.albums = value.items;
        });
      } else {
        this.albumService.get_albums().subscribe(value => {
          this.albums = value.items;
        });
      }
    });
  }
}
