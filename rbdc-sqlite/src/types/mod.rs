//! Conversions between Rust and **SQLite** types.
//!
//! # Types
//!
//! | Rust type                             | SQLite type(s)                                       |
//! |---------------------------------------|------------------------------------------------------|
//! | `bool`                                | BOOLEAN                                              |
//! | `i8`                                  | INTEGER                                              |
//! | `i16`                                 | INTEGER                                              |
//! | `i32`                                 | INTEGER                                              |
//! | `i64`                                 | BIGINT, INT8                                         |
//! | `u8`                                  | INTEGER                                              |
//! | `u16`                                 | INTEGER                                              |
//! | `u32`                                 | INTEGER                                              |
//! | `u64`                                 | BIGINT, INT8                                         |
//! | `f32`                                 | REAL                                                 |
//! | `f64`                                 | REAL                                                 |
//! | `&str`, [`String`]                    | TEXT                                                 |
//! | `&[u8]`, `Vec<u8>`                    | BLOB                                                 |
//! # Nullable
//!
//! In addition, `Option<T>` is supported where `T` implements `Type`. An `Option<T>` represents
//! a potentially `NULL` value from SQLite.
//!

mod bool;
mod bytes;
mod float;
mod int;
mod str;
mod uint;
use crate::type_info::Type;
mod value;

#[cfg(test)]
mod test {
    #[test]
    fn test_datetime() {}
}
