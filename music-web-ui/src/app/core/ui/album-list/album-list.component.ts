import {Component, EventEmitter, Input, OnInit, Output} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {Album} from '../../../models/album';
import {AlbumService} from '../../io/album/album.service';
import {ActivatedRoute} from '@angular/router';
import {ArtistService} from '../../io/artist/artist.service';
import {RequestFilter} from '../../../models/request-filter';

@Component({
  selector: 'app-album-list',
  templateUrl: './album-list.component.html',
  styleUrls: ['./album-list.component.scss']
})
export class AlbumListComponent implements OnInit {
  imageUrl = environment.baseUrl + 'images/';
  artistId: number | null = null;
  albums: Album[] = [];

  @Output() albumClicked: EventEmitter<number> = new EventEmitter<number>();
  @Output() addAlbum: EventEmitter<number> = new EventEmitter<number>();
  @Output() shuffleAlbum: EventEmitter<number> = new EventEmitter<number>();
  @Input() displayAsGrid = true;

  constructor(
    private route: ActivatedRoute,
    private albumService: AlbumService,
    private artistService: ArtistService,
  ) {
  }

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      this.artistId = params.artistId;

      const filter: RequestFilter = {
        filter: null, limit: null, order: null, page: null, sort: null
      };

      if (this.artistId !== null) {
        this.artistService.allAlbums(this.artistId, filter).subscribe(value => {
          this.albums = value.items;
        });
      } else {
        this.albumService.allAlbums(filter).subscribe(value => {
          this.albums = value.items;
        });
      }
    });
  }
}
