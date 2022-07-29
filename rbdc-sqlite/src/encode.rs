use rbdc::Error;
use crate::SqliteArgumentValue;

pub trait Encode{
    fn encode(self,args: &mut Vec<SqliteArgumentValue<'_>>)->Result<IsNull,Error>;
}