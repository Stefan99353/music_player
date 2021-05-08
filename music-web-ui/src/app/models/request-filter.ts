import {HttpParams} from '@angular/common/http';

export interface RequestFilter {
  [key: string]: any;

  filter?: string;
  sort?: string;
  order?: string;
  page?: number;
  limit?: number;
}

export function buildParams(filter: RequestFilter): HttpParams {
  let params = new HttpParams();

  const keys = Object.keys(filter);
  keys.forEach(key => {
    if (filter[key] !== undefined) {
      params = params.set(key, filter[key].toString());
    }
  });

  return params;
}
