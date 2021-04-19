import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class ManagementService {
  baseUrl = environment.baseUrl + 'management';

  constructor(private http: HttpClient) {
  }

  updateDb(): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/update_db', undefined);
  }

  rebuildDb(): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/rebuild_db', undefined);
  }

  get_updates(): Observable<any[]> {
    return this.http.get<any[]>(this.baseUrl + '/updates');
  }
}
