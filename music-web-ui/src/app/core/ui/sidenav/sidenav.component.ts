import {Component, EventEmitter, OnInit, Output} from '@angular/core';
import {Router} from '@angular/router';
import {ManagementService} from '../../io/management/management.service';
import {PlaylistService} from "../../io/playlist/playlist.service";
import {Playlist} from "../../../models/playlist";

@Component({
  selector: 'app-sidenav',
  templateUrl: './sidenav.component.html',
  styleUrls: ['./sidenav.component.scss']
})
export class SidenavComponent implements OnInit {
  playlists: Playlist[] = [];

  @Output() closeNavBar: EventEmitter<void> = new EventEmitter<void>();

  links: any[] = [
    {
      name: 'Artists',
      icon: 'face',
      href: 'artists',
    },
    {
      name: 'Albums',
      icon: 'albums',
      href: 'albums',
    },
    {
      name: 'Tracks',
      icon: 'audiotrack',
      href: 'tracks',
    },
    {
      name: 'Playlists',
      icon: 'queue_music',
      href: 'playlists',
    }
  ];

  constructor(
    private router: Router,
    private managementService: ManagementService,
    private playlistService: PlaylistService,
  ) {
  }

  ngOnInit(): void {
    this.playlistService.allPlaylists({}).subscribe(value => {
      this.playlists = value.items;
    });
  }

  navigate(href: string): void {
    this.router.navigateByUrl(href);
    this.closeNavBar.emit();
  }

  navigatePlaylist(playlistId: number): void {
    this.router.navigate(['playlists', playlistId, 'tracks']);
    this.closeNavBar.emit();
  }

  updateDb(): void {
    this.managementService.updateDb().subscribe();

    // TODO: Show Popup
  }
}
