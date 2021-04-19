import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {Observable} from 'rxjs';
import {HttpClient, HttpParams} from '@angular/common/http';
import {Track} from '../../../models/track';
import {PaginationResult} from '../../../models/pagination-result';

@Injectable({
  providedIn: 'root'
})
export class QueueService {
  baseUrl = environment.baseUrl + 'queue';

  constructor(private http: HttpClient) {
  }

  getQueue(): Observable<Track[]> {
    return this.http.get<Track[]>(this.baseUrl);
  }

  addTracks(ids: number[], shuffle: boolean = false): Observable<void> {
    const params = new HttpParams()
      .set('shuffle', shuffle.toString());

    return this.http.post<void>(this.baseUrl + '/add', ids, {params});
  }

  addAlbum(id: number, shuffle: boolean = false): Observable<void> {
    const params = new HttpParams()
      .set('shuffle', shuffle.toString());

    return this.http.post<void>(this.baseUrl + '/addAlbum', id, {params});
  }

  addArtist(id: number, shuffle: boolean = false): Observable<void> {
    const params = new HttpParams()
      .set('shuffle', shuffle.toString());

    return this.http.post<void>(this.baseUrl + '/addArtist', id, {params});
  }

  getTracks(
    sort?: string,
    order?: string,
    page?: number,
    limit?: number
  ): Observable<PaginationResult<Track>> {
    let params = new HttpParams();
    if (sort) {
      params = params.set('sort', sort);
    }
    if (order) {
      params = params.set('order', order);
    }
    if (page) {
      params = params.set('page', page.toString());
    }
    if (limit) {
      params = params.set('limit', limit.toString());
    }

    return this.http.get<PaginationResult<Track>>(this.baseUrl, {params});
  }

  clear(): Observable<void> {
    return this.http.delete<void>(this.baseUrl);
  }
}
