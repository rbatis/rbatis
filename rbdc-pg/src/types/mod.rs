/// pg types see https://www.postgresql.org/docs/current/datatype.html
pub mod oid;
pub use oid::Oid;
pub mod array;
pub mod bigdecimal;
pub mod bool;
pub mod byte;
pub mod date;
pub mod datetime;
pub mod decimal;
pub mod decode;
pub mod encode;
pub mod float;
pub mod int;
pub mod json;
pub mod money;
pub mod numeric;
pub mod string;
pub mod time;
pub mod timestamp;
pub mod timestamptz;
pub mod timetz;
pub mod uuid;
pub mod value;
use crate::type_info::PgTypeInfo;

pub trait TypeInfo {
    fn type_info(&self) -> PgTypeInfo;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_datetime() {}
}
