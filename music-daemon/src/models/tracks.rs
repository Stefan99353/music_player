use std::fs::File;
use std::io::BufReader;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::SqliteConnection;
use rodio::Decoder;
use serde::{Deserialize, Serialize};

use crate::schema::tracks;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: i32,
    pub path: String,
    pub title: String,
    pub date: Option<NaiveDateTime>,
    pub genre: Option<String>,
    pub rating: Option<f32>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub duration: i32,
    pub image_id: Option<i32>,
    pub album_id: i32,
    pub size: i32,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

#[allow(dead_code)]
impl Track {
    pub fn get(track_id: i32, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::tracks::dsl::*;

        tracks
            .filter(id.eq(track_id))
            .first::<Self>(conn)
            .optional()
    }

    pub fn find(track_title: &String, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::tracks::dsl::*;

        tracks
            .filter(title.eq(track_title))
            .first::<Self>(conn)
            .optional()
    }

    pub fn insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        use crate::schema::tracks::dsl::*;

        let now = Utc::now().naive_utc();

        let track = NewTrack {
            path: self.path,
            title: self.title,
            date: self.date,
            genre: self.genre,
            rating: self.rating,
            track_number: self.track_number,
            disc_number: self.disc_number,
            duration: self.duration,
            image_id: self.image_id,
            album_id: self.album_id,
            size: self.size,
            inserted: Some(now),
            updated: Some(now)
        };

        diesel::insert_into(tracks)
            .values(track)
            .execute(conn)?;

        tracks
            .filter(inserted.eq(now))
            .first::<Self>(conn)
    }

    pub fn find_or_insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        let track = Self::find(&self.title, conn)?;

        if let Some(track) = track {
            return Ok(track);
        }

        self.insert(conn)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "tracks"]
pub struct NewTrack {
    pub path: String,
    pub title: String,
    pub date: Option<NaiveDateTime>,
    pub genre: Option<String>,
    pub rating: Option<f32>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub duration: i32,
    pub image_id: Option<i32>,
    pub album_id: i32,
    pub size: i32,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct PopulatedTrack {
    pub id: i32,
    pub path: String,
    pub title: String,
    pub date: Option<NaiveDateTime>,
    pub genre: Option<String>,
    pub rating: Option<f32>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub duration: i32,
    pub image_id: Option<i32>,
    pub album_id: i32,
    pub artist_id: i32,
    pub album_title: String,
    pub artist_name: String,
    pub size: i32,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

#[allow(dead_code)]
impl PopulatedTrack {
    pub fn get(track_id: i32, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::populated_tracks::dsl::*;

        populated_tracks
            .filter(id.eq(track_id))
            .first::<Self>(conn)
            .optional()
    }

    pub fn find(track_title: &String, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::populated_tracks::dsl::*;

        populated_tracks
            .filter(title.eq(track_title))
            .first::<Self>(conn)
            .optional()
    }

    pub fn source(&self) -> anyhow::Result<Decoder<BufReader<File>>> {
        let file = File::open(&self.path)?;
        let source = rodio::Decoder::new(BufReader::new(file))?;

        Ok(source)
    }
}
