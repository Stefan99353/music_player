-- Your SQL goes here

-- Artists
CREATE TABLE artists
(
    id       INTEGER PRIMARY KEY NOT NULL,
    name     TEXT                NOT NULL,
    image_id INTEGER,

    inserted DATETIME,
    updated  DATETIME,

    FOREIGN KEY (image_id) REFERENCES images (id)
);

-- Albums
CREATE TABLE albums
(
    id          INTEGER PRIMARY KEY NOT NULL,
    title       TEXT                NOT NULL,
    track_count INTEGER,
    disc_count  INTEGER,
    year        INTEGER,
    rating      REAL,
    image_id    INTEGER,
    artist_id   INTEGER             NOT NULL,

    inserted    DATETIME,
    updated     DATETIME,

    FOREIGN KEY (artist_id) REFERENCES artists (id),
    FOREIGN KEY (image_id) REFERENCES images (id)
);

-- Tracks
CREATE TABLE tracks
(
    id           INTEGER PRIMARY KEY NOT NULL,
    path         TEXT                NOT NULL,
    title        TEXT                NOT NULL,
    date         DATETIME,
    genre        TEXT,
    rating       REAL,
    track_number INTEGER,
    disc_number  INTEGER,
    duration     INTEGER             NOT NULL,
    image_id     INTEGER,
    album_id     INTEGER             NOT NULL,
    size         INTEGER             NOT NULL,

    inserted     DATETIME,
    updated      DATETIME,

    FOREIGN KEY (album_id) REFERENCES albums (id),
    FOREIGN KEY (image_id) REFERENCES images (id)
);

-- Images
CREATE TABLE images
(
    id       INTEGER PRIMARY KEY NOT NULL,
    path     TEXT                NOT NULL,

    inserted DATETIME,
    updated  DATETIME
);
