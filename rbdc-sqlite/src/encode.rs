use rbdc::Error;
use rbs::Value;
use crate::{SqliteArguments, SqliteArgumentValue};

pub trait Encode{
    fn encode(self,args: &mut Vec<SqliteArgumentValue>)->Result<IsNull,Error>;
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

impl From<Vec<rbs::Value>> for SqliteArguments{
    fn from(arg: Vec<Value>) -> Self {
        todo!()
    }
}