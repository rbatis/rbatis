use futures_core::future::BoxFuture;

use super::PoolConnection;
use crate::connection::Connect;
use crate::cursor::{Cursor, HasCursor};
use crate::database::Database;
use crate::describe::Describe;
use crate::executor::Execute;
use crate::executor::{Executor, RefExecutor};
use crate::pool::Pool;

impl<'p, C, DB> Executor for &'p Pool<C>
where
    C: Connect<Database = DB>,
    DB: Database<Connection = C>,
    DB: for<'c, 'q> HasCursor<'c, 'q, Database = DB>,
{
    type Database = DB;

    fn execute<'e, 'q: 'e, 'c: 'e, E: 'e>(
        &'c mut self,
        query: E,
    ) -> BoxFuture<'e, crate::Result<u64>>
    where
        E: Execute<'q, Self::Database>,
    {
        Box::pin(async move { self.acquire().await?.execute(query).await })
    }

    fn cursor<'e, 'q, E>(&'e mut self, query: E) -> <Self::Database as HasCursor<'_, 'q>>::Cursor
    where
        E: Execute<'q, DB>,
    {
        DB::Cursor::from_pool(self, query)
    }

    #[doc(hidden)]
    fn describe<'e, 'q, E: 'e>(
        &'e mut self,
        query: E,
    ) -> BoxFuture<'e, crate::Result<Describe<Self::Database>>>
    where
        E: Execute<'q, Self::Database>,
    {
        Box::pin(async move { self.acquire().await?.describe(query).await })
    }
}

impl<'p, C, DB> RefExecutor<'p> for &'p Pool<C>
where
    C: Connect<Database = DB>,
    DB: Database<Connection = C>,
    DB: for<'c, 'q> HasCursor<'c, 'q>,
    for<'c> &'c mut C: RefExecutor<'c>,
{
    type Database = DB;

    fn fetch_by_ref<'q, E>(self, query: E) -> <Self::Database as HasCursor<'p, 'q>>::Cursor
    where
        E: Execute<'q, DB>,
    {
        DB::Cursor::from_pool(self, query)
    }
}

impl<C> Executor for PoolConnection<C>
where
    C: Connect,
{
    type Database = C::Database;

    fn execute<'e, 'q: 'e, 'c: 'e, E: 'e>(
        &'c mut self,
        query: E,
    ) -> BoxFuture<'e, crate::Result<u64>>
    where
        E: Execute<'q, Self::Database>,
    {
        (**self).execute(query)
    }

    fn cursor<'e, 'q, E>(&'e mut self, query: E) -> <C::Database as HasCursor<'_, 'q>>::Cursor
    where
        E: Execute<'q, Self::Database>,
    {
        (**self).cursor(query)
    }

    #[doc(hidden)]
    fn describe<'e, 'q, E: 'e>(
        &'e mut self,
        query: E,
    ) -> BoxFuture<'e, crate::Result<Describe<Self::Database>>>
    where
        E: Execute<'q, Self::Database>,
    {
        (**self).describe(query)
    }
}

impl<'c, C, DB> RefExecutor<'c> for &'c mut PoolConnection<C>
where
    C: Connect<Database = DB>,
    DB: Database<Connection = C>,
    DB: for<'c2, 'q> HasCursor<'c2, 'q, Database = DB>,
    &'c mut C: RefExecutor<'c, Database = DB>,
{
    type Database = DB;

    fn fetch_by_ref<'q, E>(self, query: E) -> <Self::Database as HasCursor<'c, 'q>>::Cursor
    where
        E: Execute<'q, Self::Database>,
    {
        (**self).cursor(query)
    }
}
