use rbdc::Error;
use rbs::Value;
use crate::arguments::{PgArgumentBuffer, PgArguments};


pub enum IsNull {
    No,
    Yes,
}

pub trait Encode {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error>;
}

impl From<Vec<Value>> for PgArguments {
    fn from(args: Vec<Value>) -> Self {
        let mut arg = PgArguments {
            types: Vec::with_capacity(args.len()),
            buffer: PgArgumentBuffer::default(),
        };
        for x in args {
            arg.add(x).unwrap();
        }
        arg
    }
}

