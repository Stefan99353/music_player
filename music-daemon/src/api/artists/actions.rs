use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::api::artists::ArtistFilter;
use crate::models::artists::Artist;
use crate::paginate::{LoadPaginated, PaginationResult};

pub fn find_artist_by_id(
    artist_id: i32,
    conn: &SqliteConnection,
) -> Result<Option<Artist>, diesel::result::Error> {
    use crate::schema::artists::dsl::*;

    let res = artists
        .filter(id.eq(artist_id))
        .first::<Artist>(conn)
        .optional()?;

    Ok(res)
}

pub fn get_artists(
    filter: ArtistFilter,
    conn: &SqliteConnection,
) -> Result<PaginationResult<Artist>, diesel::result::Error> {
    use crate::schema::artists::dsl::*;

    let mut query = artists.into_boxed::<Sqlite>();

    // Filter
    if let Some(filter) = filter.filter {
        let filter = format!("%{}%", filter);
        query = query.filter(name.like(filter));
    }

    query = query.order(name.asc());

    let res = query.load_with_pagination(&conn, filter.page, filter.limit)?;

    Ok(res)
}
