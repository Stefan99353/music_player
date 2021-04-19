use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::api::albums::AlbumFilter;
use crate::models::albums::Album;
use crate::paginate::{LoadPaginated, PaginationResult};

pub fn find_album_by_id(
    album_id: i32,
    conn: &SqliteConnection,
) -> Result<Option<Album>, diesel::result::Error> {
    use crate::schema::albums::dsl::*;

    let res = albums
        .filter(id.eq(album_id))
        .first::<Album>(conn)
        .optional()?;

    Ok(res)
}

pub fn get_albums(
    filter: AlbumFilter,
    conn: &SqliteConnection,
) -> Result<PaginationResult<Album>, diesel::result::Error> {
    use crate::schema::albums::dsl::*;

    let mut query = albums.into_boxed::<Sqlite>();

    // Artist ID
    if let Some(art_id) = filter.artist_id {
        query = query.filter(artist_id.eq(art_id));
    }

    // Filter
    if let Some(filter) = filter.filter {
        let filter = format!("%{}%", filter);
        query = query.filter(title.like(filter));
    }

    query = query.order(title.asc());

    let res = query.load_with_pagination(&conn, filter.page, filter.limit)?;

    Ok(res)
}
