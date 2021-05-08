import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {HttpClient, HttpParams} from '@angular/common/http';
import {Observable} from 'rxjs';
import {Album} from '../../../models/album';
import {PaginationResult} from '../../../models/pagination-result';
import {buildParams, RequestFilter} from '../../../models/request-filter';
import {Track} from '../../../models/track';

@Injectable({
  providedIn: 'root'
})
export class AlbumService {
  baseUrl = environment.baseUrl + 'albums';

  constructor(private http: HttpClient) {
  }

  allAlbums(filter: RequestFilter): Observable<PaginationResult<Album>> {
    const params = buildParams(filter);

    return this.http.get<PaginationResult<Album>>(this.baseUrl, {params});
  }

  addAlbum(newAlbum: Album): Observable<Album> {
    return this.http.post<Album>(this.baseUrl, newAlbum);
  }

  getAlbum(albumId: number): Observable<Album> {
    return this.http.get<Album>(this.baseUrl + '/' + albumId);
  }

  updateAlbum(album: Album): Observable<Album> {
    return this.http.put<Album>(this.baseUrl + '/' + album.id, album);
  }

  deleteAlbum(albumId: number): Observable<Album> {
    return this.http.delete<Album>(this.baseUrl + '/' + albumId);
  }

  allTracks(albumId: number, filter: RequestFilter): Observable<PaginationResult<Track>> {
    const params = buildParams(filter);

    return this.http.get<PaginationResult<Track>>(this.baseUrl + '/' + albumId + '/tracks', {params});
  }

  addImage(albumId: number): Observable<any> {
    // TODO: Implement adding image
    return this.http.post<any>(this.baseUrl + '/' + albumId + '/image', {});
  }

  deleteImage(albumId: number): Observable<any> {
    // TODO: Implement removing image
    return this.http.delete<any>(this.baseUrl + '/' + albumId + '/image');
  }
}
