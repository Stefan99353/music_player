use diesel::prelude::*;

use crate::crawler::ArtistPop;

pub fn insert_into_database(conn: &SqliteConnection, new_artists: Vec<ArtistPop>) {
    info!("Inserting new elements into database");

    let new_artists = new_artists;
    let mut art_ids: Vec<i32> = vec![];
    let mut alb_ids: Vec<i32> = vec![];
    let mut trk_ids: Vec<i32> = vec![];

    // Artists
    for new_artist in new_artists {
        debug!("Inserting artist '{}'", &new_artist.artist.name);
        let artist_db_id = match new_artist.artist.insert(conn) {
            Ok(art) => {
                art_ids.push(art.id);
                art.id
            }
            Err(err) => {
                error!("Error while inserting artist");
                error!("{}", err);
                continue;
            }
        };

        // Albums
        for mut new_album in new_artist.albums {
            debug!("Inserting album '{}'", &new_album.album.title);
            new_album.album.artist_id = artist_db_id;

            let album_db_id = match new_album.album.insert(conn) {
                Ok(alb) => {
                    alb_ids.push(alb.id);
                    alb.id
                }
                Err(err) => {
                    error!("Error while inserting album");
                    error!("{}", err);
                    continue;
                }
            };

            // Tracks
            for mut new_track in new_album.tracks {
                debug!("Trying inserting track '{}'", &new_track.title);
                new_track.album_id = album_db_id;

                match new_track.insert(conn) {
                    Ok(trk) => {
                        trk_ids.push(trk.id);
                    }
                    Err(err) => {
                        error!("Error while inserting track");
                        error!("{}", err);
                        continue;
                    }
                }
            }
        }
    }

    info!("Removing non-existing elements from database");
    match cleanup(conn, art_ids, alb_ids, trk_ids) {
        Ok(_) => {}
        Err(err) => {
            error!("Error during cleanup");
            error!("{}", err);
        }
    };
}

fn cleanup(conn: &SqliteConnection, art_ids: Vec<i32>, alb_ids: Vec<i32>, trk_ids: Vec<i32>) -> anyhow::Result<()> {
    use crate::schema::{albums, artists, tracks};

    // Tracks
    diesel::delete(tracks::table.filter(tracks::id.ne_all(trk_ids))).execute(conn)?;

    // Albums
    diesel::delete(albums::table.filter(albums::id.ne_all(alb_ids))).execute(conn)?;

    // Artists
    diesel::delete(artists::table.filter(artists::id.ne_all(art_ids))).execute(conn)?;

    Ok(())
}
