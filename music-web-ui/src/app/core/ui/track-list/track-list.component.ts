import {AfterViewInit, Component, EventEmitter, Input, OnInit, Output, ViewChild} from '@angular/core';
import {Track} from '../../../models/track';
import {merge, Observable, of} from 'rxjs';
import {MatPaginator} from '@angular/material/paginator';
import {MatSort} from '@angular/material/sort';
import {TrackService} from '../../io/track/track.service';
import {catchError, map, startWith, switchMap} from 'rxjs/operators';
import {MatTable} from '@angular/material/table';
import {PaginationResult} from '../../../models/pagination-result';
import {ArtistService} from '../../io/artist/artist.service';
import {AlbumService} from '../../io/album/album.service';
import {RequestFilter} from '../../../models/request-filter';
import {PlaylistService} from '../../io/playlist/playlist.service';
import {MatMenuTrigger} from '@angular/material/menu';

@Component({
  selector: 'app-track-list',
  templateUrl: './track-list.component.html',
  styleUrls: ['./track-list.component.scss'],
})
export class TrackListComponent implements AfterViewInit, OnInit {
  @Input() artistId?: number;
  @Input() albumId?: number;
  @Input() playlistId?: number;

  @Output() trackClicked: EventEmitter<number> = new EventEmitter<number>();

  displayedColumns: string[] = ['title', 'artist_name', 'album_title', 'duration'];
  pageSize = 50;
  filteredAndPagedTracks: Observable<Track[]> = of([]);

  resultsLength = 0;
  isLoadingResults = true;
  filter?: string;

  @ViewChild(MatTable) table!: MatTable<Track[]>;
  @ViewChild(MatPaginator) paginator!: MatPaginator;
  @ViewChild(MatSort) sort!: MatSort;

  @ViewChild(MatMenuTrigger) contextMenu!: MatMenuTrigger;
  contextMenuPosition = {x: '0px', y: '0px'};

  playlists$ = this.playlistService.allPlaylists({});

  constructor(
    private artistService: ArtistService,
    private albumService: AlbumService,
    private trackService: TrackService,
    private playlistService: PlaylistService,
  ) {
  }

  ngOnInit(): void {
  }

  ngAfterViewInit(): void {
    if (this.artistId !== undefined) {
      this.displayedColumns = this.displayedColumns.filter(x => x !== 'artist_name');
    }

    if (this.albumId !== undefined) {
      this.displayedColumns = this.displayedColumns.filter(x => x !== 'artist_name' && x !== 'album_title');
    }

    this.filteredAndPagedTracks = merge(this.sort.sortChange, this.paginator.page)
      .pipe(
        startWith({}),
        switchMap(() => {
          this.isLoadingResults = true;
          return this.allTracks({
            filter: this.filter,
            sort: this.sort.active,
            order: this.sort.direction,
            page: this.paginator.pageIndex,
            limit: this.pageSize
          });
        }),
        map(data => {
          this.isLoadingResults = false;
          this.resultsLength = data.totalCount;

          return data.items;
        }),
        catchError(() => {
          this.isLoadingResults = false;
          return of([]);
        })
      );
  }

  allTracks(filter: RequestFilter): Observable<PaginationResult<Track>> {
    if (this.artistId !== undefined) {
      return this.artistService.allTracks(this.artistId, filter);
    }
    if (this.albumId !== undefined) {
      return this.albumService.allTracks(this.albumId, filter);
    }
    if (this.playlistId !== undefined) {
      return this.playlistService.allTracks(this.playlistId, filter);
    }
    return this.trackService.allTracks(filter);
  }

  resetPaging(): void {
    this.paginator.pageIndex = 0;
  }

  applyFilter($event: KeyboardEvent): void {
    this.filter = ($event.target as HTMLInputElement).value;
    this.sort.sortChange.emit({active: this.sort.active, direction: this.sort.direction});
  }

  clickTrack(trackId: number): void {
    this.trackClicked.emit(trackId);
  }

  onContextMenu(event: MouseEvent, track: Track): void {
    event.preventDefault();
    this.contextMenuPosition.x = event.clientX + 'px';
    this.contextMenuPosition.y = event.clientY - 64 + 'px';
    this.contextMenu.menuData = {track};
    this.contextMenu.menu.focusFirstItem('mouse');
    this.contextMenu.openMenu();
  }

  addTrackPlaylist(trackId: number, playlistId: number): void {
    // TODO: Error handling
    this.playlistService.addTrack(playlistId, trackId).subscribe();
  }

  removeFromPlaylist(trackId: number): void {
    // TODO: Error handling
    if (this.playlistId !== undefined) {
      this.playlistService.deleteTrack(this.playlistId, trackId)
        .subscribe(() => {
          this.sort.sortChange.emit({active: this.sort.active, direction: this.sort.direction});
        });
    }
  }
}
