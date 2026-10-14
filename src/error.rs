//! The error type used by every rbatis API.
//!
//! [`Error`] is a re-export of [`rbdc::Error`], so driver errors, decode errors and
//! rbatis' own errors all travel in the same type. [`Result`] is the matching alias,
//! which is why generated code returns `rbatis::Result<T>`.
//!
//! ```rust
//! use rbatis::{Error, RBatis};
//!
//! async fn run(rb: &RBatis) -> rbatis::Result<()> {
//!     let _ = rb.driver_type()?;
//!     Ok(())
//! }
//!
//! // `Error` implements `From<&str>` and `std::error::Error`, so `?` and `anyhow` work
//! let _e: Error = Error::from("something went wrong");
//! ```

/// `Result<T, Error>` returned by every fallible rbatis API.
pub type Result<T> = std::result::Result<T, Error>;

/// re-export of [`rbdc::Error`], the error type shared by all database drivers
pub type Error = rbdc::Error;
