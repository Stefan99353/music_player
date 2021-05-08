export interface Track {
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
}
