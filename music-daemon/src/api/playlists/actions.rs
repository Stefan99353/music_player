use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::api::RequestFilter;
use crate::models::playlists::Playlist;
use crate::models::tracks::PopulatedTrack;
use crate::paginate::{LoadPaginated, PaginationResult};
use chrono::Utc;

pub fn all_playlists(
    filter: RequestFilter,
    conn: &SqliteConnection,
) -> diesel::QueryResult<PaginationResult<Playlist>> {
    use crate::schema::playlists::dsl::*;

    let mut query = playlists.into_boxed::<Sqlite>();

    // Filter
    if let Some(filter) = filter.filter {
        query = query.filter(name.like(format!("%{}%", filter)))
    }

    // Order by
    let sort_column = filter.sort.unwrap_or_else(|| { String::from("name") }).to_ascii_lowercase();
    let direction = filter.order.unwrap_or_else(|| { String::from("asc") }).to_ascii_lowercase();
    match direction.as_str() {
        "desc" => {
            match sort_column.as_str() {
                "name" => {
                    query = query.order((name.desc(), id.asc()))
                }
                "description" => {
                    query = query.order((description.desc(), id.asc()))
                }
                _ => {
                    query = query.order(id.asc())
                }
            }
        }
        _ => {
            match sort_column.as_str() {
                "name" => {
                    query = query.order((name.asc(), id.asc()))
                }
                "description" => {
                    query = query.order((description.asc(), id.asc()))
                }
                _ => {
                    query = query.order(id.asc())
                }
            }
        }
    }

    query.load_with_pagination(&conn, filter.page, filter.limit)
}

pub fn update_playlist(
    playlist_id: i32,
    new_playlist: Playlist,
    conn: &SqliteConnection,
) -> diesel::QueryResult<Playlist> {
    use crate::schema::playlists::dsl::*;

    let now = Some(Utc::now().naive_utc());

    diesel::update(playlists.filter(id.eq(playlist_id)))
        .set((
            name.eq(new_playlist.name),
            icon.eq(new_playlist.icon),
            description.eq(new_playlist.description),
            updated.eq(now)
        ))
        .execute(conn)?;

    playlists.filter(id.eq(playlist_id))
        .first(conn)
}

pub fn delete_playlist(
    playlist_id: i32,
    conn: &SqliteConnection,
) -> diesel::QueryResult<usize> {
    use crate::schema::playlists::dsl::*;

    diesel::delete(playlists.filter(id.eq(playlist_id)))
        .execute(conn)
}

pub fn all_tracks(
    playlist_id: i32,
    filter: RequestFilter,
    conn: &SqliteConnection,
) -> diesel::QueryResult<PaginationResult<PopulatedTrack>> {
    use crate::schema::{playlists, playlist_track, populated_tracks};

    let mut query = playlist_track::table.into_boxed::<Sqlite>()
        .inner_join(populated_tracks::table)
        .inner_join(playlists::table)
        .select(populated_tracks::all_columns)
        .filter(playlists::id.eq(playlist_id));

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

pub fn delete_track(
    pl_id: i32,
    tr_id: i32,
    conn: &SqliteConnection,
) -> diesel::QueryResult<usize> {
    use crate::schema::playlist_track::dsl::*;

    diesel::delete(playlist_track
        .filter(playlist_id.eq(pl_id))
        .filter(track_id.eq(tr_id))
    )
        .execute(conn)
}
