import {Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {environment} from '../../../../environments/environment';
import {Observable} from 'rxjs';
import {Artist} from '../../../models/artist';
import {PaginationResult} from '../../../models/pagination-result';
import {buildParams, RequestFilter} from '../../../models/request-filter';
import {Album} from '../../../models/album';
import {Track} from '../../../models/track';

@Injectable({
  providedIn: 'root'
})
export class ArtistService {
  baseUrl = environment.baseUrl + 'artists';

  constructor(private http: HttpClient) {
  }

  allArtists(filter: RequestFilter): Observable<PaginationResult<Artist>> {
    const params = buildParams(filter);

    return this.http.get<PaginationResult<Artist>>(this.baseUrl, {params});
  }

  addArtist(newArtist: Artist): Observable<Artist> {
    return this.http.post<Artist>(this.baseUrl, newArtist);
  }

  getArtist(artistId: number): Observable<Artist> {
    return this.http.get<Artist>(this.baseUrl + '/' + artistId);
  }

  updateArtist(artist: Artist): Observable<Artist> {
    return this.http.put<Artist>(this.baseUrl + '/' + artist.id, artist);
  }

  deleteArtist(artistId: number): Observable<Artist> {
    return this.http.delete<Artist>(this.baseUrl + '/' + artistId);
  }

  allAlbums(artistId: number, filter: RequestFilter): Observable<PaginationResult<Album>> {
    const params = buildParams(filter);

    return this.http.get<PaginationResult<Album>>(this.baseUrl + '/' + artistId + '/albums', {params});
  }

  allTracks(artistId: number, filter: RequestFilter): Observable<PaginationResult<Track>> {
    const params = buildParams(filter);

    return this.http.get<PaginationResult<Track>>(this.baseUrl + '/' + artistId + '/tracks', {params});
  }

  addImage(artistId: number): Observable<any> {
    // TODO: Implement adding image
    return this.http.post<any>(this.baseUrl + '/' + artistId + '/image', {});
  }

  deleteImage(artistId: number): Observable<any> {
    // TODO: Implement removing image
    return this.http.delete<any>(this.baseUrl + '/' + artistId + '/image');
  }
}
