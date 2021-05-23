use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, SqliteConnection};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

impl Album {
    pub fn get(album_id: i32, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::albums::dsl::*;

        albums
            .filter(id.eq(album_id))
            .first::<Self>(conn)
            .optional()
    }

    pub fn find(album_title: &str, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::albums::dsl::*;

        albums
            .filter(title.eq(album_title))
            .first::<Self>(conn)
            .optional()
    }

    pub fn insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        use crate::schema::albums::dsl::*;

        let now = Utc::now().naive_utc();

        let album = NewAlbum {
            title: self.title,
            track_count: self.track_count,
            disc_count: self.disc_count,
            year: self.year,
            rating: self.rating,
            image_id: self.image_id,
            artist_id: self.artist_id,
            inserted: Some(now),
            updated: Some(now)
        };

        diesel::insert_into(albums)
            .values(album)
            .execute(conn)?;

        albums
            .filter(inserted.eq(now))
            .first::<Self>(conn)
    }

    pub fn find_or_insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        let album = Self::find(&self.title, conn)?;

        if let Some(album) = album {
            return Ok(album);
        }

        self.insert(conn)
    }
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
