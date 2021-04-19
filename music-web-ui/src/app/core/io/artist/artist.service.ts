import {Injectable} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../../environments/environment';
import {Observable} from 'rxjs';
import {Artist} from '../../../models/artist';
import {PaginationResult} from '../../../models/pagination-result';

@Injectable({
  providedIn: 'root'
})
export class ArtistService {
  baseUrl = environment.baseUrl + 'artists';

  constructor(private http: HttpClient) {
  }

  get_artist(artistId: number): Observable<Artist> {
    return this.http.get<Artist>(this.baseUrl + '/' + artistId);
  }

  get_artists(
    filter?: string,
    page?: number,
    limit?: number,
  ): Observable<PaginationResult<Artist>> {
    let params = new HttpParams();
    if (filter) {
      params = params.set('filter', filter);
    }
    if (page) {
      params = params.set('page', page.toString());
    }
    if (limit) {
      params = params.set('limit', limit.toString());
    }

    return this.http.get<PaginationResult<Artist>>(this.baseUrl, {params});
  }
}
