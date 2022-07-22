use crate::type_info::PgTypeInfo;
use rbs::Value;

pub enum IsNull {
    No,
    Yes,
}
pub trait Encode {
    fn encode(self) -> (IsNull, PgTypeInfo);
}

impl Encode for Value {
    fn encode(self) -> (IsNull, PgTypeInfo) {
        todo!()
    }
}
