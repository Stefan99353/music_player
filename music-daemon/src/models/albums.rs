use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, SqliteConnection};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::schema::albums;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub track_count: Option<i32>,
    pub disc_count: Option<i32>,
    pub year: Option<i32>,
    pub rating: Option<f32>,
    pub image_id: Option<i32>,
    pub artist_id: i32,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "albums"]
pub struct NewAlbum {
    pub title: String,
    pub track_count: Option<i32>,
    pub disc_count: Option<i32>,
    pub year: Option<i32>,
    pub rating: Option<f32>,
    pub image_id: Option<i32>,
    pub artist_id: i32,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

impl NewAlbum {
    pub fn insert(mut self, conn: &SqliteConnection) -> anyhow::Result<Album> {
        use crate::schema::albums::dsl::*;

        let album: Option<Album> = albums
            .filter(title.eq(&self.title))
            .filter(artist_id.eq(&self.artist_id))
            .first::<Album>(conn)
            .optional()?;

        if let Some(album) = album {
            return Ok(album);
        }

        let now = Utc::now().naive_utc();
        self.inserted = Some(now);
        self.updated = Some(now);

        diesel::insert_into(albums)
            .values(self)
            .execute(conn)?;

        let album = albums
            .filter(inserted.eq(now))
            .first::<Album>(conn)?;

        Ok(album)
    }
}
