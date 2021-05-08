import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {AlbumsComponent} from './pages/albums/albums.component';
import {ArtistsComponent} from './pages/artists/artists.component';
import {TracksComponent} from './pages/tracks/tracks.component';
import {PlaylistsComponent} from './pages/playlists/playlists.component';

const routes: Routes = [
  {path: 'artists/:artistId/albums', component: AlbumsComponent},
  {path: 'artists/:artistId/tracks', component: TracksComponent},
  {path: 'albums/:albumId/tracks', component: TracksComponent},
  {path: 'playlists/:playlistId/tracks', component: TracksComponent},
  {path: 'artists', component: ArtistsComponent},
  {path: 'albums', component: AlbumsComponent},
  {path: 'tracks', component: TracksComponent},
  {path: 'playlists', component: PlaylistsComponent},
  {path: '', pathMatch: 'full', redirectTo: 'artists'}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
