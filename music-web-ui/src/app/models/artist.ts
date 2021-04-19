export class Artist {
  id: number;
  name: string;
  imageId?: number | undefined;
  inserted: Date;
  updated: Date;


  constructor(
    id: number,
    name: string,
    imageId: number | undefined,
    inserted: Date,
    updated: Date
  ) {
    this.id = id;
    this.name = name;
    this.imageId = imageId;
    this.inserted = inserted;
    this.updated = updated;
  }
}
