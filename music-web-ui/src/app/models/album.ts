export interface Album {
  id: number;
  title: string;
  trackCount: number | null;
  discCount: number | null;
  year: number | null;
  rating: number | null;
  imageId: number | null;
  artistId: number;
  inserted: Date;
  updated: Date;
}
