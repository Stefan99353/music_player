use std::{fs, thread};
use std::path::PathBuf;
use std::sync::Arc;

use chrono::Utc;
use diesel::expression::dsl::count;
use diesel::prelude::*;
use image::ImageFormat;
use regex::Regex;

use crate::DbPool;
use crate::models::albums::Album;
use crate::models::artists::Artist;
use crate::models::db_updates::NewDbUpdate;
use crate::models::images::Image;
use crate::models::tracks::Track;
use crate::ws::notifications::hub::WsNotificationHub;
use actix::Addr;
use crate::ws::notifications::messages::{Notification, NotificationType};
use crate::api::get_db_connection;

mod database;

const SUPPORTED_EXTENSIONS: &[&str] = &["mp3", "flac", "ogg", "wav"];

pub struct ArtistPop {
    pub artist: Artist,
    pub albums: Vec<AlbumPop>,
}

pub struct AlbumPop {
    pub album: Album,
    pub tracks: Vec<Track>,
}

pub struct Crawler {
    paths: Vec<String>,
    db_pool: Arc<DbPool>,
    notifications: Arc<Addr<WsNotificationHub>>,
}

impl Crawler {
    pub fn new(
        paths: Vec<String>,
        db_pool: Arc<DbPool>,
        notifications: Arc<Addr<WsNotificationHub>>,
    ) -> Self {
        Crawler {
            paths,
            db_pool,
            notifications
        }
    }

    pub fn start(self) {
        thread::spawn(move || {
            let conn = match get_db_connection(self.db_pool){
                Ok(conn) => conn,
                Err(err) => {
                    error!("Couldn't get db connection from pool");
                    error!("{}", err);
                    return;
                }
            };
            let mut artists: Vec<ArtistPop> = vec![];

            // Send notification that scan started
            self.notifications.do_send(Notification {
                message: String::from("Started updating database"),
                message_type: NotificationType::Info,
                timestamp: Utc::now().naive_utc()
            });

            let started = Utc::now().naive_utc();
            let (tracks_before, albums_before, artists_before) = get_counts(&conn);

            // Check every folder configured in settings
            info!("Scanning configured paths");
            for path in self.paths {
                match crawl_root(&path, &mut artists, &conn) {
                    Ok(_) => {
                        // Finished scanning root folder
                        debug!("Finished crawling root folder: {}", &path);
                    }
                    Err(err) => {
                        error!("Error while crawling root folder: {}", &path);
                        error!("{}", err.to_string());
                    }
                }
            }

            // Insert into Database
            database::insert_into_database(&conn, artists);

            let (tracks_after, albums_after, artists_after) = get_counts(&conn);
            let finished = Utc::now().naive_utc();

            let db_update = NewDbUpdate {
                started,
                finished,
                tracks_before: tracks_before as i32,
                tracks_after: tracks_after as i32,
                albums_before: albums_before as i32,
                albums_after: albums_after as i32,
                artists_before: artists_before as i32,
                artists_after: artists_after as i32,
                inserted: Some(finished),
                updated: Some(finished),
            };

            use crate::schema::db_updates;

            diesel::insert_into(db_updates::table)
                .values(db_update)
                .execute(&conn)
                .unwrap();

            // Send notification that scan finished
            self.notifications.do_send(Notification {
                message: String::from("Finished updating database"),
                message_type: NotificationType::Success,
                timestamp: Utc::now().naive_utc()
            });
        });
    }
}

// Search root folder for artist folders
fn crawl_root(path: &str, artists: &mut Vec<ArtistPop>, conn: &SqliteConnection) -> anyhow::Result<()> {
    debug!("Search in root folder for artists: '{}'", path);
    let root_contents = fs::read_dir(path)?;

    // Run through every element in root folder
    for dir_entry in root_contents {
        let path = dir_entry?.path();

        // Check if entry is folder
        if path.is_dir() {
            // Found artist folder
            crawl_artist(path, artists, conn)?;
        }
    }

    Ok(())
}

// Search artist folder for albums
fn crawl_artist(path: PathBuf, artists: &mut Vec<ArtistPop>, conn: &SqliteConnection) -> anyhow::Result<()> {
    debug!("Search artist folder for albums: '{}'", path.to_string_lossy());
    let mut new_artist = ArtistPop {
        artist: Artist {
            id: 0,
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            image_id: None,
            inserted: None,
            updated: None,
        },
        albums: vec![],
    };

    let artist_contents = fs::read_dir(&path)?;

    // Run through every element in artist folder
    for dir_entry in artist_contents {
        let path = dir_entry?.path();

        // Check if entry is folder or file
        if path.is_dir() {
            // Found album folder
            crawl_album(path, &mut new_artist, conn)?;
        } else {
            // Check for image
            if ImageFormat::from_path(&path).is_ok() {
                debug!("Found image for artist: '{}'", path.to_string_lossy());
                // Insert Image
                let new_image = Image {
                    id: 0,
                    path: path.to_string_lossy().to_string(),
                    inserted: None,
                    updated: None,
                };
                let image_id = match new_image.find_or_insert(conn) {
                    Ok(img) => { Some(img.id) }
                    Err(err) => {
                        error!("Error while inserting image for artist: '{}'", &new_artist.artist.name);
                        error!("{}", err);
                        None
                    }
                };
                new_artist.artist.image_id = image_id;
            }
        }
    }

    // Temporarily store artist for DB insertion
    artists.push(new_artist);

    Ok(())
}

