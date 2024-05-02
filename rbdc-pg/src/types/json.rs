use crate::arguments::PgArgumentBuffer;
use crate::type_info::PgTypeInfo;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::types::TypeInfo;
use crate::value::{PgValue, PgValueFormat};
use rbdc::json::Json;
use rbdc::Error;
use rbs::Value;
use std::io::Write;

impl Encode for Json {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let mut bytes = self.0.into_bytes();
        if bytes.is_empty() {
            bytes = "null".to_string().into_bytes();
        }
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
        buf.write_all(&bytes)?;

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

pub fn decode_json(value: PgValue) -> Result<Value, Error> {
    let fmt = value.format();
    let type_info = value.type_info;
    let mut buf = value.value.unwrap_or_default();
    if buf.len() == 0 {
        return Ok(Value::Null);
    }
    if fmt == PgValueFormat::Binary && type_info == PgTypeInfo::JSONB {
        assert_eq!(
            buf[0], 1,
            "unsupported JSONB format version {}; please open an issue",
            buf[0]
        );
        buf.remove(0);
    }
    Ok(
        serde_json::from_str(&unsafe { String::from_utf8_unchecked(buf) })
            .map_err(|e| Error::from(e.to_string()))?,
    )
}

pub fn encode_json(v: Value, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
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
    buf.write_all(&v.to_string().into_bytes())?;

    Ok(IsNull::No)
}

impl TypeInfo for Json {
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::JSONB
    }
}
