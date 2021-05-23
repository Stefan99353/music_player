use std::sync::{Arc, Mutex};

use diesel::prelude::*;

use crate::models::tracks::PopulatedTrack;
use crate::player::RodioPlayer;

pub fn add_to_queue(
    conn: &SqliteConnection,
    player: Arc<Mutex<RodioPlayer>>,
    ids: Vec<i32>,
) -> anyhow::Result<()> {
    use crate::schema::populated_tracks::dsl::*;

    let tracks_to_add = populated_tracks
        .filter(id.eq_any(ids))
        .load::<PopulatedTrack>(conn)?;

    let mut player = player.lock().unwrap();

    player.add_tracks(tracks_to_add);

    Ok(())
}

pub fn add_artist_to_queue(
    conn: &SqliteConnection,
    player: Arc<Mutex<RodioPlayer>>,
    artist_id: i32,
) -> anyhow::Result<()> {
    use crate::schema::{populated_tracks, artists};

    let tracks_to_add = populated_tracks::table
        .select(populated_tracks::all_columns)
        .inner_join(artists::table)
        .filter(artists::id.eq(artist_id))
        .order_by(populated_tracks::title.asc())
        .load::<PopulatedTrack>(conn)?;

    let mut player = player.lock().unwrap();

    player.add_tracks(tracks_to_add);

    Ok(())
}

pub fn add_album_to_queue(
    conn: &SqliteConnection,
    player: Arc<Mutex<RodioPlayer>>,
    album_id: i32,
) -> anyhow::Result<()> {
    use crate::schema::{populated_tracks, albums};

    let tracks_to_add = populated_tracks::table
        .select(populated_tracks::all_columns)
        .inner_join(albums::table)
        .filter(albums::id.eq(album_id))
        .order_by(populated_tracks::title.asc())
        .load::<PopulatedTrack>(conn)?;

    let mut player = player.lock().unwrap();

    player.add_tracks(tracks_to_add);

    Ok(())
}
