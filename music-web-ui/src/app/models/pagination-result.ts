export class PaginationResult<T> {
  page = 0;
  pageCount = 0;
  pageSize = 0;
  totalCount = 0;
  items: T[] = [];
}
