import {Component, EventEmitter, Input, OnInit, Output} from '@angular/core';
import {ArtistService} from '../../io/artist/artist.service';
import {Artist} from '../../../models/artist';
import {environment} from '../../../../environments/environment';

@Component({
  selector: 'app-artist-list',
  templateUrl: './artist-list.component.html',
  styleUrls: ['./artist-list.component.scss']
})
export class ArtistListComponent implements OnInit {
  imageUrl = environment.baseUrl + 'images/';
  artists: Artist[] = [];

  @Output() artistClicked: EventEmitter<number> = new EventEmitter<number>();
  @Output() addArtist: EventEmitter<number> = new EventEmitter<number>();
  @Input() displayAsGrid = false;

  constructor(private artistService: ArtistService) {
  }

  ngOnInit(): void {
    this.artistService.allArtists({
      filter: null,
      limit: null,
      order: null,
      page: null,
      sort: null
    }).subscribe(value => {
      this.artists = value.items;
    });
  }
}
