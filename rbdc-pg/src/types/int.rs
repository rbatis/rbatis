use crate::arguments::PgArgumentBuffer;
use crate::type_info::PgTypeInfo;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::types::TypeInfo;
use crate::value::{PgValue, PgValueFormat};
use byteorder::{BigEndian, ByteOrder};
use rbdc::Error;

impl Decode for u64 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_u64(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for u32 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                let bytes = value.as_bytes()?;
                if bytes.len() == 8 {
                    BigEndian::read_u64(bytes) as u32
                } else if bytes.len() == 4 {
                    BigEndian::read_u32(bytes)
                } else {
                    return Err(Error::from("error u32 bytes len"));
                }
            },
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for u16 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                let bytes = value.as_bytes()?;
                if bytes.len() == 8 {
                    BigEndian::read_u64(bytes) as u16
                } else if bytes.len() == 4 {
                    BigEndian::read_u32(bytes) as u16
                } else if bytes.len() == 2 {
                    BigEndian::read_u16(bytes)
                } else {
                    return Err(Error::from("error u16 bytes len"));
                }
            },
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for u8 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        // note: in the TEXT encoding, a value of "0" here is encoded as an empty string
        Ok(value.as_bytes()?.get(0).copied().unwrap_or_default() as u8)
    }
}

impl Decode for i64 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => BigEndian::read_i64(value.as_bytes()?),
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for i32 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                let bytes = value.as_bytes()?;
                if bytes.len() == 8 {
                    BigEndian::read_i64(bytes) as i32
                } else if bytes.len() == 4 {
                    BigEndian::read_i32(bytes)
                }else {
                    return Err(Error::from("error i32 bytes len"));
                }
            },
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for i16 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                let bytes = value.as_bytes()?;
                if bytes.len() == 8 {
                    BigEndian::read_i64(bytes) as i16
                } else if bytes.len() == 4 {
                    BigEndian::read_i32(bytes) as i16
                } else if bytes.len() == 2 {
                    BigEndian::read_i16(bytes)
                } else {
                    return Err(Error::from("error i16 bytes len"));
                }
            },
            PgValueFormat::Text => value.as_str()?.parse()?,
        })
    }
}

impl Decode for i8 {
    fn decode(value: PgValue) -> Result<Self, Error> {
        // note: in the TEXT encoding, a value of "0" here is encoded as an empty string
        Ok(value.as_bytes()?.get(0).copied().unwrap_or_default() as i8)
    }
}

///encode

impl Encode for u64 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());
        Ok(IsNull::No)
    }
}

impl Encode for u32 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for u16 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for u8 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for i64 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for i32 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for i16 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

impl Encode for i8 {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(&self.to_be_bytes());

        Ok(IsNull::No)
    }
}

///TypeInfo

impl TypeInfo for i8 {
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::BYTEA
    }
}



#[cfg(test)]
mod test {
    use crate::type_info::PgTypeInfo;
    use crate::types::decode::Decode;
    use crate::value::{PgValue, PgValueFormat};

    #[test]
    fn test_decode_u32() {
        let bytes: [u8; 4] = 3_u32.to_be_bytes();
        let r: u32 = Decode::decode(PgValue {
            value: Some(bytes.to_vec()),
            type_info: PgTypeInfo::INT8,
            format: PgValueFormat::Binary,
        }).unwrap();
        assert_eq!(r, 3);
    }

    #[test]
    fn test_decode_u32_by_u64() {
        let bytes: [u8; 8] = 3_u64.to_be_bytes();
        println!("bytes={:?}", bytes);
        let r: u32 = Decode::decode(PgValue {
            value: Some(bytes.to_vec()),
            type_info: PgTypeInfo::INT8,
            format: PgValueFormat::Binary,
        }).unwrap();
        assert_eq!(r, 3);
    }

    #[test]
    fn test_decode_u16_by_u64() {
        let bytes: [u8; 8] = 3_u64.to_be_bytes();
        println!("bytes={:?}", bytes);
        let r: u16 = Decode::decode(PgValue {
            value: Some(bytes.to_vec()),
            type_info: PgTypeInfo::INT8,
            format: PgValueFormat::Binary,
        }).unwrap();
        assert_eq!(r, 3);
    }


    #[test]
    fn test_decode_i32() {
        let bytes: [u8; 4] = 3_i32.to_be_bytes();
        let r: i32 = Decode::decode(PgValue {
            value: Some(bytes.to_vec()),
            type_info: PgTypeInfo::INT8,
            format: PgValueFormat::Binary,
        }).unwrap();
        assert_eq!(r, 3);
    }

    #[test]
    fn test_decode_i32_by_i64() {
        let bytes: [u8; 8] = 3_i64.to_be_bytes();
        println!("bytes={:?}", bytes);
        let r: i32 = Decode::decode(PgValue {
            value: Some(bytes.to_vec()),
            type_info: PgTypeInfo::INT8,
            format: PgValueFormat::Binary,
        }).unwrap();
        assert_eq!(r, 3);
    }

    #[test]
    fn test_decode_i16_by_i64() {
        let bytes: [u8; 8] = 3_i64.to_be_bytes();
        println!("bytes={:?}", bytes);
        let r: i16 = Decode::decode(PgValue {
            value: Some(bytes.to_vec()),
            type_info: PgTypeInfo::INT8,
            format: PgValueFormat::Binary,
        }).unwrap();
        assert_eq!(r, 3);
    }
}