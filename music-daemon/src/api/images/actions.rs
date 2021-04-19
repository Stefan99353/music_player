use diesel::prelude::*;

use crate::models::images::Image;

pub fn find_image_by_id(
    image_id: i32,
    conn: &SqliteConnection,
) -> Result<Option<Image>, diesel::result::Error> {
    use crate::schema::images::dsl::*;

    let res = images
        .filter(id.eq(image_id))
        .first::<Image>(conn)
        .optional()?;

    Ok(res)
}

pub fn find_album_image(
    album_id: i32,
    conn: &SqliteConnection,
) -> Result<Option<Image>, diesel::result::Error> {
    use crate::schema::{images, albums};

    let res: Option<Image> = images::table
        .select(images::all_columns)
        .inner_join(albums::table)
        .filter(albums::id.eq(album_id))
        .first::<Image>(conn)
        .optional()?;

    Ok(res)
}

pub fn find_artist_image(
    artist_id: i32,
    conn: &SqliteConnection,
) -> Result<Option<Image>, diesel::result::Error> {
    use crate::schema::{images, artists};

    let res: Option<Image> = images::table
        .select(images::all_columns)
        .inner_join(artists::table)
        .filter(artists::id.eq(artist_id))
        .first::<Image>(conn)
        .optional()?;

    Ok(res)
}
