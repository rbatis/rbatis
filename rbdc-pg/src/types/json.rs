use crate::arguments::PgArgumentBuffer;
use crate::type_info::PgTypeInfo;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use rbdc::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io::Write;
use rbdc::json::Json;
use crate::types::TypeInfo;

impl Encode for Json {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        // we have a tiny amount of dynamic behavior depending if we are resolved to be JSON
        // instead of JSONB
        buf.patch(|buf, ty: &PgTypeInfo| {
            if *ty == PgTypeInfo::JSON || *ty == PgTypeInfo::JSON_ARRAY {
                buf[0] = b' ';
            }
        });

        // JSONB version (as of 2020-03-20)
        buf.push(1);

        // the JSON data written to the buffer is the same regardless of parameter type
        buf.write(&self.0.into_bytes())?;

        Ok(IsNull::No)
    }
}

impl Decode for Json {
    fn decode(value: PgValue) -> Result<Self, Error> {
        let fmt = value.format();
        let type_info = value.type_info;
        let mut buf = value.value.unwrap_or_default();
        if buf.len() == 0 {
            return Ok(Json {
                0: "null".to_string(),
            });
        }
        if fmt == PgValueFormat::Binary && type_info == PgTypeInfo::JSONB {
            assert_eq!(
                buf[0], 1,
                "unsupported JSONB format version {}; please open an issue",
                buf[0]
            );
            buf.remove(0);
        }
        Ok(Self {
            0: unsafe { String::from_utf8_unchecked(buf) },
        })
    }
}

impl TypeInfo for Json{
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::JSONB
    }
}