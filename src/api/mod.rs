use std::sync::{Mutex, Arc};

use diesel::SqliteConnection;
use serde::Deserialize;

use crate::DbPool;
use crate::player::RodioPlayer;
use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;

pub mod albums;
pub mod artists;
pub mod management;
pub mod player;
pub mod queue;
pub mod tracks;
pub mod images;
pub mod playlists;

pub type PlayerData = Mutex<RodioPlayer>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestFilter {
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

pub fn get_db_connection(pool: Arc<DbPool>) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, actix_web::Error> {
    pool.get().map_err(|err| {
        error!("{}", err);
        actix_web::HttpResponse::InternalServerError().finish().into()
    })
}