// Search album folder for tracks
fn crawl_album(path: PathBuf, artist: &mut ArtistPop, conn: &SqliteConnection) -> anyhow::Result<()> {
    debug!("Search album folder for tracks: '{}'", path.to_string_lossy());
    let mut new_album = AlbumPop {
        album: Album {
            id: 0,
            title: path.file_name().unwrap().to_string_lossy().to_string(),
            track_count: None,
            disc_count: None,
            year: None,
            rating: None,
            image_id: None,
            artist_id: 0,
            inserted: None,
            updated: None,
        },
        tracks: vec![],
    };

    let album_contents = fs::read_dir(&path)?;

    // Run through every element in album folder
    for dir_entry in album_contents {
        let path = dir_entry?.path();

        // Check if entry is file
        if path.is_file() {
            // Check for image
            if ImageFormat::from_path(&path).is_ok() {
                debug!("Found image for album: '{}'", path.to_string_lossy());
                // Insert Image
                let new_image = Image {
                    id: 0,
                    path: path.to_string_lossy().to_string(),
                    inserted: None,
                    updated: None,
                };
                let image_id = match new_image.find_or_insert(conn) {
                    Ok(img) => { Some(img.id) }
                    Err(err) => {
                        error!("Error while inserting image for album: '{}'", &new_album.album.title);
                        error!("{}", err);
                        None
                    }
                };
                new_album.album.image_id = image_id;
            }

            process_file(path, &mut new_album)?;
        }
    }

    // Temporarily store album for DB insertion
    artist.albums.push(new_album);

    Ok(())
}

// Processes file and checks if it is a track
fn process_file(path: PathBuf, album: &mut AlbumPop) -> anyhow::Result<()> {
    if let Some(extension) = path.extension() {
        let extension = extension.to_string_lossy().to_string();
        if !SUPPORTED_EXTENSIONS.contains(&extension.as_str()) {
            // File does not have supported extension
            return Ok(());
        }
    } else {
        // File does not have an extension
        return Ok(());
    }

    debug!("Processing track: '{}'", path.to_string_lossy());

    let title = process_track_name(&path)?;

    let mut new_track = Track {
        id: 0,
        path: path.to_string_lossy().to_string(),
        title,
        date: None,
        genre: None,
        rating: None,
        track_number: None,
        disc_number: None,
        duration: 0,
        image_id: None,
        album_id: 0,
        size: path.metadata()?.len() as i32,
        inserted: None,
        updated: None,
    };

    // Add file Metadata
    track_metadata(&mut new_track)?;

    // Temporarily store track for DB insertion
    album.tracks.push(new_track);
    Ok(())
}

fn process_track_name(
    path: &PathBuf
) -> anyhow::Result<String> {
    let mut track_name = path.file_name().unwrap().to_string_lossy().to_string();

    // Remove extension
    if let Some(ext) = path.extension() {
        let ext_len = ext.to_string_lossy().to_string().chars().count() + 1;
        let name_len = track_name.chars().count();
        track_name = track_name.chars().into_iter().take(name_len - ext_len).collect();
    }

    // Remove number
    let regex = Regex::new(r"(?P<num>[\d]{2}[\. ][ ]*)(?P<name>.*)")?;
    track_name = regex.replace_all(&track_name, "$name").to_string();

    Ok(track_name)
}

fn track_metadata(track: &mut Track) -> anyhow::Result<()> {
    debug!("Getting metadata for track '{}'", &track.title);

    let ffprobe = ffprobe::ffprobe(&track.path)?;

    let duration: i32 = (ffprobe.format.duration.parse::<f64>()? * 1000.0) as i32;

    track.duration = duration;

    Ok(())
}

fn get_counts(conn: &SqliteConnection) -> (i64, i64, i64) {
    use crate::schema::{albums, artists, tracks};

    let tracks: i64 = tracks::table
        .select(count(tracks::id))
        .first::<i64>(conn)
        .unwrap_or(0);
    let albums: i64 = albums::table
        .select(count(albums::id))
        .first::<i64>(conn)
        .unwrap_or(0);
    let artists: i64 = artists::table
        .select(count(artists::id))
        .first::<i64>(conn)
        .unwrap_or(0);

    (tracks, albums, artists)
}
