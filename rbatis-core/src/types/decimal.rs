use std::any::type_name;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use bigdecimal_::{BigDecimal, ParseBigDecimalError};
use rbson::Bson;
use rbson::spec::BinarySubtype;
use serde::{Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

/// Rbatis Decimal
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Decimal {
    pub inner: BigDecimal,
}

impl From<BigDecimal> for Decimal {
    fn from(arg: BigDecimal) -> Self {
        Self {
            inner: arg
        }
    }
}

impl From<&BigDecimal> for Decimal {
    fn from(arg: &BigDecimal) -> Self {
        Self {
            inner: arg.clone()
        }
    }
}

impl serde::Serialize for Decimal {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return serializer.serialize_str(&format!("Decimal({})", self.inner));
        }else{
            return self.inner.serialize(serializer);
        }
    }
}

/// Decimal allow deserialize by an String or Binary
impl<'de> serde::Deserialize<'de> for Decimal {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let bson = rbson::Bson::deserialize(deserializer)?;
        return match bson {
            Bson::String(s) => {
                if s.starts_with("Decimal(") && s.ends_with(")") {
                    let inner_data = &s["Decimal(".len()..(s.len() - 1)];
                    return Ok(Self {
                        inner: BigDecimal::from_str(inner_data).or_else(|e| Err(D::Error::custom(e.to_string())))?,
                    });
                } else {
                    Ok(Self {
                        inner: BigDecimal::from_str(s.as_str()).unwrap_or_default(),
                    })
                }
            }
            Bson::Int32(s) => {
                Ok(Self {
                    inner: BigDecimal::from(s),
                })
            }
            Bson::Int64(s) => {
                Ok(Self {
                    inner: BigDecimal::from(s),
                })
            }
            Bson::UInt64(s) => {
                Ok(Self {
                    inner: BigDecimal::from(s),
                })
            }
            Bson::Decimal128(s) => {
                Ok(Self {
                    inner: BigDecimal::from_str(&s.to_string()).unwrap_or_default(),
                })
            }
            _ => {
                Err(D::Error::custom("deserialize unsupported bson type!"))
            }
        };
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Deref for Decimal {
    type Target = BigDecimal;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Decimal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


impl Decimal {
    pub fn from(s: &str) -> Self {
        let b = BigDecimal::from_str(s).unwrap_or_default();
        Self {
            inner: b
        }
    }

    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        let b = BigDecimal::from_str(arg)?;
        Ok(Self {
            inner: b
        })
    }
}

#[cfg(test)]
mod test {
    use rbson::Bson;
    use crate::types::Decimal;

    #[test]
    fn test_ser_de() {
        let b = Decimal::from("1");
        let bsons = rbson::to_bson(&b).unwrap();
        match &bsons{
            rbson::Bson::String(s)=>{
                assert_eq!(s,"Decimal(1)");
            }
            _ => {
                panic!("not str");
            }
        }
        let b_de: Decimal = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }
}