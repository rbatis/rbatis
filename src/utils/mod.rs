//! Helper macros, mostly used by the code generated from `crud!` and the SQL
//! attribute macros.
//!
//! * [`table!`](crate::table) builds a struct from its `Default` value and a list of fields.
//! * [`table_field_vec!`](crate::table_field_vec), [`table_field_set!`](crate::table_field_set),
//!   [`table_field_map!`](crate::table_field_map) and
//!   [`table_field_btree!`](crate::table_field_btree) collect one field of a
//!   `Vec<Struct>` (typically the primary keys used in an `IN (...)` clause).
//! * [`field_name!`](crate::field_name) / [`field_key!`](crate::field_key) turn a
//!   field name into a `&str`, which keeps string keys and struct fields in sync.
//! * [`impled!`](crate::impled) is the placeholder body of every function handled by
//!   `#[py_sql]` / `#[html_sql]` / `#[sql]`: the attribute macro replaces it with
//!   the real implementation.

/// macros that read fields out of a `Vec<Struct>`
#[macro_use]
pub mod table_util;
/// the [`crate::impled!`] placeholder body used by the SQL attribute macros
pub mod impled;
