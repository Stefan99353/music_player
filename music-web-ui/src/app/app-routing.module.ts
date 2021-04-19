import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {AlbumsComponent} from './pages/albums/albums.component';
import {ArtistsComponent} from './pages/artists/artists.component';
import {TracksComponent} from './pages/tracks/tracks.component';

const routes: Routes = [
  {path: 'artists/:artistId/albums', component: AlbumsComponent},
  {path: 'artists/:artistId/tracks', component: TracksComponent},
  {path: 'artists/:artistId/albums/:albumId/tracks', component: TracksComponent},
  {path: 'artists', component: ArtistsComponent},
  {path: 'albums', component: AlbumsComponent},
  {path: 'albums/:albumId/tracks', component: TracksComponent},
  {path: 'tracks', component: TracksComponent},
  {path: '', pathMatch: 'full', redirectTo: 'artists'}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
