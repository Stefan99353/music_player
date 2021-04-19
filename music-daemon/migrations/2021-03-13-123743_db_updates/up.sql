-- Your SQL goes here

CREATE TABLE db_updates
(
    id             INTEGER PRIMARY KEY NOT NULL,
    started        DATETIME            NOT NULL,
    finished       DATETIME            NOT NULL,
    tracks_before  INTEGER             NOT NULL,
    tracks_after   INTEGER             NOT NULL,
    albums_before  INTEGER             NOT NULL,
    albums_after   INTEGER             NOT NULL,
    artists_before INTEGER             NOT NULL,
    artists_after  INTEGER             NOT NULL,

    inserted       DATETIME,
    updated        DATETIME
);
