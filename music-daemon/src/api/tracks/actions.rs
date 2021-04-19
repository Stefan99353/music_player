use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::api::tracks::TrackFilter;
use crate::models::tracks::PopulatedTrack;
use crate::paginate::{LoadPaginated, PaginationResult};

pub fn find_track_by_id(
    track_id: i32,
    conn: &SqliteConnection,
) -> Result<Option<PopulatedTrack>, diesel::result::Error> {
    use crate::schema::populated_tracks::dsl::*;

    let res = populated_tracks
        .filter(id.eq(track_id))
        .first::<PopulatedTrack>(conn)
        .optional()?;

    Ok(res)
}

pub fn get_tracks(
    filter: TrackFilter,
    conn: &SqliteConnection,
) -> Result<PaginationResult<PopulatedTrack>, diesel::result::Error> {
    use crate::schema::populated_tracks::dsl::*;

    let mut query = populated_tracks.into_boxed::<Sqlite>();

    // Artist ID
    if let Some(art_id) = filter.artist_id {
        query = query.filter(artist_id.eq(art_id));
    }

    // Album ID
    if let Some(alb_id) = filter.album_id {
        query = query.filter(album_id.eq(alb_id));
    }

    // Filter
    if let Some(filter) = filter.filter {
        let filter = format!("%{}%", filter);
        query = query.filter(title.like(filter));
    }

    let sort_column = filter.sort.unwrap_or_else(|| { String::from("title") });

    // Order By
    if filter.order == Some(String::from("desc")) {
        match sort_column.as_str() {
            "duration" => {
                query = query.order((duration.desc(), title.asc()));
            }
            "album_title" => {
                query = query.order((album_title.desc(), title.asc()));
            }
            "artist_name" => {
                query = query.order((artist_name.desc(), title.asc()));
            }
            _ => {
                // Title
                query = query.order(title.desc());
            }
        }
    } else {
        match sort_column.as_str() {
            "duration" => {
                query = query.order((duration.asc(), title.asc()));
            }
            "album_title" => {
                query = query.order((album_title.asc(), title.asc()));
            }
            "artist_name" => {
                query = query.order((artist_name.asc(), title.asc()));
            }
            _ => {
                // Title
                query = query.order(title.asc());
            }
        }
    }

    let res = query.load_with_pagination(&conn, filter.page, filter.limit)?;

    Ok(res)
}
