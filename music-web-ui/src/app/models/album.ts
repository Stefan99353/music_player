export class Album {
  id: number;
  title: string;
  trackCount?: number | undefined;
  discCount?: number | undefined;
  year?: number | undefined;
  rating?: number | undefined;
  imageId?: number | undefined;
  artistId: number;
  inserted: Date;
  updated: Date;


  constructor(
    id: number,
    title: string,
    trackCount: number | undefined,
    discCount: number | undefined,
    year: number | undefined,
    rating: number | undefined,
    imageId: number | undefined,
    artistId: number,
    inserted: Date, updated: Date
  ) {
    this.id = id;
    this.title = title;
    this.trackCount = trackCount;
    this.discCount = discCount;
    this.year = year;
    this.rating = rating;
    this.imageId = imageId;
    this.artistId = artistId;
    this.inserted = inserted;
    this.updated = updated;
  }
}
