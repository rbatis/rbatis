use crate::Error;
use futures_core::future::BoxFuture;
use rbs::value::map::ValueMap;
use rbs::Value;
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};

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

impl Driver for Box<dyn Driver>{
    fn name(&self) -> &str {
        self.deref().name()
    }

    fn connect(&self, url: &str) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        self.deref().connect(url)
    }

    fn connect_opt<'a>(&'a self, opt: &'a dyn ConnectOptions) -> BoxFuture<Result<Box<dyn Connection>, Error>> {
        self.deref().connect_opt(opt)
    }

    fn default_option(&self) -> Box<dyn ConnectOptions> {
        self.deref().default_option()
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
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

    /// ping
    fn ping(&mut self) -> BoxFuture<Result<(), Error>>;

    /// close connection
    /// Normally conn is dropped when the link is dropped,
    /// but it is recommended to actively close this function so that the database does not report errors.
    /// If &mut self is not satisfied close, when you need mut self,
    /// It is recommended to use Option<DataBaseConnection>
    /// and then call take to take ownership and then if let Some(v) = self.inner.take() {v.lose ().await; }
    fn close(&mut self) -> BoxFuture<Result<(), Error>>;
}

impl Connection for Box<dyn Connection>{
    fn get_rows(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<Vec<Box<dyn Row>>, Error>> {
        self.deref_mut().get_rows(sql,params)
    }

    fn exec(&mut self, sql: &str, params: Vec<Value>) -> BoxFuture<Result<ExecResult, Error>> {
        self.deref_mut().exec(sql,params)
    }

    fn ping(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().ping()
    }

    fn close(&mut self) -> BoxFuture<Result<(), Error>> {
        self.deref_mut().close()
    }
}


/// Result set from executing a query against a statement
pub trait Row: 'static + Send + Debug {
    /// get meta data about this result set
    fn meta_data(&self) -> Box<dyn MetaData>;

    /// get Value from index
    fn get(&mut self, i: usize) -> Result<Value, Error>;
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
    ///```rust
    /// use std::any::Any;
    /// pub struct SqliteConnectOptions{
    ///   pub immutable:bool,
    /// };
    ///  impl SqliteConnectOptions{
    ///             pub fn new()->Self{
    ///                 Self{
    ///                     immutable: false,
    ///                 }
    ///             }
    ///             fn set(&mut self, arg: Box<dyn Any>){
    ///             }
    ///         }
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
}

/// database driver ConnectOptions
impl dyn ConnectOptions {
    pub fn downcast_ref<E: ConnectOptions>(&self) -> Option<&E> {
        let v = unsafe {
            //this is safe
            std::mem::transmute_copy::<&dyn ConnectOptions, &E>(&self)
        };
        Some(v)
    }

    pub fn downcast_ref_mut<E: ConnectOptions>(&mut self) -> Option<&mut E> {
        let v = unsafe {
            //this is safe
            std::mem::transmute_copy::<&mut dyn ConnectOptions, &mut E>(&self)
        };
        Some(v)
    }
}

/// make all database drivers support dialect '?'
/// you can use util package to impl this
/// for example:
/// ```rust
/// use rbdc::db::Placeholder;
/// pub struct MyPgDriver{}
/// impl Placeholder for MyPgDriver{
///     fn exchange(&self, sql: &str) -> String {
///         rbdc::impl_exchange("$",1,sql)
///     }
/// }
/// ```
///
/// for example: postgres driver
/// ```log
///  "select * from  table where name = ?"
/// ```
/// to
/// ```log
/// "select * from  table where name =  $1"
pub trait Placeholder {
    fn exchange(&self, sql: &str) -> String;
}
