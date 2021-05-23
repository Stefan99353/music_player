use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, SqliteConnection};
use serde::{Deserialize, Serialize};

use crate::schema::images;
use diesel::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: i32,
    pub path: String,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

impl Image {
    pub fn get(image_id: i32, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::images::dsl::*;

        images
            .filter(id.eq(image_id))
            .first::<Self>(conn)
            .optional()
    }

    pub fn find(image_path: &str, conn: &SqliteConnection) -> diesel::QueryResult<Option<Self>> {
        use crate::schema::images::dsl::*;

        images
            .filter(path.eq(image_path))
            .first::<Self>(conn)
            .optional()
    }

    pub fn insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        use crate::schema::images::dsl::*;

        let now = Utc::now().naive_utc();

        let image = NewImage {
            path: self.path,
            inserted: Some(now),
            updated: Some(now)
        };

        diesel::insert_into(images)
            .values(image)
            .execute(conn)?;

        images
            .filter(inserted.eq(now))
            .first::<Self>(conn)
    }

    pub fn find_or_insert(self, conn: &SqliteConnection) -> diesel::QueryResult<Self> {
        let image = Self::find(&self.path, conn)?;

        if let Some(image) = image {
            return Ok(image);
        }

        self.insert(conn)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "images"]
pub struct NewImage {
    pub path: String,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}
