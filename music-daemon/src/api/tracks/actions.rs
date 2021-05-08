use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::SqliteConnection;

use crate::api::RequestFilter;
use crate::models::tracks::{PopulatedTrack, Track};
use crate::paginate::{LoadPaginated, PaginationResult};
use crate::models::albums::Album;

pub fn all_tracks(
    filter: RequestFilter,
    conn: &SqliteConnection,
) -> diesel::QueryResult<PaginationResult<PopulatedTrack>> {
    use crate::schema::populated_tracks;

    let mut query = populated_tracks::table.into_boxed::<Sqlite>();

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

pub fn add_track(
    _track: Track,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<Track> {
    todo!()
}

pub fn update_track(
    _track_id: i32,
    _track: Track,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<Track> {
    todo!()
}

pub fn delete_track(
    _track_id: i32,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<()> {
    todo!()
}

pub fn get_image_id(
    track_id: i32,
    conn: &SqliteConnection,
) -> diesel::QueryResult<Option<i32>> {
    let track = match Track::get(track_id, conn)? {
        None => { return Ok(None); }
        Some(track) => { track }
    };

    match track.image_id {
        None => {
            match Album::get(track.album_id, conn)? {
                None => {
                    Ok(None)
                }
                Some(album) => {
                    Ok(album.image_id)
                }
            }
        }
        Some(id) => {
            Ok(Some(id))
        }
    }
}

pub fn add_image(
    _artist_id: i32,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<Track> {
    todo!()
}

pub fn delete_image(
    _artist_id: i32,
    _conn: &SqliteConnection,
) -> diesel::QueryResult<()> {
    todo!()
}
