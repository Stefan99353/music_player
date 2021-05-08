# TODO

- [ ] Playlists
  - [ ] Add songs to playlist
  - [ ] Remove song from playlist 
- [ ] Upload von Tracks
- [ ] YouTube Download/Stream
- [ ] Stream Tracks to Browser

# API
 
## Artists

| Path                          | Method   | Description              | Filter |
| ----------------------------- |:--------:| ------------------------ |:------:|
| `/artists`                    | `GET`    | All Artists              | x      |
| `/artists`                    | `POST`   | Add Artist               |        |
| `/artists/{artist_id}`        | `GET`    | Single Artist            |        |
| `/artists/{artist_id}`        | `PUT`    | Update Artist            |        |
| `/artists/{artist_id}`        | `DELETE` | Delete Artist            |        |
| `/artists/{artist_id}/albums` | `GET`    | Get all Albums of Artist | x      |
| `/artists/{artist_id}/tracks` | `GET`    | Get all Tracks of Artist | x      |
| `/artists/{artist_id}/image`  | `POST`   | Upload Image for Artist  |        |
| `/artists/{artist_id}/image`  | `DELETE` | Delete Image from Artist |        |

## Albums

| Path                        | Method   | Description             | Filter |
| --------------------------- |:--------:| ----------------------- |:------:|
| `/albums`                   | `GET`    | All Albums              | x      |
| `/albums`                   | `POST`   | Add Album               |        |
| `/albums/{album_id}`        | `GET`    | Single Album            |        |
| `/albums/{album_id}`        | `PUT`    | Update Album            |        |
| `/albums/{album_id}`        | `DELETE` | Delete Album            |        |
| `/albums/{album_id}/tracks` | `GET`    | Get all Tracks of Album | x      |
| `/albums/{album_id}/image`  | `POST`   | Upload Image for Album  |        |
| `/albums/{album_id}/image`  | `DELETE` | Delete Image from Album |        |

## Tracks

| Path                        | Method   | Description             | Filter |
| --------------------------- |:--------:| ----------------------- |:------:|
| `/tracks`                   | `GET`    | All Tracks              | x      |
| `/tracks`                   | `POST`   | Add Track               |        |
| `/tracks/{track_id}`        | `GET`    | Single Track            |        |
| `/tracks/{track_id}`        | `PUT`    | Update Track            |        |
| `/tracks/{track_id}`        | `DELETE` | Delete Track            |        |
| `/tracks/{track_id}/stream` | `GET`    | Stream Track            |        |
| `/tracks/{track_id}/image`  | `GET`    | Get image ID of Track   |        |
| `/tracks/{track_id}/image`  | `POST`   | Upload Image for Track  |        |
| `/tracks/{track_id}/image`  | `DELETE` | Delete Image from Track |        |

## Playlists

| Path                              | Method   | Description                | Filter |
| --------------------------------- |:--------:| -------------------------- |:------:|
| `/playlists`                      | `GET`    | All Playlists              | x      |
| `/playlists`                      | `POST`   | Add Playlist               |        |
| `/playlists/{playlist_id}`        | `GET`    | Single Playlist            |        |
| `/playlists/{playlist_id}`        | `PUT`    | Update Playlist            |        |
| `/playlists/{playlist_id}`        | `DELETE` | Delete Playlist            |        |
| `/playlists/{playlist_id}/tracks` | `GET`    | Get all Tracks of Playlist | x      |
| `/playlists/{playlist_id}/tracks` | `POST`   | Add Track to Playlist      |        |
| `/playlists/{playlist_id}/tracks` | `DELETE` | Remove Track from Playlist |        |

## Player

| Path             | Method   | Description      | Filter |
| ---------------- |:--------:| ---------------- |:------:|
| `/player`        | `GET`    | Get Player State |        |
| `/player/resume` | `POST`   | Resume Playback  |        |
| `/player/pause`  | `POST`   | Pause Playback   |        |
| `/player/stop`   | `POST`   | Stop Playback    |        |
| `/player/next`   | `POST`   | Next Track       |        |
| `/player/prev`   | `POST`   | Previous Track   |        |
| `/player/seek`   | `POST`   | Seek in Track    |        |
| `/player/volume` | `POST`   | Set Volume       |        |

## Queue

| Path                | Method   | Description         | Filter |
| ------------------- |:--------:| ------------------- |:------:|
| `/queue`            | `GET`    | Get Queue           |        |
| `/queue`            | `DELETE` | Clear Queue         |        |
| `/queue/length`     | `GET`    | Get Queue Length    |        |
| `/queue/add`        | `POST`   | Add Tracks to Queue |        |
| `/queue/add/artist` | `GET`    | Add Artist to Queue |        |
| `/queue/add/album`  | `GET`    | Add Album to Queue  |        |

## Management

| Path       | Method | Description    | Filter |
| ---------- |:------:| -------------- |:------:|
| `/updates` | `GET`  | Get DB Updates | Filter |
| `/updates` | `POST` | Update DB      | Filter |
| `/updates` | `PUT`  | Rebuild DB     | Filter |

## Images

| Path                 | Method | Description  | Filter |
| -------------------- |:------:| ------------ |:------:|
| `/images/{image_id}` | `GET`  | Single Image |        |
