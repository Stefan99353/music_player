use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

use crate::schema::{playlist_track, playlists};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

#[allow(dead_code)]
impl Playlist {
    pub fn get(playlist_id: i32, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::playlists::dsl::*;

        playlists
            .filter(id.eq(playlist_id))
            .first::<Self>(conn)
            .optional()
    }

    pub fn find(playlist_name: &String, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::playlists::dsl::*;

        playlists
            .filter(name.eq(playlist_name))
            .first::<Self>(conn)
            .optional()
    }

    pub fn add_track(playlist_id: i32, track_id: i32, conn: &SqliteConnection) -> diesel::QueryResult<usize> {
        let now = Utc::now().naive_utc();

        let new_playlist_track = PlaylistTrack {
            playlist_id,
            track_id,
            inserted: Some(now),
            updated: Some(now),
        };

        diesel::insert_into(playlist_track::table)
            .values(new_playlist_track)
            .execute(conn)
    }

    pub fn insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        use crate::schema::playlists::dsl::*;

        let now = Utc::now().naive_utc();

        let playlist = NewPlaylist {
            name: self.name,
            icon: self.icon,
            description: self.description,
            inserted: Some(now),
            updated: Some(now)
        };

        diesel::insert_into(playlists)
            .values(playlist)
            .execute(conn)?;

        playlists
            .filter(inserted.eq(now))
            .first::<Self>(conn)
    }

    pub fn find_or_insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        let playlist = Self::find(&self.name, conn)?;

        if let Some(playlist) = playlist {
            return Ok(playlist);
        }

        self.insert(conn)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "playlists"]
pub struct NewPlaylist {
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "playlist_track"]
pub struct PlaylistTrack {
    pub playlist_id: i32,
    pub track_id: i32,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

impl PlaylistTrack {
    pub fn insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        use crate::schema::playlist_track::dsl::*;
        let mut new_track = self;

        let now = Utc::now().naive_utc();
        new_track.inserted = Some(now);
        new_track.updated = Some(now);

        diesel::insert_into(playlist_track)
            .values(&new_track)
            .execute(conn)?;

        Ok(new_track)
    }
}
