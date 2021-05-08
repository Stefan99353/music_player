use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, SqliteConnection};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

#[allow(dead_code)]
impl Artist {
    pub fn get(artist_id: i32, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::artists::dsl::*;

        artists
            .filter(id.eq(artist_id))
            .first::<Self>(conn)
            .optional()
    }

    pub fn find(artist_name: &String, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::artists::dsl::*;

        artists
            .filter(name.eq(artist_name))
            .first::<Self>(conn)
            .optional()
    }

    pub fn insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        use crate::schema::artists::dsl::*;

        let now = Utc::now().naive_utc();

        let artist = NewArtist {
            name: self.name,
            image_id: self.image_id,
            inserted: Some(now),
            updated: Some(now)
        };

        diesel::insert_into(artists)
            .values(artist)
            .execute(conn)?;

        artists
            .filter(inserted.eq(now))
            .first::<Self>(conn)
    }

    pub fn find_or_insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        let artist = Self::find(&self.name, conn)?;

        if let Some(artist) = artist {
            return Ok(artist);
        }

        self.insert(conn)
    }
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
