//! **SQLite** database driver.

// SQLite is a C library. All interactions require FFI which is unsafe.
// All unsafe blocks should have comments pointing to SQLite docs and ensuring that we maintain
// invariants.
#![allow(unsafe_code)]

pub use arguments::{SqliteArgumentValue, SqliteArguments};
pub use column::SqliteColumn;
pub use connection::{LockedSqliteHandle, SqliteConnection};
pub use database::Sqlite;
pub use error::SqliteError;
pub use options::{
    SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode, SqliteSynchronous,
};
pub use query_result::SqliteQueryResult;
pub use row::SqliteRow;
pub use statement::SqliteStatement;
pub use type_info::SqliteTypeInfo;
pub use value::{SqliteValue, SqliteValueRef};

pub mod driver;
pub mod encode;
pub mod decode;
pub mod arguments;
pub mod column;
pub mod connection;
pub mod database;
pub mod error;
pub mod options;
pub mod query_result;
pub mod row;
pub mod statement;
pub mod type_info;
pub mod types;
pub mod value;
pub mod query;
