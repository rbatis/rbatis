use crate::{SqliteArgumentValue, SqliteArguments};
use rbdc::Error;
use rbs::Value;

pub trait Encode {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error>;
}

/// The return type of [Encode::encode].
pub enum IsNull {
    /// The value is null; no data was written.
    Yes,

    /// The value is not null.
    ///
    /// This does not mean that data was written.
    No,
}

impl From<Vec<rbs::Value>> for SqliteArguments {
    fn from(args: Vec<Value>) -> Self {
        let mut arg = SqliteArguments {
            values: Vec::with_capacity(args.len()),
        };
        for x in args {
            arg.add(x).unwrap();
        }
        arg
    }
}

impl Encode for Value {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        match self {
            Value::Null => Ok(IsNull::Yes),
            Value::Bool(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::I32(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::I64(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::U32(v) => {
                (v as i32).encode(args)?;
                Ok(IsNull::No)
            }
            Value::U64(v) => {
                (v as i64).encode(args)?;
                Ok(IsNull::No)
            }
            Value::F32(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::F64(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::String(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::Binary(v) => {
                v.encode(args)?;
                Ok(IsNull::No)
            }
            Value::Array(_) => Ok(IsNull::Yes),
            Value::Map(_) => Ok(IsNull::Yes),
            Value::Ext(t, v) => match t {
                "Date" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "DateTime" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "Time" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "Timestamp" => {
                    (v.as_u64().unwrap_or_default() as i64).encode(args)?;
                    Ok(IsNull::No)
                }
                "Decimal" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "Json" => {
                    v.into_bytes().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                "Uuid" => {
                    v.into_string().unwrap_or_default().encode(args)?;
                    Ok(IsNull::No)
                }
                _ => Ok(IsNull::Yes),
            },
        }
    }
}
