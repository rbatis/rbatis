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

impl SqliteArguments{
    pub fn from_args(args: Vec<Value>) -> Result<Self,Error>{
        let mut arg = SqliteArguments {
            values: Vec::with_capacity(args.len()),
        };
        for x in args {
            arg.add(x)?;
        }
        Ok(arg)
    }
}
