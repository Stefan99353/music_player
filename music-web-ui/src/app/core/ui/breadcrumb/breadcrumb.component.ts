import {Component, OnInit} from '@angular/core';
import {NavigationEnd, Router} from '@angular/router';
import {filter} from 'rxjs/operators';
import {Artist} from '../../../models/artist';
import {Album} from '../../../models/album';
import {ArtistService} from '../../io/artist/artist.service';
import {AlbumService} from '../../io/album/album.service';
import {Playlist} from '../../../models/playlist';
import {PlaylistService} from '../../io/playlist/playlist.service';

@Component({
  selector: 'app-breadcrumb',
  templateUrl: './breadcrumb.component.html',
  styleUrls: ['./breadcrumb.component.scss'],
})
export class BreadcrumbComponent implements OnInit {

  artistId?: number;
  albumId?: number;
  playlistId?: number;
  artist?: Artist;
  album?: Album;
  playlist?: Playlist;

  elements: any[] = [
    {
      name: 'Music',
      link: 'artists',
    }
  ];

  constructor(
    private router: Router,
    private artistService: ArtistService,
    private albumService: AlbumService,
    private playlistService: PlaylistService,
  ) {
  }

  ngOnInit(): void {
    this.router.events
      .pipe(filter(event => event instanceof NavigationEnd))
      .subscribe(e => {
        const value = e as NavigationEnd;

        this.artistId = undefined;
        this.albumId = undefined;
        this.playlistId = undefined;
        this.artist = undefined;
        this.album = undefined;
        this.playlist = undefined;

        const url = value.url.substr(1).split('/');

        if (url[0] === 'artists' && url[1] !== undefined) {
          this.artistId = !isNaN(parseInt(url[1], 10)) ? parseInt(url[1], 10) : undefined;
        } else if (url[0] === 'albums') {
          this.albumId = !isNaN(parseInt(url[1], 10)) ? parseInt(url[1], 10) : undefined;
        } else if (url[0] === 'playlists') {
          this.playlistId = !isNaN(parseInt(url[1], 10)) ? parseInt(url[1], 10) : undefined;
        }

        if (this.artistId !== undefined) {
          this.artistService.getArtist(this.artistId).subscribe(art => {
            this.artist = art;
            this.buildBreadCrumb();
          });
        }

        if (this.albumId !== undefined) {
          this.albumService.getAlbum(this.albumId).subscribe(alb => {
            this.album = alb;

            this.artistService.getArtist(alb.artistId).subscribe(art => {
              this.artist = art;
              this.buildBreadCrumb();
            });
          });
        }

        if (this.playlistId !== undefined) {
          this.playlistService.getPlaylist(this.playlistId).subscribe(pll => {
            this.playlist = pll;
            this.buildBreadCrumb();
          });
        }

        this.buildBreadCrumb();
      });
  }


  buildBreadCrumb(): void {
    this.elements = [
      {
        name: 'Music',
        link: 'artists',
      }
    ];

    if (this.artist !== undefined) {
      this.elements.push({
        name: this.artist.name,
        link: 'artists/' + this.artist.id + '/albums',
      });
    }

    if (this.album !== undefined) {
      this.elements.push({
        name: this.album.title,
        link: 'albums/' + this.album.id + '/tracks',
      });
    }

    if (this.playlist !== undefined) {
      this.elements.push({
        name: this.playlist.name,
        link: 'playlists/' + this.playlist.id + '/tracks',
      });
    }
  }
}
