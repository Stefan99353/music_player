table! {
    albums (id) {
        id -> Integer,
        title -> Text,
        track_count -> Nullable<Integer>,
        disc_count -> Nullable<Integer>,
        year -> Nullable<Integer>,
        rating -> Nullable<Float>,
        image_id -> Nullable<Integer>,
        artist_id -> Integer,
        inserted -> Nullable<Timestamp>,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    artists (id) {
        id -> Integer,
        name -> Text,
        image_id -> Nullable<Integer>,
        inserted -> Nullable<Timestamp>,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    db_updates (id) {
        id -> Integer,
        started -> Timestamp,
        finished -> Timestamp,
        tracks_before -> Integer,
        tracks_after -> Integer,
        albums_before -> Integer,
        albums_after -> Integer,
        artists_before -> Integer,
        artists_after -> Integer,
        inserted -> Nullable<Timestamp>,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    images (id) {
        id -> Integer,
        path -> Text,
        inserted -> Nullable<Timestamp>,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    tracks (id) {
        id -> Integer,
        path -> Text,
        title -> Text,
        date -> Nullable<Timestamp>,
        genre -> Nullable<Text>,
        rating -> Nullable<Float>,
        track_number -> Nullable<Integer>,
        disc_number -> Nullable<Integer>,
        duration -> Integer,
        image_id -> Nullable<Integer>,
        album_id -> Integer,
        size -> Integer,
        inserted -> Nullable<Timestamp>,
        updated -> Nullable<Timestamp>,
    }
}

table! {
    populated_tracks (id) {
        id -> Integer,
        path -> Text,
        title -> Text,
        date -> Nullable<Timestamp>,
        genre -> Nullable<Text>,
        rating -> Nullable<Float>,
        track_number -> Nullable<Integer>,
        disc_number -> Nullable<Integer>,
        duration -> Integer,
        image_id -> Nullable<Integer>,
        album_id -> Integer,
        artist_id -> Integer,
        album_title -> Text,
        artist_name -> Text,
        size -> Integer,
        inserted -> Nullable<Timestamp>,
        updated -> Nullable<Timestamp>,
    }
}

joinable!(albums -> artists (artist_id));
joinable!(albums -> images (image_id));
joinable!(artists -> images (image_id));
joinable!(tracks -> albums (album_id));
joinable!(tracks -> images (image_id));
joinable!(populated_tracks -> albums (album_id));
joinable!(populated_tracks -> artists (artist_id));

allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    db_updates,
    images,
    tracks,
    populated_tracks,
);
