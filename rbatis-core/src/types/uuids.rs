use std::any::type_name;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use rbson::{Binary, Bson};
use rbson::spec::BinarySubtype;
use serde::{Deserializer, Serialize, Serializer};
use serde::de::Error;

/// Uuid
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Uuid {
    pub inner: uuid::Uuid,
}

impl From<&rbson::Binary> for Uuid {
    fn from(arg: &rbson::Binary) -> Self {
        let id = &String::from_utf8(arg.bytes.clone()).unwrap_or_default();
        Uuid {
            inner: uuid::Uuid::from_str(&id).unwrap_or_default()
        }
    }
}


impl From<uuid::Uuid> for Uuid {
    fn from(arg: uuid::Uuid) -> Self {
        Self {
            inner: arg
        }
    }
}

impl From<&uuid::Uuid> for Uuid {
    fn from(arg: &uuid::Uuid) -> Self {
        Self {
            inner: arg.clone()
        }
    }
}

impl From<rbson::Binary> for Uuid {
    fn from(arg: rbson::Binary) -> Self {
        let id = &String::from_utf8(arg.bytes).unwrap_or_default();
        Uuid {
            inner: uuid::Uuid::from_str(&id).unwrap_or_default()
        }
    }
}

impl serde::Serialize for Uuid {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return serializer.serialize_str(&format!("Uuid({})", self.inner));
        }else {
            return self.inner.serialize(serializer);
        }
    }
}

impl<'de> serde::Deserialize<'de> for Uuid {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        match Bson::deserialize(deserializer)? {
            Bson::String(s) => {
                if s.starts_with("Uuid(") && s.ends_with(")") {
                    let inner_data = &s["Uuid(".len()..(s.len() - 1)];
                    return Ok(Self {
                        inner: uuid::Uuid::parse_str(inner_data).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                } else {
                    return Ok(Self {
                        inner: uuid::Uuid::parse_str(&s).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                }
            }
            _ => {
                Err(D::Error::custom("deserialize un supported bson type!"))
            }
        }
    }
}


impl Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Uuid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.to_string().fmt(f)
    }
}

impl std::fmt::Debug for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.to_string().fmt(f)
    }
}

impl Uuid {
    pub fn new() -> Uuid {
        let uuid = uuid::Uuid::new_v4();
        Uuid {
            inner: uuid
        }
    }

    pub fn parse_str(arg: &str) -> crate::error::Result<Self> {
        Ok(Uuid {
            inner: uuid::Uuid::parse_str(arg)?
        })
    }
}

#[cfg(test)]
mod test {
    use crate::types::Uuid;

    #[test]
    fn test_display() {
        println!("{}", Uuid::new());
    }

    #[test]
    fn test_debug() {
        let uuid = Uuid::new();
        println!("{:?}", uuid);
    }

    #[test]
    fn test_ser_de() {
        let b = Uuid::new();
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: Uuid = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }
}