use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::LoadQuery;
use diesel::sql_types::{HasSqlType, Integer};
use diesel::sqlite::Sqlite;
use serde::Serialize;

const DEFAULT_PAGE_SIZE: i32 = 10;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationResult<T> {
    pub page: Option<i32>,
    pub page_count: Option<i32>,
    pub page_size: Option<i32>,
    pub total_count: Option<i32>,
    pub items: Vec<T>,
}

#[derive(QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i32,
    page_size: i32,
}

pub trait Paginate: Sized {
    fn paginate(self, page: i32) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i32) -> Paginated<Self> {
        Paginated {
            query: self,
            page,
            page_size: DEFAULT_PAGE_SIZE,
        }
    }
}

impl<T> QueryFragment<Sqlite> for Paginated<T>
    where
        T: QueryFragment<Sqlite>,
{
    fn walk_ast(&self, mut out: AstPass<Sqlite>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<Integer, _>(&self.page_size)?;
        out.push_sql(" OFFSET ");
        let offset = self.page * self.page_size;
        out.push_bind_param::<Integer, _>(&offset)?;

        Ok(())
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, Integer);
}

impl<T> RunQueryDsl<SqliteConnection> for Paginated<T> {}

impl<T> Paginated<T> {
    pub fn page_size(self, page_size: i32) -> Self {
        Paginated { page_size, ..self }
    }

    pub fn load_and_count<U>(self, conn: &SqliteConnection) -> QueryResult<(Vec<U>, i32)>
        where
            Self: LoadQuery<SqliteConnection, (U, i32)>,
    {
        let results = self.load::<(U, i32)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();

        Ok((records, total))
    }
}

pub trait LoadPaginated<U>: Query + QueryId + QueryFragment<Sqlite> + LoadQuery<SqliteConnection, U> {
    fn load_with_pagination(self, conn: &SqliteConnection, page: Option<i32>, page_size: Option<i32>) -> QueryResult<PaginationResult<U>>;
}

impl<T, U> LoadPaginated<U> for T
    where
        Self: Query + QueryId + QueryFragment<Sqlite> + LoadQuery<SqliteConnection, U>,
        U: Queryable<Self::SqlType, Sqlite>,
        Sqlite: HasSqlType<Self::SqlType>,
{
    fn load_with_pagination(
        self, conn: &SqliteConnection,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> QueryResult<PaginationResult<U>> {
        let mut result = PaginationResult {
            page,
            page_count: None,
            page_size: None,
            total_count: None,
            items: vec![],
        };

        match page {
            None => {
                let items = self.load::<U>(conn)?;
                result.items = items;
            }
            Some(page) => {
                let mut query = self.paginate(page);

                let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);

                query = query.page_size(page_size);
                result.page_size = Some(page_size);

                let (items, total) = query.load_and_count::<U>(conn)?;
                result.items = items;
                result.total_count = Some(total);
                result.page_count = Some((total as f64 / page_size as f64).ceil() as i32);
            }
        }

        Ok(result)
    }
}
