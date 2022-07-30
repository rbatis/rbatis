use tiberius::ToSql;
use rbdc::Error;
use rbs::Value;

pub trait Encode {
    fn encode<'a>(&'a self) -> Result<&'a dyn ToSql, Error>;
}

impl Encode for Value {
    fn encode<'a>(&'a self) -> Result<&'a dyn ToSql, Error> {
        match self {
            Value::Null => { todo!() }
            Value::Bool(v) => { Ok(v) }
            Value::I32(v) => { Ok(v) }
            Value::I64(v) => { Ok(v) }
            Value::U32(v) => { unimplemented!() }
            Value::U64(v) => { unimplemented!() }
            Value::F32(v) => { Ok(v) }
            Value::F64(v) => { Ok(v) }
            Value::String(v) => { Ok(v) }
            Value::Binary(v) => { Ok(v) }
            Value::Array(_) => { todo!() }
            Value::Map(_) => { todo!() }
            Value::Ext(t, v) => {
                match *t {
                    "Date" => { todo!() }
                    "DateTime" => { todo!() }
                    "Time" => { todo!() }
                    "Decimal" => { todo!() }
                    "Json" => { todo!() }
                    "Timestamp" => { todo!() }
                    "Uuid" => { todo!() }
                    _ => {
                        unimplemented!()
                    }
                }
            }
        }
    }
}