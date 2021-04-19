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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "images"]
pub struct NewImage {
    pub path: String,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

impl NewImage {
    pub fn insert(mut self, conn: &SqliteConnection) -> anyhow::Result<Image> {
        use crate::schema::images::dsl::*;

        let image: Option<Image> = images
            .filter(path.eq(&self.path))
            .first::<Image>(conn)
            .optional()?;

        if let Some(image) = image {
            return Ok(image);
        }

        let now = Utc::now().naive_utc();
        self.inserted = Some(now);
        self.updated = Some(now);

        diesel::insert_into(images)
            .values(self)
            .execute(conn)?;

        let image = images
            .filter(inserted.eq(now))
            .first::<Image>(conn)?;

        Ok(image)
    }
}
