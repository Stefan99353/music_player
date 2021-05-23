import {HttpParams} from '@angular/common/http';

export interface RequestFilter {
  [key: string]: any;

  filter: string | null;
  sort: string | null;
  order: string | null;
  page: number | null;
  limit: number | null;
}

export function buildParams(filter: RequestFilter): HttpParams {
  let params = new HttpParams();

  const keys = Object.keys(filter);
  keys.forEach(key => {
    if (filter[key] !== null) {
      params = params.set(key, filter[key].toString());
    }
  });

  return params;
}
