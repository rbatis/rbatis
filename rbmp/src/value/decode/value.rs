use std::cmp::min;
use std::io::{self, Read};

use rmp::decode::{
    read_data_f32, read_data_f64, read_data_i16, read_data_i32, read_data_i64, read_data_i8,
    read_data_u16, read_data_u32, read_data_u64, read_data_u8, read_marker,
};
use rmp::Marker;

use super::Error;
use crate::{Value};

// See https://github.com/3Hren/msgpack-rust/issues/151
const PREALLOC_MAX: usize = 64 * 1024; // 64 KiB

fn read_array_data<R: Read>(rd: &mut R, mut len: usize, depth: usize) -> Result<Vec<Value>, Error> {
    let depth = super::decrement_depth(depth)?;

    // Note: Do not preallocate a Vec of size `len`.
    // See https://github.com/3Hren/msgpack-rust/issues/151
    let mut vec = Vec::new();

    while len > 0 {
        vec.push(read_value_inner(rd, depth)?);
        len -= 1;
    }

    Ok(vec)
}

fn read_map_data<R: Read>(rd: &mut R, mut len: usize, depth: usize) -> Result<Vec<(Value, Value)>, Error> {
    let depth = super::decrement_depth(depth)?;

    // Note: Do not preallocate a Vec of size `len`.
    // See https://github.com/3Hren/msgpack-rust/issues/151
    let mut vec = Vec::new();

    while len > 0 {
        vec.push((read_value_inner(rd, depth)?, read_value_inner(rd, depth)?));
        len -= 1;
    }

    Ok(vec)
}

fn read_str_data<R: Read>(rd: &mut R, len: usize, depth: usize) -> Result<Vec<u8>, Error> {
    let depth = super::decrement_depth(depth)?;

    // match String::from_utf8(read_bin_data(rd, len, depth)?) {
    //     Ok(s) => Ok(Utf8String::from(s)),
    //     Err(err) => {
    //         let e = err.utf8_error();
    //         let s = Utf8String {
    //             s: Err((err.into_bytes(), e)),
    //         };
    //         Ok(s)
    //     }
    // }
    Ok(read_bin_data(rd, len, depth)?)
}

fn read_bin_data<R: Read>(rd: &mut R, len: usize, depth: usize) -> Result<Vec<u8>, Error> {
    let _depth = super::decrement_depth(depth)?;

    let mut buf = Vec::with_capacity(min(len, PREALLOC_MAX));
    let bytes_read = rd.take(len as u64).read_to_end(&mut buf).map_err(Error::InvalidDataRead)?;
    if bytes_read != len {
        return Err(Error::InvalidDataRead(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            format!("Expected {} bytes, read {} bytes", len, bytes_read),
        )));
    }

    Ok(buf)
}

fn read_ext_body<R: Read>(rd: &mut R, len: usize, depth: usize) -> Result<(i8, Vec<u8>), Error> {
    let depth = super::decrement_depth(depth)?;

    let ty = read_data_i8(rd)?;
    let vec = read_bin_data(rd, len, depth)?;

    Ok((ty, vec))
}

