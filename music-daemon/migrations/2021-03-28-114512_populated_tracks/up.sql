-- Your SQL goes here

CREATE VIEW populated_tracks AS
SELECT tr.id,
       tr.path,
       tr.title,
       tr.date,
       tr.genre,
       tr.rating,
       tr.track_number,
       tr.disc_number,
       tr.duration,
       tr.image_id,
       tr.album_id,
       ar.id    AS artist_id,
       al.title AS album_title,
       ar.name  AS artist_name,
       tr.size,
       tr.inserted,
       tr.updated
FROM tracks tr
         JOIN albums al ON al.id = tr.album_id
         JOIN artists ar ON al.artist_id = ar.id;
