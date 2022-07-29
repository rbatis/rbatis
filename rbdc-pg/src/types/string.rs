use rbdc::Error;
use crate::arguments::PgArgumentBuffer;
use crate::type_info::PgTypeInfo;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::types::TypeInfo;
use crate::value::PgValue;

impl Decode for String {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(value.as_str()?.to_owned())
    }
}

impl Encode for String {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        buf.extend(self.into_bytes());
        Ok(IsNull::No)
    }
}

impl TypeInfo for String{
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::VARCHAR
    }
}