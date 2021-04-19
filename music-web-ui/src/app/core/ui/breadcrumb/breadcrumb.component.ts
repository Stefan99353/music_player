import {Component, OnInit} from '@angular/core';
import {NavigationEnd, Router} from '@angular/router';
import {filter} from 'rxjs/operators';
import {Artist} from "../../../models/artist";
import {Album} from "../../../models/album";
import {ArtistService} from "../../io/artist/artist.service";
import {AlbumService} from "../../io/album/album.service";

@Component({
  selector: 'app-breadcrumb',
  templateUrl: './breadcrumb.component.html',
  styleUrls: ['./breadcrumb.component.scss']
})
export class BreadcrumbComponent implements OnInit {

  artistId?: number;
  albumId?: number;
  artist?: Artist;
  album?: Album;

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
  ) {
  }

  ngOnInit(): void {
    this.router.events
      .pipe(filter(event => event instanceof NavigationEnd))
      .subscribe(e => {
        const value = e as NavigationEnd;
        this.artistId = undefined;
        this.albumId = undefined;
        this.artist = undefined;
        this.album = undefined;

        const url = value.url.substr(1).split('/');

        for (let i = 0; i < url.length; i++) {
          if (url[i] === 'artists' && parseInt(url[i + 1], 10)) {
            this.artistId = parseInt(url[i + 1], 10) || undefined;
          }

          if (url[i] === 'albums' && parseInt(url[i + 1], 10)) {
            this.albumId = parseInt(url[i + 1], 10) || undefined;
          }
        }

        if (this.artistId !== undefined) {
          this.artistService.get_artist(this.artistId).subscribe(art => {
            this.artist = art;
            this.buildBreadCrumb();
          });
        }

        if (this.albumId !== undefined) {
          this.albumService.get_album(this.albumId).subscribe(alb => {
            this.album = alb;
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
      if (this.artist === undefined) {
        this.elements.push({
          name: this.album.title,
          link: 'albums/' + this.album.id + '/tracks',
        });
      } else {
        this.elements.push({
          name: this.album.title,
          link: 'artists/' + this.artist.id + '/albums/' + this.album.id + '/tracks',
        });
      }
    }
  }
}
