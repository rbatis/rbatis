use std::io::Write;

use rmp::encode::{write_array_len, write_bin, write_bool, write_ext_meta, write_f32, write_f64, write_i32, write_map_len, write_nil, write_sint, write_u32, write_uint};

use super::Error;
use crate::value::{IntPriv, Integer, Value};

/// Encodes and attempts to write the most efficient representation of the given Value.
///
/// # Note
///
/// All instances of `ErrorKind::Interrupted` are handled by this function and the underlying
/// operation is retried.
pub fn write_value<W>(wr: &mut W, val: &Value) -> Result<(), Error>
    where W: Write
{
    match *val {
        Value::Nil => {
            write_nil(wr).map_err(Error::InvalidMarkerWrite)?;
        }
        Value::Bool(val) => {
            write_bool(wr, val).map_err(Error::InvalidMarkerWrite)?;
        }
        Value::I32(n) =>{
            write_i32(wr, n)?;
        }
        Value::I64(n) =>{
            write_sint(wr, n)?;
        }
        Value::U32(n) =>{
            write_u32(wr, n)?;
        }
        Value::U64(n) =>{
            write_uint(wr, n)?;
        }
        Value::F32(val) => {
            write_f32(wr, val)?;
        }
        Value::F64(val) => {
            write_f64(wr, val)?;
        }
        Value::String(ref s) => {
            // match *s {
            //     Ok(ref val) => write_str(wr, &val)?,
            //     Err(ref err) => write_bin(wr, &err.0)?,
            // }
            write_bin(wr, s.as_bytes())?;
        }
        Value::Binary(ref val) => {
            write_bin(wr, &val)?;
        }
        Value::Array(ref vec) => {
            write_array_len(wr, vec.len() as u32)?;
            for v in vec {
                write_value(wr, v)?;
            }
        }
        Value::Map(ref map) => {
            write_map_len(wr, map.len() as u32)?;
            for &(ref key, ref val) in map {
                write_value(wr, key)?;
                write_value(wr, val)?;
            }
        }
        Value::Ext(ty, ref data) => {
            write_ext_meta(wr, data.len() as u32, ty)?;
            wr.write_all(data).map_err(Error::InvalidDataWrite)?;
        }
    }

    Ok(())
}
