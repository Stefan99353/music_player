import {Component, EventEmitter, Input, OnInit, Output} from '@angular/core';
import {Playlist} from '../../../models/playlist';
import {PlaylistService} from '../../io/playlist/playlist.service';

@Component({
  selector: 'app-playlist-list',
  templateUrl: './playlist-list.component.html',
  styleUrls: ['./playlist-list.component.scss']
})
export class PlaylistListComponent implements OnInit {
  @Input() playlists: Playlist[] = [];
  @Output() playlistClicked: EventEmitter<number> = new EventEmitter<number>();
  @Output() editClicked: EventEmitter<Playlist> = new EventEmitter<Playlist>();
  @Output() deleteClicked: EventEmitter<number> = new EventEmitter<number>();

  constructor() { }

  ngOnInit(): void {
  }
}
