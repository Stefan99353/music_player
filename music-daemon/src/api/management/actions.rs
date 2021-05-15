use std::sync::Arc;
use std::thread;

use actix::Addr;
use chrono::Utc;
use diesel::prelude::*;

use crate::api::get_db_connection;
use crate::crawler::Crawler;
use crate::DbPool;
use crate::models::playlists::Playlist;
use crate::settings::Settings;
use crate::ws::notifications::hub::WsNotificationHub;
use crate::ws::notifications::messages::{Notification, NotificationType};

pub fn clear_db(
    pool: Arc<DbPool>,
    settings: Arc<Settings>,
    notifications: Arc<Addr<WsNotificationHub>>,
) {
    thread::spawn(move || {
        info!("Started clearing database");
        notifications.do_send(Notification {
            message: String::from("Started clearing database"),
            message_type: NotificationType::Info,
            timestamp: Utc::now().naive_utc(),
        });

        let conn = match get_db_connection(pool.clone()) {
            Ok(conn) => conn,
            Err(err) => {
                notifications.do_send(Notification {
                    message: String::from("Error getting database connection"),
                    message_type: NotificationType::Error,
                    timestamp: Utc::now().naive_utc(),
                });
                error!("{}", err);
                return;
            }
        };

        use crate::schema::{albums, artists, images, playlist_track, playlists, tracks};

        // Delete Entities
        // playlist_track
        unwrap_db_delete_result(diesel::delete(playlist_track::table).execute(&conn), &notifications);
        // playlists
        unwrap_db_delete_result(diesel::delete(playlists::table).execute(&conn), &notifications);
        // tracks
        unwrap_db_delete_result(diesel::delete(tracks::table).execute(&conn), &notifications);
        // albums
        unwrap_db_delete_result(diesel::delete(albums::table).execute(&conn), &notifications);
        // artists
        unwrap_db_delete_result(diesel::delete(artists::table).execute(&conn), &notifications);
        // images
        unwrap_db_delete_result(diesel::delete(images::table).execute(&conn), &notifications);

        // Add Favorites Playlist
        let playlist = Playlist {
            id: 0,
            name: "Favorites".to_string(),
            icon: Some("favorite".to_string()),
            description: Some("Playlist with all your favorite tracks".to_string()),
            inserted: None,
            updated: None,
        };
        match playlist.insert(&conn) {
            Ok(_) => {}
            Err(err) => {
                notifications.do_send(Notification {
                    message: String::from("Error inserting row in database"),
                    message_type: NotificationType::Error,
                    timestamp: Utc::now().naive_utc(),
                });
                error!("{}", err);
            }
        }

        info!("Finished clearing database");

        let paths = settings
            .music
            .iter()
            .map(|d| String::from(&d.path))
            .collect::<Vec<String>>();

        Crawler::new(
            paths,
            pool,
            notifications,
        ).start();
    });
}

fn unwrap_db_delete_result(res: QueryResult<usize>, notifications: &Arc<Addr<WsNotificationHub>>) {
    match res {
        Ok(_) => {}
        Err(err) => {
            notifications.do_send(Notification {
                message: String::from("Error deleting rows in database"),
                message_type: NotificationType::Error,
                timestamp: Utc::now().naive_utc(),
            });
            error!("{}", err);
        }
    }
}
