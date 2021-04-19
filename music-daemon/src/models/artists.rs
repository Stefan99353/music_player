use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, SqliteConnection};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::schema::artists;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub image_id: Option<i32>,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "artists"]
pub struct NewArtist {
    pub name: String,
    pub image_id: Option<i32>,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

impl NewArtist {
    pub fn insert(mut self, conn: &SqliteConnection) -> anyhow::Result<Artist> {
        use crate::schema::artists::dsl::*;

        let artist: Option<Artist> = artists
            .filter(name.eq(&self.name))
            .first::<Artist>(conn)
            .optional()?;

        if let Some(artist) = artist {
            return Ok(artist);
        }

        let now = Utc::now().naive_utc();
        self.inserted = Some(now);
        self.updated = Some(now);

        diesel::insert_into(artists)
            .values(self)
            .execute(conn)?;

        let artist = artists
            .filter(inserted.eq(now))
            .first::<Artist>(conn)?;

        Ok(artist)
    }
}
