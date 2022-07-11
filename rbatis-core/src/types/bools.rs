use rbson::Bson;
use serde::de::Error;
use serde::{Deserializer, Serializer};
use std::ops::{Deref, DerefMut};

/// Rbatis Bool
/// for example:
/// let b = Bool::from(true);
/// let b = Bool::from("true");
/// let b = Bool::from(1);
///
///
/// 1,1.0,"1",true              will be Bool{ inner: true }
/// -1,0,"2",false...and..more  will be Bool{ inner: false }
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bool {
    pub inner: bool,
}

impl From<bool> for Bool {
    fn from(arg: bool) -> Self {
        Bool { inner: arg }
    }
}

impl From<i32> for Bool {
    fn from(arg: i32) -> Self {
        Bool {
            inner: {
                if arg == 1 {
                    true
                } else {
                    false
                }
            },
        }
    }
}

impl From<i64> for Bool {
    fn from(arg: i64) -> Self {
        Bool {
            inner: {
                if arg == 1 {
                    true
                } else {
                    false
                }
            },
        }
    }
}

impl From<u32> for Bool {
    fn from(arg: u32) -> Self {
        Bool {
            inner: {
                if arg == 1 {
                    true
                } else {
                    false
                }
            },
        }
    }
}

impl From<u64> for Bool {
    fn from(arg: u64) -> Self {
        Bool {
            inner: {
                if arg == 1 {
                    true
                } else {
                    false
                }
            },
        }
    }
}

impl From<f64> for Bool {
    fn from(arg: f64) -> Self {
        Bool {
            inner: {
                if arg == 1f64 {
                    true
                } else {
                    false
                }
            },
        }
    }
}

impl From<&str> for Bool {
    fn from(arg: &str) -> Self {
        Bool {
            inner: {
                if arg == "true" || arg == "1" || arg == "1.0" {
                    true
                } else {
                    false
                }
            },
        }
    }
}

impl serde::Serialize for Bool {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.inner)
    }
}

impl<'de> serde::Deserialize<'de> for Bool {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bson = Bson::deserialize(deserializer)?;
        match bson {
            Bson::Boolean(data) => {
                return Ok(Bool { inner: data });
            }
            Bson::Int32(data) => {
                return Ok(Bool::from(data));
            }
            Bson::Int64(data) => {
                return Ok(Bool::from(data));
            }
            Bson::Double(data) => {
                return Ok(Bool::from(data));
            }
            Bson::UInt32(data) => {
                return Ok(Bool::from(data));
            }
            Bson::UInt64(data) => {
                return Ok(Bool::from(data));
            }
            Bson::String(data) => {
                return Ok(Bool::from(data.as_str()));
            }
            _ => Err(D::Error::custom("deserialize unsupported bson type!")),
        }
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Bool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod test {
    use crate::Bool;

    #[test]
    fn test_ser_de() {
        let b = Bool::from("1");
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: Bool = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
        assert_eq!(b.inner, b_de.inner);
    }
}
