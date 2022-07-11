use rbson::Bson;
use serde::de::Error;
use serde::{Deserializer, Serializer};
use std::ops::{Deref, DerefMut};

/// Rbatis Bytes
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bytes {
    pub inner: Vec<u8>,
}

impl From<rbson::Binary> for Bytes {
    fn from(arg: rbson::Binary) -> Self {
        Self { inner: arg.bytes }
    }
}

impl From<&rbson::Binary> for Bytes {
    fn from(arg: &rbson::Binary) -> Self {
        Self {
            inner: arg.bytes.clone(),
        }
    }
}

impl From<&[u8]> for Bytes {
    fn from(arg: &[u8]) -> Self {
        Self {
            inner: arg.to_owned(),
        }
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(arg: Vec<u8>) -> Self {
        Self { inner: arg }
    }
}

impl From<&Vec<u8>> for Bytes {
    fn from(arg: &Vec<u8>) -> Self {
        Self {
            inner: arg.to_owned(),
        }
    }
}

impl serde::Serialize for Bytes {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.inner)
    }
}

impl<'de> serde::Deserialize<'de> for Bytes {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bson = Bson::deserialize(deserializer)?;
        match bson {
            Bson::Binary(data) => {
                return Ok(Bytes { inner: data.bytes });
            }
            Bson::String(data) => {
                return match base64::decode(data) {
                    Ok(v) => Ok(Bytes { inner: v }),
                    Err(e) => {
                        return Err(D::Error::custom(e.to_string()));
                    }
                };
            }
            _ => Err(D::Error::custom("deserialize unsupported bson type!")),
        }
    }
}

impl Bytes {
    pub fn new(arg: Vec<u8>) -> Bytes {
        Bytes { inner: arg }
    }
}

impl Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod test {
    use crate::types::Bytes;
    use rbson::Bson;

    #[test]
    fn test_ser_de() {
        let b = Bytes::from("111".as_bytes());
        let bsons = rbson::to_bson(&b).unwrap();
        match &bsons {
            rbson::Bson::Binary(b) => {
                assert_eq!(b.subtype, rbson::spec::BinarySubtype::Generic);
                println!("yes is BinarySubtype::Generic");
            }
            _ => {}
        }
        let b_de: Bytes = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
        assert_eq!(b.inner, b_de.inner);
    }
}
