export interface Track {
  id: number;
  path: string;
  title: string;
  date: Date | null;
  genre: string | null;
  rating: number | null;
  trackNumber: number | null;
  discNumber: number | null;
  duration: number;
  imageId: number | null;
  artistId: number;
  albumId: number;
  artistName: string;
  albumTitle: string;
  size: number;
  inserted: Date;
  updated: Date;
}
