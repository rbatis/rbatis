use crate::Error;
use std::alloc;

use futures_core::future::BoxFuture;
use rbs::Value;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;

/// Represents database driver that can be shared between threads, and can therefore implement
/// a connection pool
pub trait Driver: Sync + Send {
    /// Create a connection to the database. Note that connections are intended to be used
    /// in a single thread since most database connections are not thread-safe
    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>>;
}

/// Represents a connection to a database
pub trait Connection {
    /// Execute a query that is expected to return a result set, such as a `SELECT` statement
    fn fetch(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>>;

    /// Execute a query that is expected to update some rows.
    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<u64, Error>>;
}

/// Result set from executing a query against a statement
pub trait Row: 'static + Send {
    /// get meta data about this result set
    fn meta_data(&self) -> &dyn MetaData;

    /// get Value from index
    fn get(&self, i: usize) -> Option<Value>;
}

/// Meta data for result set
pub trait MetaData {
    fn column_len(&self) -> usize;
    fn column_name(&self, i: usize) -> String;
    fn column_type(&self, i: usize) -> String;
}

/// connect option
pub trait ConnectOptions: 'static + Send + Sync + FromStr<Err = Error> + Debug + Clone {
    /// Establish a new database connection with the options specified by `self`.
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>>;
}
