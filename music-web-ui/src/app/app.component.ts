import {Component, OnInit} from '@angular/core';
import {MatSlideToggleChange} from '@angular/material/slide-toggle';
import {StorageField, StorageService} from './core/io/storage/storage.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = '';
  grid = true;

  constructor(private storageService: StorageService) {
  }

  ngOnInit(): void {
    this.storageService.storageObservable().subscribe(value => {
      if (value.key === StorageField.GRID) {
        this.grid = JSON.parse(value.value);
      }
    });
    this.storageService.next(StorageField.GRID);
  }

  toggleGrid($event: MatSlideToggleChange): void {
    this.storageService.save({key: StorageField.GRID, value: JSON.stringify($event.checked)});
  }
}
