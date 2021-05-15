import {Component, Inject, OnInit} from '@angular/core';
import {MAT_DIALOG_DATA, MatDialogRef} from '@angular/material/dialog';
import {PlaylistService} from '../../io/playlist/playlist.service';
import {Playlist} from '../../../models/playlist';
import {NotifierService} from "angular-notifier";

export interface AddPlaylistDialogData {
  new: boolean;
  playlist: Playlist;
}

@Component({
  selector: 'app-add-playlist-dialog',
  templateUrl: './add-playlist-dialog.component.html',
  styleUrls: ['./add-playlist-dialog.component.scss']
})
export class AddPlaylistDialogComponent implements OnInit {
  id = 0;
  name = '';
  icon = '';
  description = '';
  new = true;

  constructor(
    public dialogRef: MatDialogRef<AddPlaylistDialogComponent>,
    @Inject(MAT_DIALOG_DATA) public data: AddPlaylistDialogData,
    private playlistService: PlaylistService,
    private notifierService: NotifierService
  ) {
    this.id = data.playlist.id;
    this.name = data.playlist.name;
    this.icon = data.playlist.icon || '';
    this.description = data.playlist.description || '';

    this.new = data.new;
  }

  ngOnInit(): void {
  }

  onNoClick(): void {
    this.dialogRef.close();
  }

  addPlaylist(): void {
    if (this.name.length > 0) {
      this.playlistService.addPlaylist({
        id: 0,
        name: this.name,
        icon: this.icon || undefined,
        description: this.description || undefined,
      }).subscribe(value => {
        this.dialogRef.close(value);
      });
    } else {
      this.notifierService.notify('default', 'Name needs to be at least 1 character long');
    }
  }

  updatePlaylist(): void {
    if (this.name.length > 0) {
      this.playlistService.updatePlaylist(this.id, {
        id: 0,
        name: this.name,
        icon: this.icon,
        description: this.description
      }).subscribe(value => {
        this.dialogRef.close(value);
      });
    } else {
      this.notifierService.notify('default', 'Name needs to be at least 1 character long');
    }
  }
}
