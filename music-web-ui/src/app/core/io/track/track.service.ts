import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {HttpClient, HttpParams} from '@angular/common/http';
import {Observable} from 'rxjs';
import {Track} from '../../../models/track';
import {PaginationResult} from '../../../models/pagination-result';

@Injectable({
  providedIn: 'root'
})
export class TrackService {
  baseUrl = environment.baseUrl + 'tracks';

  constructor(private http: HttpClient) {
  }

  getTrack(trackId: number): Observable<Track> {
    return this.http.get<Track>(this.baseUrl + '/' + trackId);
  }

  getTracks(
    artistId?: number,
    albumId?: number,
    filter?: string,
    sort?: string,
    order?: string,
    page?: number,
    limit?: number,
  ): Observable<PaginationResult<Track>> {
    let params = new HttpParams();
    if (artistId !== undefined) {
      params = params.set('artistId', artistId.toString());
    }
    if (albumId !== undefined) {
      params = params.set('albumId', albumId.toString());
    }
    if (filter) {
      params = params.set('filter', filter);
    }
    if (sort) {
      params = params.set('sort', sort);
    }
    if (order) {
      params = params.set('order', order);
    }
    if (page !== undefined) {
      params = params.set('page', page.toString());
    }
    if (limit !== undefined) {
      params = params.set('limit', limit.toString());
    }

    return this.http.get<PaginationResult<Track>>(this.baseUrl, {params});
  }
}
