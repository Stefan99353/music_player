import {Component, OnInit} from '@angular/core';
import {Router} from '@angular/router';
import {MatDialog} from '@angular/material/dialog';
import {AddPlaylistDialogComponent} from '../../core/dialogs/add-playlist-dialog/add-playlist-dialog.component';
import {PlaylistService} from '../../core/io/playlist/playlist.service';
import {Playlist} from '../../models/playlist';
import {ConfirmDialogComponent} from '../../core/dialogs/confirm-dialog/confirm-dialog.component';

@Component({
  selector: 'app-playlists',
  templateUrl: './playlists.component.html',
  styleUrls: ['./playlists.component.scss']
})
export class PlaylistsComponent implements OnInit {
  playlists: Playlist[] = [];

  constructor(
    private playlistService: PlaylistService,
    private router: Router,
    private dialog: MatDialog
  ) {
  }

  ngOnInit(): void {
    this.playlistService.allPlaylists({
      filter: null, limit: null, order: null, page: null, sort: null
    }).subscribe(value => {
      this.playlists = value.items;
    });
  }

  playlistClicked(playlistId: number): void {
    this.router.navigate(['playlists', playlistId, 'tracks']);
  }

  openNewDialog(): void {
    const dialogRef = this.dialog.open(AddPlaylistDialogComponent, {
      width: '600px',
      data: {
        new: true,
        playlist: {
          id: 0,
          name: ''
        }
      }
    });

    dialogRef.afterClosed().subscribe(value => {
      if (value !== null && value !== undefined) {
        this.playlists.push(value);
      }
    });
  }

  openEditDialog(playlist: Playlist): void {
    const dialogRef = this.dialog.open(AddPlaylistDialogComponent, {
      width: '600px',
      data: {
        new: false,
        playlist,
      },
    });

    dialogRef.afterClosed().subscribe(value => {
      if (value !== null && value !== undefined) {
        this.playlists.splice(
          this.playlists.indexOf(playlist),
          1,
          value
        );
      }
    });
  }

  openConfirmDeleteDialog(playlistId: number): void {
    const dialogRef = this.dialog.open(ConfirmDialogComponent, {
      width: '300px'
    });

    dialogRef.afterClosed().subscribe(confirmed => {
      if (confirmed === true) {
        this.playlistService.deletePlaylist(playlistId).subscribe(() => {
          this.playlists = this.playlists.filter(x => x.id !== playlistId);
        });
      }
    });
  }
}
