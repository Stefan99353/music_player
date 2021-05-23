import {Injectable} from '@angular/core';
import {environment} from '../../../../environments/environment';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {RodioPlayerState} from '../../../models/rodio-player-state';

@Injectable({
  providedIn: 'root'
})
export class PlayerService {
  baseUrl = environment.baseUrl + 'player';

  constructor(private http: HttpClient) {
  }

  state(): Observable<RodioPlayerState> {
    return this.http.get<RodioPlayerState>(this.baseUrl);
  }

  resume(): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/resume', undefined);
  }

  pause(): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/pause', undefined);
  }

  shuffle(shuffle: boolean): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/shuffle', undefined);
  }

  stop(): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/stop', undefined);
  }

  next(): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/next', undefined);
  }

  prev(): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/prev', undefined);
  }

  setVolume(volume: number): Observable<void> {
    return this.http.post<void>(this.baseUrl + '/volume', {volume});
  }

  getVolume(): Observable<number> {
    return this.http.get<number>(this.baseUrl + '/volume');
  }
}
