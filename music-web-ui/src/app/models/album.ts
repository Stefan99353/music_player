export interface Album {
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
}
