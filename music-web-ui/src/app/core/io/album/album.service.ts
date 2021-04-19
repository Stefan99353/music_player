import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {HttpClient, HttpParams} from '@angular/common/http';
import {Observable} from 'rxjs';
import {Album} from '../../../models/album';
import {PaginationResult} from '../../../models/pagination-result';

@Injectable({
  providedIn: 'root'
})
export class AlbumService {
  baseUrl = environment.baseUrl + 'albums';

  constructor(private http: HttpClient) {
  }

  get_album(albumId: number): Observable<Album> {
    return this.http.get<Album>(this.baseUrl + '/' + albumId);
  }

  get_albums(
    artistId?: number,
    filter?: string,
    page?: number,
    limit?: number,
  ): Observable<PaginationResult<Album>> {
    let params = new HttpParams();
    if (artistId) {
      params = params.set('artistId', artistId.toString());
    }
    if (filter) {
      params = params.set('filter', filter);
    }
    if (page) {
      params = params.set('page', page.toString());
    }
    if (limit) {
      params = params.set('limit', limit.toString());
    }

    return this.http.get<PaginationResult<Album>>(this.baseUrl, {params});
  }
}
