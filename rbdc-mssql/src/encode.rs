use tiberius::ToSql;
use rbdc::Error;
use rbs::Value;

pub trait Encode{
    fn encode<'a>(&'a self) -> Result<&'a dyn ToSql, Error>;
}

impl Encode for Value{
    fn encode<'a>(&'a self) -> Result<&'a dyn ToSql, Error> {
        todo!()
    }
}