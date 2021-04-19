export class Track {
  id: number;
  path: string;
  title: string;
  date?: Date | undefined;
  genre?: string | undefined;
  rating?: number | undefined;
  trackNumber?: number | undefined;
  discNumber?: number | undefined;
  duration: number;
  imageId?: number | undefined;
  artistId: number;
  albumId: number;
  artistName: string;
  albumTitle: string;
  size: number;
  inserted: Date;
  updated: Date;


  constructor(
    id: number,
    path: string,
    title: string,
    date: Date | undefined,
    genre: string | undefined,
    rating: number | undefined,
    trackNumber: number | undefined,
    discNumber: number | undefined,
    duration: number,
    imageId: number | undefined,
    artistId: number,
    albumId: number,
    artistName: string,
    albumTitle: string,
    size: number,
    inserted: Date,
    updated: Date
  ) {
    this.id = id;
    this.path = path;
    this.title = title;
    this.date = date;
    this.genre = genre;
    this.rating = rating;
    this.trackNumber = trackNumber;
    this.discNumber = discNumber;
    this.duration = duration;
    this.imageId = imageId;
    this.artistId = artistId;
    this.albumId = albumId;
    this.artistName = artistName;
    this.albumTitle = albumTitle;
    this.size = size;
    this.inserted = inserted;
    this.updated = updated;
  }
}
