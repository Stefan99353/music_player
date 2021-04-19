import {Injectable} from '@angular/core';
import {Observable, Subject} from 'rxjs';

export enum StorageField {
  GRID = 'grid',
}

export interface StorageItem {
  key: StorageField;
  value: string;
}

@Injectable({
  providedIn: 'root'
})
export class StorageService {
  private storageSubject = new Subject<StorageItem>();

  constructor() {
    if (localStorage.getItem(StorageField.GRID) === null) {
      localStorage.setItem(StorageField.GRID, 'true');
    }
  }

  save(item: StorageItem): void {
    localStorage.setItem(item.key, item.value);
    this.storageSubject.next(item);
  }

  next(key: StorageField): void {
    const value = localStorage.getItem(key);
    if (value !== null) {
      this.storageSubject.next({key, value});
    }
  }

  storageObservable(): Observable<StorageItem> {
    return this.storageSubject.asObservable();
  }
}
