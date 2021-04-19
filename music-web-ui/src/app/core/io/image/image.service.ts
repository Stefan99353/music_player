import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class ImageService {
  baseUrl = environment.baseUrl + 'images';

  constructor(private http: HttpClient) {
  }

  get_album_image_id(albumId: number): Observable<number> {
    return this.http.get<number>(this.baseUrl + '/album/' + albumId + '/id');
  }
}
