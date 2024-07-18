use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};
use byteorder::{BigEndian, ByteOrder};
use rbdc::Error;

impl Decode for f64 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_f64(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for f32 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                let bytes = value.as_bytes()?;
                if bytes.len() == 8 {
                    BigEndian::read_f64(bytes) as f32
                } else if bytes.len() == 4 {
                    BigEndian::read_f32(bytes)
                } else {
                    return Err(Error::from("error f32 bytes len"));
                }
            }
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Encode for f64 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for f32 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}


#[cfg(test)]
mod test {
    use crate::type_info::PgTypeInfo;
    use crate::types::decode::Decode;
    use crate::value::{PgValue, PgValueFormat};

    #[test]
    fn test_decode_f32() {
        let bytes: [u8; 4] = 3_f32.to_be_bytes();
        let r: f32 = Decode::decode(PgValue {
            value: Some(bytes.to_vec()),
            type_info: PgTypeInfo::FLOAT4,
            format: PgValueFormat::Binary,
        }).unwrap();
        assert_eq!(r, 3.0);
    }

    #[test]
    fn test_decode_f32_by_f64() {
        let bytes: [u8; 8] = 3_f64.to_be_bytes();
        println!("bytes={:?}", bytes);
        let r: f32 = Decode::decode(PgValue {
            value: Some(bytes.to_vec()),
            type_info: PgTypeInfo::FLOAT4,
            format: PgValueFormat::Binary,
        }).unwrap();
        assert_eq!(r, 3.0);
    }
}