use crate::types::BINARY_SUBTYPE_JSON;
use rbson::spec::BinarySubtype;
use rbson::{Binary, Bson, Document};
use serde::de::{DeserializeOwned, Error, Expected};
use serde::{Deserializer, Serialize, Serializer};
use serde_json::ser::Formatter;
use serde_json::Value;
use std::any::type_name;
use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::slice::IterMut;

/// Json
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Json<T>
where
    T: Serialize,
{
    pub inner: T,
}

impl<T> From<T> for Json<T>
where
    T: Serialize,
{
    fn from(arg: T) -> Self {
        Self { inner: arg }
    }
}

impl<T> From<&T> for Json<T>
where
    T: Serialize + Clone,
{
    fn from(arg: &T) -> Self {
        Self { inner: arg.clone() }
    }
}

impl<T: Serialize> serde::Serialize for Json<T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        if type_name::<S::Error>().eq("rbson::ser::error::Error") {
            return Binary {
                subtype: BinarySubtype::UserDefined(BINARY_SUBTYPE_JSON),
                bytes: serde_json::to_vec(&self.inner).unwrap_or_default(),
            }
            .serialize(serializer);
        } else {
            return self.inner.serialize(serializer);
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for Json<T>
where
    T: Serialize + DeserializeOwned,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b = Bson::deserialize(deserializer)?;
        match b {
            Bson::String(s) => {
                return Ok(Self {
                    inner: serde_json::from_str(&s)
                        .or_else(|e| Err(D::Error::custom(e.to_string())))?,
                });
            }
            Bson::Binary(data) => {
                let v = serde_json::from_slice::<T>(&data.bytes)
                    .or_else(|e| Err(D::Error::custom(e.to_string())))?;
                Ok(Json { inner: v })
            }
            Bson::Decimal128(v) => {
                let v = serde_json::from_value::<T>(serde_json::Value::String(v.to_string()))
                    .or_else(|e| Err(D::Error::custom(e.to_string())))?;
                Ok(Json { inner: v })
            }
            _ => {
                let v = serde_json::from_value::<T>(b.into_canonical_extjson())
                    .or_else(|e| Err(D::Error::custom(e.to_string())))?;
                Ok(Json { inner: v })
            }
        }
    }
}

impl<T: Serialize> std::fmt::Display for Json<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Serialize> std::fmt::Debug for Json<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Serialize> Json<T> {
    pub fn to_string(&self) -> String
    where
        T: std::fmt::Display,
    {
        self.inner.to_string()
    }
}

impl<T: Serialize> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Serialize> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Json<T>
where
    T: Serialize,
{
    pub fn from_value(arg: Value) -> Self
    where
        T: Serialize + DeserializeOwned + Default,
    {
        Json {
            inner: serde_json::from_value(arg).unwrap_or_default(),
        }
    }
}

impl<T> Json<T>
where
    T: Serialize + DeserializeOwned,
{
    /// create from str
    pub fn from_str(arg: &str) -> Result<Self, crate::error::Error> {
        let inner = serde_json::from_str(arg)?;
        Ok(Self { inner: inner })
    }
}

#[cfg(test)]
mod test {
    use crate::types::Json;

    #[test]
    fn test_ser_de() {
        let b = Json { inner: 1 };
        let bsons = rbson::to_bson(&b).unwrap();
        let b_de: Json<i32> = rbson::from_bson(bsons).unwrap();
        assert_eq!(b, b_de);
    }
}
