use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;

/// rbatis bytes use serde_bytes
///
#[derive(Clone, Eq)]
pub struct Bytes {
    pub inner: Vec<u8>,
}

impl Bytes {
    /// Construct a new, empty `ByteBuf`.
    pub fn new() -> Self {
        Bytes { inner: Vec::new() }
    }

    /// Construct a new, empty `ByteBuf` with the specified capacity.
    pub fn with_capacity(cap: usize) -> Self {
        Bytes {
            inner: Vec::with_capacity(cap),
        }
    }

    /// Wrap existing bytes in a `ByteBuf`.
    pub fn from(bytes: Vec<u8>) -> Self {
        Self { inner: bytes }
    }

    /// Unwrap the vector of byte underlying this `ByteBuf`.
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }
}

impl Debug for Bytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl<Rhs> PartialEq<Rhs> for Bytes
where
    Rhs: ?Sized + AsRef<[u8]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<Rhs> PartialOrd<Rhs> for Bytes
where
    Rhs: ?Sized + AsRef<[u8]>,
{
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes { inner: value }
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Self {
            inner: value.to_vec(),
        }
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl AsMut<[u8]> for Bytes {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Self {
            inner: Vec::default(),
        }
    }
}

impl Hash for Bytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<'a> IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Bytes {
    type Item = &'a u8;
    type IntoIter = <&'a [u8] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut Bytes {
    type Item = &'a mut u8;
    type IntoIter = <&'a mut [u8] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let inner = serde_bytes::Bytes::new(&self.inner);
        inner.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = serde_bytes::ByteBuf::deserialize(deserializer)?;
        Ok(Self {
            inner: buf.into_vec(),
        })
    }
}
