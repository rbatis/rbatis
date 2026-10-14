//! Decoding rows into your own types.
//!
//! Every query in rbatis returns rows as [`rbs::Value`] (an array of maps). The
//! functions here turn that into anything that implements
//! [`serde::Deserialize`](serde::de::DeserializeOwned) - a struct, a `Vec<T>`, a
//! scalar or an `Option<T>`:
//!
//! ```rust,no_run
//! use rbatis::{RBatis, decode};
//!
//! #[derive(Debug, serde::Serialize, serde::Deserialize)]
//! pub struct BizActivity {
//!     pub id: Option<String>,
//!     pub name: Option<String>,
//! }
//!
//! async fn run(rb: &RBatis) -> rbatis::Result<()> {
//!     // decoded automatically by the typed helpers ...
//!     let rows: Vec<BizActivity> =
//!         rb.exec_decode("select * from biz_activity", vec![]).await?;
//!     let count: u64 =
//!         rb.exec_decode("select count(1) as count from biz_activity", vec![]).await?;
//!
//!     // ... or manually, from a value produced by `Executor::query`
//!     let value = rb.query("select * from biz_activity", vec![]).await?;
//!     let rows: Vec<BizActivity> = decode(value)?;
//!     println!("{:?} {}", rows, count);
//!     Ok(())
//! }
//! ```
//!
//! A single row is decoded both from `[{"k":1}]` and from the unwrapped `{"k":1}`,
//! so `select count(1)` can be decoded into a plain `u64` (see
//! [`try_decode_elements`]).

use rbs::Value;
use serde::de::DeserializeOwned;

use crate::Error;

/// Decode a query result into `T`.
///
/// `values` is the array of rows returned by a query (`[{"k": v}, ...]`), `T` can be
/// `Vec<Struct>`, `Struct`, `Option<Struct>` or a scalar type such as `String`,
/// `i64`, `bool` or a `rbdc` type like `DateTime` / `Decimal`.
pub fn decode_ref<T>(values: &Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    match values {
        Value::Array(_) => {
            // First try rbs direct decode. This handles [{k:v},...] -> Vec<T>.
            // The fallback only unwraps single-column rows for scalar values.
            match rbs::from_value_ref::<T>(values) {
                Ok(v) => Ok(v),
                Err(e) => match try_decode_single_column(values) {
                    Ok(v) => Ok(v),
                    Err(Some(fallback_err)) => Err(fallback_err),
                    Err(None) => Err(e.into()),
                },
            }
        }
        _ => Err(Error::from("decode error: expected array value")),
    }
}

/// Decode a query result into `T`, the owned counterpart of [`decode_ref`].
pub fn decode<T>(bs: Value) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    decode_ref(&bs)
}

/// Decode a single-column query result.
///
/// Unwraps single-element containers (`{"k": v}` -> `v`, `[v]` -> `v`) so that
/// scalars and `Option<T>` scalars can be decoded, without allowing a single row to
/// be decoded as a struct.
pub fn try_decode_elements<T: DeserializeOwned>(datas: &Value) -> Result<T, Error> {
    try_decode_single_column(datas).map_err(|e| {
        e.unwrap_or_else(|| Error::from("decode error: unsupported single row struct decode"))
    })
}

fn try_decode_single_column<T: DeserializeOwned>(datas: &Value) -> Result<T, Option<Error>> {
    let items = match datas {
        Value::Array(items) => items,
        _ => return rbs::from_value_ref::<T>(datas).map_err(|e| Some(e.into())),
    };

    let mut last_err = None;
    for item in items {
        let inner = match item {
            Value::Map(m) if m.len() == 1 => m.into_iter().next().map(|(_, v)| v),
            Value::Array(a) if a.len() == 1 => Some(&a[0]),
            _ => None,
        };
        if let Some(v) = inner {
            if let Ok(r) = rbs::from_value_ref::<T>(v) {
                return Ok(r);
            }
            last_err = rbs::from_value_ref::<T>(v).err().map(Error::from);
        }
    }

    Err(last_err)
}

/// Whether the crate was compiled in debug mode with the `debug_mode` feature enabled.
///
/// It is `cfg!(all(debug_assertions, feature = "debug_mode"))`, and it is what makes
/// rbatis print the JSON rows of every query and the decode errors while developing.
pub fn is_debug_mode() -> bool {
    cfg!(all(debug_assertions, feature = "debug_mode"))
}
