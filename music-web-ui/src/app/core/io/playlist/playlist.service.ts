import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {HttpClient} from '@angular/common/http';
import {buildParams, RequestFilter} from '../../../models/request-filter';
import {Observable} from 'rxjs';
import {PaginationResult} from '../../../models/pagination-result';
import {Playlist} from '../../../models/playlist';
import {Track} from '../../../models/track';

@Injectable({
  providedIn: 'root'
})
export class PlaylistService {
  baseUrl = environment.baseUrl + 'playlists';

  constructor(private http: HttpClient) {
  }

  allPlaylists(filter: RequestFilter): Observable<PaginationResult<Playlist>> {
    const params = buildParams(filter);

    return this.http.get<PaginationResult<Playlist>>(this.baseUrl, {params});
  }

  addPlaylist(newPlaylist: Playlist): Observable<Playlist> {
    return this.http.post<Playlist>(this.baseUrl, newPlaylist);
  }

  getPlaylist(playlistId: number): Observable<Playlist> {
    return this.http.get<Playlist>(this.baseUrl + '/' + playlistId);
  }

  updatePlaylist(playlistId: number, playlist: Playlist): Observable<Playlist> {
    return this.http.put<Playlist>(this.baseUrl + '/' + playlist.id, playlist);
  }

  deletePlaylist(playlistId: number): Observable<Playlist> {
    return this.http.delete<Playlist>(this.baseUrl + '/' + playlistId);
  }

  allTracks(playlistId: number, filter: RequestFilter): Observable<PaginationResult<Track>> {
    const params = buildParams(filter);

    return this.http.get<PaginationResult<Track>>(this.baseUrl + '/' + playlistId + '/tracks', {params});
  }

  addTrack(playlistId: number, trackId: number): Observable<any> {
    return this.http.post<any>(this.baseUrl + '/' + playlistId + '/tracks/' + trackId, null);
  }

  deleteTrack(playlistId: number, trackId: number): Observable<void> {
    return this.http.delete<void>(this.baseUrl + '/' + playlistId + '/tracks/' + trackId);
  }
}
