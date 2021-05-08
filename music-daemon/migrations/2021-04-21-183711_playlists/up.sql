-- Your SQL goes here

CREATE TABLE playlists
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL,
    icon        TEXT,
    description TEXT,

    inserted    DATETIME,
    updated     DATETIME
);

CREATE TABLE playlist_track
(
    playlist_id INTEGER NOT NULL,
    track_id    INTEGER NOT NULL,

    inserted    DATETIME,
    updated     DATETIME,

    PRIMARY KEY (playlist_id, track_id),

    FOREIGN KEY (playlist_id) REFERENCES playlists (id),
    FOREIGN KEY (track_id) REFERENCES tracks (id)
);

INSERT INTO playlists (id, name, icon, description, inserted, updated)
VALUES (0, 'Favorites', 'favorite', 'Playlist with all your favorite tracks', datetime('now'), datetime('now'));
