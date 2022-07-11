use rbs::Value;
use crate::Error;

/// Represents database driver that can be shared between threads, and can therefore implement
/// a connection pool
pub trait Driver: Sync + Send {
    /// Create a connection to the database. Note that connections are intended to be used
    /// in a single thread since most database connections are not thread-safe
    fn connect(&self, url: &str) -> Result<Box<dyn Connection>, Error>;
}

/// Represents a connection to a database
pub trait Connection {
    /// Create a statement for execution
    fn create(&mut self, sql: &str) -> Result<Box<dyn Statement + '_>, Error>;

    /// Create a prepared statement for execution
    fn prepare(&mut self, sql: &str) -> Result<Box<dyn Statement + '_>, Error>;
}

/// Represents an executable statement
pub trait Statement {
    /// Execute a query that is expected to return a result set, such as a `SELECT` statement
    fn fetch(&mut self, params: &[Value]) -> Result<Box<dyn ResultSet + '_>, Error>;

    /// Execute a query that is expected to update some rows.
    fn exec(&mut self, params: &[Value]) -> Result<u64, Error>;
}

/// Result set from executing a query against a statement
pub trait ResultSet {
    /// get meta data about this result set
    fn meta_data(&self) -> Result<Box<dyn ResultSetMetaData>, Error>;

    /// Move the cursor to the next available row if one exists and return true if it does
    fn next(&mut self) -> bool;

    fn get_v(&self, i: u64) -> Result<Value, Error>;
}

/// Meta data for result set
pub trait ResultSetMetaData {
    fn column_len(&self) -> u64;
    fn column_name(&self, i: usize) -> String;
    fn column_type(&self, i: usize) -> String;
}


#[cfg(test)]
mod test {
    use crate::db::{Connection, Driver};
    use crate::Error;

    pub struct M {}

    impl Driver for M {
        fn connect(&self, url: &str) -> Result<Box<dyn Connection>, Error> {
            todo!()
        }
    }

    #[test]
    fn test_db() {
        let b: Box<dyn Driver> = Box::new(M {});
    }
}
