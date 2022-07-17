use crate::Error;
use futures_core::future::BoxFuture;
use rbs::Value;
use std::fmt::Debug;

/// Represents database driver that can be shared between threads, and can therefore implement
/// a connection pool
pub trait Driver: Sync + Send {
    /// Create a connection to the database. Note that connections are intended to be used
    /// in a single thread since most database connections are not thread-safe
    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>>;

    /// make option
    fn make_option(&self, url: &str) -> Result<Box<dyn ConnectOptions>, Error>;
}

/// Represents a connection to a database
pub trait Connection: Send {
    /// Execute a query that is expected to return a result set, such as a `SELECT` statement
    fn get_rows(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>>;

    /// Execute a query that is expected to return a result set, such as a `SELECT` statement
    fn get_values(
        &mut self,
        sql: &str,
        params: Vec<Value>,
    ) -> BoxFuture<Result<Vec<Value>, Error>> {
        let v = self.get_rows(sql, params);
        Box::pin(async move {
            let v = v.await?;
            let mut rows = Vec::with_capacity(v.len());
            for mut x in v {
                let md = x.meta_data();
                let mut m = Vec::with_capacity(md.column_len());
                for i in 0..md.column_len() {
                    let n = md.column_name(i);
                    m.push((Value::String(n), x.get(i).unwrap_or(Value::Null)));
                }
                rows.push(Value::Map(m));
            }
            Ok(rows)
        })
    }

    /// Execute a query that is expected to update some rows.
    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<u64, Error>>;

    /// close connection
    fn close(&mut self) -> BoxFuture<Result<(), Error>>;

    /// ping
    fn ping(&mut self) -> BoxFuture<Result<(), Error>>;
}

/// Result set from executing a query against a statement
pub trait Row: 'static + Send + Debug {
    /// get meta data about this result set
    fn meta_data(&self) -> Box<dyn MetaData>;

    /// get Value from index
    fn get(&mut self, i: usize) -> Option<Value>;
}

/// Meta data for result set
pub trait MetaData: Debug {
    fn column_len(&self) -> usize;
    fn column_name(&self, i: usize) -> String;
    fn column_type(&self, i: usize) -> String;
}

/// connect option
pub trait ConnectOptions: 'static + Send + Sync + Debug {
    /// Establish a new database connection with the options specified by `self`.
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>>;
}
