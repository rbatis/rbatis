use crate::Error;
use futures_core::future::BoxFuture;
use rbs::value::map::ValueMap;
use rbs::Value;
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};

/// Represents database driver that can be shared between threads, and can therefore implement
/// a connection pool
pub trait Driver: Debug + Sync + Send {
    fn name(&self) -> &str;
    /// Create a connection to the database. Note that connections are intended to be used
    /// in a single thread since most database connections are not thread-safe
    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>>;

    fn connect_opt<'a>(
        &'a self,
        opt: &'a dyn ConnectOptions,
    ) -> BoxFuture<Result<Box<dyn Connection>, Error>>;

    /// make an default option
    fn default_option(&self) -> Box<dyn ConnectOptions>;
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ExecResult {
    pub rows_affected: u64,
    /// If some databases do not support last_insert_id, the default value is Null
    pub last_insert_id: Value,
}

impl Display for ExecResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .key(&"rows_affected")
            .value(&self.rows_affected)
            .key(&"last_insert_id")
            .value(&self.last_insert_id)
            .finish()
    }
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
                let mut m = ValueMap::with_capacity(md.column_len());
                for mut i in 0..md.column_len() {
                    i = md.column_len() - i - 1;
                    let n = md.column_name(i);
                    m.insert(Value::String(n), x.get(i)?);
                }
                rows.push(Value::Map(m));
            }
            Ok(rows)
        })
    }

    /// Execute a query that is expected to update some rows.
    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>>;

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
    fn get(&mut self, i: usize) -> Result<Value,Error>;
}

/// Meta data for result set
pub trait MetaData: Debug {
    fn column_len(&self) -> usize;
    fn column_name(&self, i: usize) -> String;
    fn column_type(&self, i: usize) -> String;
}

/// connect option
pub trait ConnectOptions: Any + Send + Sync + Debug + 'static {
    /// Establish a new database connection with the options specified by `self`.
    fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>>;

    ///set option
    ///
    /// for exmample:
    ///
    /// ```rust
    /// pub struct SqliteConnectOptions{
    ///   pub immutable:bool,
    /// };
    ///
    /// let mut d = SqliteConnectOptions{immutable:false};
    ///         d.set(Box::new({
    ///             let mut new = SqliteConnectOptions::new();
    ///             new.immutable=true;
    ///             new
    ///         }));
    /// ```
    ///
    #[inline]
    fn set(&mut self, arg: Box<dyn Any>)
    where
        Self: Sized,
    {
        *self = *arg.downcast().expect("must be self type!");
    }

    ///set option from uri
    fn set_uri(&mut self, uri: &str) -> Result<(), Error>;

    /// uppercase self,default is this code
    ///```rust
    /// use std::any::Any;
    /// use futures_core::future::BoxFuture;
    /// use rbdc::db::{Connection, ConnectOptions};
    /// use rbdc::Error;
    /// #[derive(Debug)]
    /// pub struct MyConnectOptions{}
    ///
    /// impl ConnectOptions for MyConnectOptions{
    ///   fn connect(&self) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
    ///         todo!()
    ///     }
    ///
    ///    fn set_uri(&mut self, uri: &str) -> Result<(), Error> {
    ///         todo!()
    ///     }
    ///
    ///    fn uppercase_self(&self) -> &(dyn Any + Send + Sync) {
    ///         self
    ///    }
    ///
    /// }
    /// ```
    fn uppercase_self(&self) -> &(dyn Any + Send + Sync);
}


/// database driver ConnectOptions
impl dyn ConnectOptions {
    pub fn downcast_ref<E: ConnectOptions>(&self) -> Option<&E> {
        self.uppercase_self().downcast_ref()
    }
}

/// make all database drivers support dialect '?'
///
/// for example: postgres driver
/// ```log
///  "select * from  table where name = ？"
/// ```
/// to
/// ```log
/// "select * from  table where name =  $1"
pub trait Placeholder {
    fn exchange(&self, sql: &str) -> String;
}