fn read_value_inner<R>(rd: &mut R, depth: usize) -> Result<Value, Error> where R: Read {
    let depth = super::decrement_depth(depth)?;
    let val = match read_marker(rd)? {
        Marker::Null => Value::Nil,
        Marker::True => Value::Bool(true),
        Marker::False => Value::Bool(false),
        Marker::FixPos(val) => Value::from(val),
        Marker::FixNeg(val) => Value::from(val),
        Marker::U8 => Value::from(read_data_u8(rd)?),
        Marker::U16 => Value::from(read_data_u16(rd)?),
        Marker::U32 => Value::from(read_data_u32(rd)?),
        Marker::U64 => Value::from(read_data_u64(rd)?),
        Marker::I8 => Value::from(read_data_i8(rd)?),
        Marker::I16 => Value::from(read_data_i16(rd)?),
        Marker::I32 => Value::from(read_data_i32(rd)?),
        Marker::I64 => Value::from(read_data_i64(rd)?),
        Marker::F32 => Value::F32(read_data_f32(rd)?),
        Marker::F64 => Value::F64(read_data_f64(rd)?),
        Marker::FixStr(len) => {
            let res = read_str_data(rd, len as usize, depth)?;
            Value::String(unsafe{String::from_utf8_unchecked(res)})
        }
        Marker::Str8 => {
            let len = read_data_u8(rd)?;
            let res = read_str_data(rd, len as usize, depth)?;
            Value::String(unsafe{String::from_utf8_unchecked(res)})
        }
        Marker::Str16 => {
            let len = read_data_u16(rd)?;
            let res = read_str_data(rd, len as usize, depth)?;
            Value::String(unsafe{String::from_utf8_unchecked(res)})
        }
        Marker::Str32 => {
            let len = read_data_u32(rd)?;
            let res = read_str_data(rd, len as usize, depth)?;
            Value::String(unsafe{String::from_utf8_unchecked(res)})
        }
        Marker::FixArray(len) => {
            let vec = read_array_data(rd, len as usize, depth)?;
            Value::Array(vec)
        }
        Marker::Array16 => {
            let len = read_data_u16(rd)?;
            let vec = read_array_data(rd, len as usize, depth)?;
            Value::Array(vec)
        }
        Marker::Array32 => {
            let len = read_data_u32(rd)?;
            let vec = read_array_data(rd, len as usize, depth)?;
            Value::Array(vec)
        }
        Marker::FixMap(len) => {
            let map = read_map_data(rd, len as usize, depth)?;
            Value::Map(map)
        }
        Marker::Map16 => {
            let len = read_data_u16(rd)?;
            let map = read_map_data(rd, len as usize, depth)?;
            Value::Map(map)
        }
        Marker::Map32 => {
            let len = read_data_u32(rd)?;
            let map = read_map_data(rd, len as usize, depth)?;
            Value::Map(map)
        }
        Marker::Bin8 => {
            let len = read_data_u8(rd)?;
            let vec = read_bin_data(rd, len as usize, depth)?;
            Value::Binary(vec)
        }
        Marker::Bin16 => {
            let len = read_data_u16(rd)?;
            let vec = read_bin_data(rd, len as usize, depth)?;
            Value::Binary(vec)
        }
        Marker::Bin32 => {
            let len = read_data_u32(rd)?;
            let vec = read_bin_data(rd, len as usize, depth)?;
            Value::Binary(vec)
        }
        Marker::FixExt1 => {
            let len = 1 as usize;
            let (ty, vec) = read_ext_body(rd, len, depth)?;
            Value::Ext(ty, vec)
        }
        Marker::FixExt2 => {
            let len = 2 as usize;
            let (ty, vec) = read_ext_body(rd, len, depth)?;
            Value::Ext(ty, vec)
        }
        Marker::FixExt4 => {
            let len = 4 as usize;
            let (ty, vec) = read_ext_body(rd, len, depth)?;
            Value::Ext(ty, vec)
        }
        Marker::FixExt8 => {
            let len = 8 as usize;
            let (ty, vec) = read_ext_body(rd, len, depth)?;
            Value::Ext(ty, vec)
        }
        Marker::FixExt16 => {
            let len = 16 as usize;
            let (ty, vec) = read_ext_body(rd, len, depth)?;
            Value::Ext(ty, vec)
        }
        Marker::Ext8 => {
            let len = read_data_u8(rd)? as usize;
            let (ty, vec) = read_ext_body(rd, len, depth)?;
            Value::Ext(ty, vec)
        }
        Marker::Ext16 => {
            let len = read_data_u16(rd)? as usize;
            let (ty, vec) = read_ext_body(rd, len, depth)?;
            Value::Ext(ty, vec)
        }
        Marker::Ext32 => {
            let len = read_data_u32(rd)? as usize;
            let (ty, vec) = read_ext_body(rd, len, depth)?;
            Value::Ext(ty, vec)
        }
        Marker::Reserved => Value::Nil,
    };

    Ok(val)
}

/// Attempts to read bytes from the given reader and interpret them as a [`Value`].
///
/// # Errors
///
/// This function will return [`Error`] on any I/O error while either reading or decoding a [`Value`].
/// All instances of [`ErrorKind::Interrupted`](io::ErrorKind) are handled by this function and the
/// underlying operation is retried.
///
/// [`Error::DepthLimitExceeded`] is returned if this function recurses
/// [`MAX_DEPTH`](super::MAX_DEPTH) times. To configure the maximum recursion depth, use
/// [`read_value_with_max_depth`] instead.
#[inline(never)]
pub fn read_value<R>(rd: &mut R) -> Result<Value, Error>
    where R: Read
{
    read_value_inner(rd, super::MAX_DEPTH)
}

/// Attempts to read bytes from the given reader and interpret them as a [`Value`].
///
/// # Errors
///
/// This function will return [`Error`] on any I/O error while either reading or decoding a [`Value`].
/// All instances of [`ErrorKind::Interrupted`](io::ErrorKind) are handled by this function and the
/// underlying operation is retried.
///
/// [`Error::DepthLimitExceeded`] is returned if this function recurses
/// `max_depth` times. If the default [`MAX_DEPTH`](super::MAX_DEPTH) is sufficient or you do not
/// need recursion depth checking for your data, consider using [`read_value`] instead.
#[inline(never)]
pub fn read_value_with_max_depth<R>(rd: &mut R, max_depth: usize) -> Result<Value, Error>
    where R: Read
{
    read_value_inner(rd, max_depth)
}
