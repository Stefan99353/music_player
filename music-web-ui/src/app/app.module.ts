import {NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';

import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {ArtistsComponent} from './pages/artists/artists.component';
import {AlbumsComponent} from './pages/albums/albums.component';
import {TracksComponent} from './pages/tracks/tracks.component';
import {MatToolbarModule} from '@angular/material/toolbar';
import {MatIconModule} from '@angular/material/icon';
import {MatButtonModule} from '@angular/material/button';
import {PlayerControlComponent} from './core/ui/player-control/player-control.component';
import {HttpClientModule} from '@angular/common/http';
import {MatSliderModule} from '@angular/material/slider';
import {MatButtonToggleModule} from '@angular/material/button-toggle';
import {MatCardModule} from '@angular/material/card';
import {NgScrollbarModule} from 'ngx-scrollbar';
import {MatListModule} from '@angular/material/list';
import {QueueComponent} from './core/dialogs/queue/queue.component';
import {MatDialogModule} from '@angular/material/dialog';
import {TrackListComponent} from './core/ui/track-list/track-list.component';
import {MatTableModule} from '@angular/material/table';
import {MatProgressSpinnerModule} from '@angular/material/progress-spinner';
import {MatPaginatorModule} from '@angular/material/paginator';
import {MatSortModule} from '@angular/material/sort';
import {DurationPipe} from './core/pipes/duration/duration.pipe';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatRippleModule} from '@angular/material/core';
import {ArtistListComponent} from './core/ui/artist-list/artist-list.component';
import {AlbumListComponent} from './core/ui/album-list/album-list.component';
import {MatSidenavModule} from '@angular/material/sidenav';
import {MatSlideToggleModule} from '@angular/material/slide-toggle';
import {SidenavComponent} from './core/ui/sidenav/sidenav.component';
import {BreadcrumbComponent} from './core/ui/breadcrumb/breadcrumb.component';

@NgModule({
  declarations: [
    AppComponent,
    ArtistsComponent,
    AlbumsComponent,
    TracksComponent,
    PlayerControlComponent,
    QueueComponent,
    TrackListComponent,
    DurationPipe,
    ArtistListComponent,
    AlbumListComponent,
    SidenavComponent,
    BreadcrumbComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    HttpClientModule,
    NgScrollbarModule,
    MatToolbarModule,
    MatIconModule,
    MatButtonModule,
    MatSliderModule,
    MatButtonToggleModule,
    MatCardModule,
    MatListModule,
    MatDialogModule,
    MatTableModule,
    MatProgressSpinnerModule,
    MatPaginatorModule,
    MatSortModule,
    MatInputModule,
    MatFormFieldModule,
    MatRippleModule,
    MatSidenavModule,
    MatSlideToggleModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule {
}
