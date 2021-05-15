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

    return this.http.post<void>(this.baseUrl + '/add/album', id, {params});
  }

  addArtist(id: number, shuffle: boolean = false): Observable<void> {
    const params = new HttpParams()
      .set('shuffle', shuffle.toString());

    return this.http.post<void>(this.baseUrl + '/add/artist', id, {params});
  }

  clear(): Observable<void> {
    return this.http.delete<void>(this.baseUrl);
  }
}
