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
