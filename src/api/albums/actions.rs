use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::api::RequestFilter;
use crate::models::albums::Album;
use crate::models::tracks::PopulatedTrack;
use crate::paginate::{LoadPaginated, PaginationResult};

pub fn all_albums(
    filter: RequestFilter,
    conn: &SqliteConnection,
) -> diesel::QueryResult<PaginationResult<Album>> {
    use crate::schema::albums::dsl::*;

    let mut query = albums.into_boxed::<Sqlite>();

    // Filter
    if let Some(filter) = filter.filter {
        query = query.filter(title.like(format!("%{}%", filter)))
    }

    // Order by
    let sort_column = filter.sort.unwrap_or_else(|| { String::from("title") }).to_ascii_lowercase();
    let direction = filter.order.unwrap_or_else(|| { String::from("asc") }).to_ascii_lowercase();
    match direction.as_str() {
        "desc" => {
            match sort_column.as_str() {
                "title" => {
                    query = query.order((title.desc(), id.asc()))
                }
                "year" => {
                    query = query.order((year.desc(), id.asc()))
                }
                "rating" => {
                    query = query.order((year.desc(), id.asc()))
                }
                _ => {
                    query = query.order(id.asc())
                }
            }
        }
        _ => {
            match sort_column.as_str() {
                "title" => {
                    query = query.order((title.asc(), id.asc()))
                }
                "year" => {
                    query = query.order((year.asc(), id.asc()))
                }
                "rating" => {
                    query = query.order((year.asc(), id.asc()))
                }
                _ => {
                    query = query.order(id.asc())
                }
            }
        }
    }

    query.load_with_pagination(&conn, filter.page, filter.limit)
}

pub fn add_album(
    _album: Album,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<Album> {
    todo!()
}

pub fn update_album(
    _album_id: i32,
    _album: Album,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<Album> {
    todo!()
}

pub fn delete_album(
    _album_id: i32,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<()> {
    todo!()
}

pub fn all_tracks(
    album_id: i32,
    filter: RequestFilter,
    conn: &SqliteConnection,
) -> diesel::QueryResult<PaginationResult<PopulatedTrack>> {
    use crate::schema::{albums, populated_tracks};

    let mut query = populated_tracks::table.into_boxed::<Sqlite>()
        .select(populated_tracks::all_columns)
        .inner_join(albums::table)
        .filter(albums::id.eq(album_id));

    // Filter
    if let Some(filter) = filter.filter {
        query = query.filter(populated_tracks::title.like(format!("%{}%", filter)))
    }

    // Order by
    let sort_column = filter.sort.unwrap_or_else(|| { String::from("title") }).to_ascii_lowercase();
    let direction = filter.order.unwrap_or_else(|| { String::from("asc") }).to_ascii_lowercase();
    match direction.as_str() {
        "desc" => {
            match sort_column.as_str() {
                "title" => {
                    query = query.order((populated_tracks::title.desc(), populated_tracks::id.asc()))
                }
                "date" => {
                    query = query.order((populated_tracks::date.desc(), populated_tracks::id.asc()))
                }
                "genre" => {
                    query = query.order((populated_tracks::genre.desc(), populated_tracks::id.asc()))
                }
                "rating" => {
                    query = query.order((populated_tracks::rating.desc(), populated_tracks::id.asc()))
                }
                "duration" => {
                    query = query.order((populated_tracks::duration.desc(), populated_tracks::id.asc()))
                }
                _ => {
                    query = query.order(populated_tracks::id.asc())
                }
            }
        }
        _ => {
            match sort_column.as_str() {
                "title" => {
                    query = query.order((populated_tracks::title.asc(), populated_tracks::id.asc()))
                }
                "date" => {
                    query = query.order((populated_tracks::date.asc(), populated_tracks::id.asc()))
                }
                "genre" => {
                    query = query.order((populated_tracks::genre.asc(), populated_tracks::id.asc()))
                }
                "rating" => {
                    query = query.order((populated_tracks::rating.asc(), populated_tracks::id.asc()))
                }
                "duration" => {
                    query = query.order((populated_tracks::duration.asc(), populated_tracks::id.asc()))
                }
                _ => {
                    query = query.order(populated_tracks::id.asc())
                }
            }
        }
    }

    query.load_with_pagination(&conn, filter.page, filter.limit)
}

pub fn add_image(
    _album_id: i32,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<Album> {
    todo!()
}

pub fn delete_image(
    _album_id: i32,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<()> {
    todo!()
}
