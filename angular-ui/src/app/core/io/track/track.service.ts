import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {HttpClient, HttpParams} from '@angular/common/http';
import {Observable} from 'rxjs';
import {Track} from '../../../models/track';
import {PaginationResult} from '../../../models/pagination-result';
import {buildParams, RequestFilter} from '../../../models/request-filter';

@Injectable({
  providedIn: 'root'
})
export class TrackService {
  baseUrl = environment.baseUrl + 'tracks';

  constructor(private http: HttpClient) {
  }

  allTracks(filter: RequestFilter): Observable<PaginationResult<Track>> {
    const params = buildParams(filter);

    return this.http.get<PaginationResult<Track>>(this.baseUrl, {params});
  }

  addTrack(newTrack: Track): Observable<Track> {
    return this.http.post<Track>(this.baseUrl, newTrack);
  }

  getTrack(trackId: number): Observable<Track> {
    return this.http.get<Track>(this.baseUrl + '/' + trackId);
  }

  updateTrack(track: Track): Observable<Track> {
    return this.http.put<Track>(this.baseUrl + '/' + track.id, track);
  }

  deleteTrack(trackId: number): Observable<Track> {
    return this.http.delete<Track>(this.baseUrl + '/' + trackId);
  }

  streamTrack(trackId: number): Observable<any> {
    return this.http.get<any>(this.baseUrl + '/' + trackId + '/stream');
  }

  addImage(trackId: number): Observable<any> {
    // TODO: Implement adding image
    return this.http.post<any>(this.baseUrl + '/' + trackId + '/image', {});
  }

  deleteImage(trackId: number): Observable<any> {
    // TODO: Implement removing image
    return this.http.delete<any>(this.baseUrl + '/' + trackId + '/image');
  }
}
