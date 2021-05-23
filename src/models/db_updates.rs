use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::db_updates;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct DbUpdate {
    pub id: i32,
    pub started: NaiveDateTime,
    pub finished: NaiveDateTime,
    pub tracks_before: i32,
    pub tracks_after: i32,
    pub albums_before: i32,
    pub albums_after: i32,
    pub artists_before: i32,
    pub artists_after: i32,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "db_updates"]
pub struct NewDbUpdate {
    pub started: NaiveDateTime,
    pub finished: NaiveDateTime,
    pub tracks_before: i32,
    pub tracks_after: i32,
    pub albums_before: i32,
    pub albums_after: i32,
    pub artists_before: i32,
    pub artists_after: i32,
    pub inserted: Option<NaiveDateTime>,
    pub updated: Option<NaiveDateTime>,
}
