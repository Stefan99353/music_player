use std::fs::File;
use std::io::BufReader;

use chrono::{NaiveDateTime, Utc};
use rodio::Decoder;
use serde::{Deserialize, Serialize};

use crate::schema::tracks;
use diesel::SqliteConnection;
use diesel::prelude::*;

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

impl NewTrack {
    pub fn insert(mut self, conn: &SqliteConnection) -> anyhow::Result<Track> {
        use crate::schema::tracks::dsl::*;

        let track: Option<Track> = tracks
            .filter(title.eq(&self.title))
            .filter(album_id.eq(&self.album_id))
            .first::<Track>(conn)
            .optional()?;

        if let Some(track) = track {
            return Ok(track);
        }

        let now = Utc::now().naive_utc();
        self.inserted = Some(now);
        self.updated = Some(now);

        diesel::insert_into(tracks)
            .values(self)
            .execute(conn)?;

        let track = tracks
            .filter(inserted.eq(now))
            .first::<Track>(conn)?;

        Ok(track)
    }
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

impl PopulatedTrack {
    pub fn source(&self) -> anyhow::Result<Decoder<BufReader<File>>> {
        let file = File::open(&self.path)?;
        let source = rodio::Decoder::new(BufReader::new(file))?;

        Ok(source)
    }
}
