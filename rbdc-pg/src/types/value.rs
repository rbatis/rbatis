use crate::arguments::PgArgumentBuffer;
use crate::type_info::PgTypeInfo;
use rbs::Value;

pub enum IsNull {
    No,
    Yes,
}
pub trait Encode {
    fn encode(self, arg: &mut PgArgumentBuffer) -> (IsNull, PgTypeInfo);
}

impl Encode for Value {
    fn encode(self, arg: &mut PgArgumentBuffer) -> (IsNull, PgTypeInfo) {
        match self {
            Value::Null => (IsNull::Yes, PgTypeInfo::with_name("unknown")),
            Value::Bool(v) => (IsNull::No, PgTypeInfo::with_name("bool")),
            Value::I32(v) => {
                todo!()
            }
            Value::I64(v) => {
                todo!()
            }
            Value::U32(v) => {
                todo!()
            }
            Value::U64(v) => {
                todo!()
            }
            Value::F32(v) => {
                todo!()
            }
            Value::F64(v) => {
                todo!()
            }
            Value::String(v) => {
                todo!()
            }
            Value::Binary(v) => {
                todo!()
            }
            Value::Array(v) => {
                todo!()
            }
            Value::Map(v) => {
                todo!()
            }
            Value::Ext(_, _) => {
                todo!()
            }
        }
    }
}
