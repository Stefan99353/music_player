import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {Observable} from 'rxjs';
import {HttpClient} from '@angular/common/http';
import {Track} from '../../../models/track';

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

  addTracks(ids: number[]): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/add', ids);
  }

  addAlbum(id: number): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/add/album', id);
  }

  addArtist(id: number): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/add/artist', id);
  }

  clear(): Observable<void> {
    return this.http.delete<void>(this.baseUrl);
  }
}
