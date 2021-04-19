import {AfterViewInit, Component, EventEmitter, Output, ViewChild} from '@angular/core';
import {Track} from '../../../models/track';
import {merge, Observable, of} from 'rxjs';
import {MatPaginator} from '@angular/material/paginator';
import {MatSort} from '@angular/material/sort';
import {TrackService} from '../../io/track/track.service';
import {catchError, map, startWith, switchMap} from 'rxjs/operators';
import {MatTable} from '@angular/material/table';
import {ActivatedRoute} from '@angular/router';

@Component({
  selector: 'app-track-list',
  templateUrl: './track-list.component.html',
  styleUrls: ['./track-list.component.scss']
})
export class TrackListComponent implements AfterViewInit {
  artistId?: number;
  albumId?: number;

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

  constructor(
    private route: ActivatedRoute,
    private trackService: TrackService,
  ) {
  }

  ngAfterViewInit(): void {
    this.route.params.subscribe(params => {
      this.albumId = params.albumId;
      this.artistId = params.artistId;

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
            return this.trackService.getTracks(
              this.artistId,
              this.albumId,
              this.filter,
              this.sort.active,
              this.sort.direction,
              this.paginator.pageIndex,
              this.pageSize
            );
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
    });
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
}
