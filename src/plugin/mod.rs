//! Plugins shipped with rbatis.
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`page`] | pagination: [`Page`], [`PageRequest`] and the `Page<T>` return type |
//! | [`intercept`] | hooks that can rewrite SQL before it runs and inspect the result afterwards ([`Intercept`], logging, pagination) |
//! | [`id_generator`] | distributed id generators: [`Snowflake`] and [`ObjectId`] |
//! | [`table_sync`] | create missing tables and columns from a struct or a `rbs::Value` map |
//!
//! [`RBatis::new`](crate::RBatis::new) installs the page and log interceptors for you;
//! the other plugins have to be plugged in explicitly:
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use rbatis::RBatis;
//! use rbatis::plugin::intercept_log::LogInterceptor;
//! use rbatis::plugin::intercept_page::PageIntercept;
//!
//! let mut rb = RBatis::new();
//! rb.set_intercepts(vec![
//!     Arc::new(PageIntercept::new()),
//!     Arc::new(LogInterceptor::new(log::LevelFilter::Debug)),
//! ]);
//! ```

/// distributed id generators, see [`id_generator::Snowflake`] and [`id_generator::ObjectId`]
pub mod id_generator;
/// SQL interceptors, see [`Intercept`]
pub mod intercept;
/// pagination types, see [`Page`] and [`PageRequest`]
pub mod page;
/// create/update tables from structs, see [`table_sync::sync`]
pub mod table_sync;

pub use id_generator::*;
pub use intercept::*;
pub use page::*;
