use crate::Error;
use futures_core::future::BoxFuture;
use rbs::Value;
use std::fmt::Debug;
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
    /// Create a statement for execution
    fn create(&mut self, sql: &str) -> BoxFuture<Result<Box<dyn Statement>, Error>>;

    /// Create a prepared statement for execution
    fn prepare(&mut self, sql: &str) -> BoxFuture<Result<Box<dyn Statement>, Error>>;
}

/// Represents an executable statement
pub trait Statement {
    /// Execute a query that is expected to return a result set, such as a `SELECT` statement
    fn fetch(&mut self, params: Vec<Value>) -> BoxFuture<Result<Box<dyn ResultSet>, Error>>;

    /// Execute a query that is expected to update some rows.
    fn exec(&mut self, params: Vec<Value>) -> BoxFuture<Result<u64, Error>>;
}

/// Result set from executing a query against a statement
pub trait ResultSet {
    /// get meta data about this result set
    fn meta_data(&self) -> Result<Box<dyn MetaData>, Error>;

    /// Move the cursor to the next available row if one exists and return true if it does
    fn next(&mut self) -> bool;

    /// get Value from index
    fn get(&self, i: u64) -> Result<Value, Error>;
}

/// Meta data for result set
pub trait MetaData {
    fn column_len(&self) -> u64;
    fn column_name(&self, i: usize) -> String;
    fn column_type(&self, i: usize) -> String;
}

/// connect option
pub trait ConnectOptions: 'static + Send + Sync + FromStr<Err = Error> + Debug + Clone {
    /// Establish a new database connection with the options specified by `self`.
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>>;
}
