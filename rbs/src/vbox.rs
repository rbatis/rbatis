use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use crate::Value;

pub struct VBox<T> {
    inner: Option<NonNull<T>>,
    _p: PhantomData<T>,
}

impl<T> VBox<T> {
    pub fn new(mut arg: T) -> Self {
        Self {
            inner: Some(NonNull::new(&mut arg).unwrap()),
            _p: PhantomData::default(),
        }
    }
    pub fn take(mut self) -> Option<T>
    where
        T: Default,
    {
        match self.inner {
            None => None,
            Some(v) => Some(std::mem::take(unsafe {
                self.inner.take().unwrap().as_mut()
            })),
        }
    }
}
unsafe impl<T: Sync> Sync for VBox<T> {}
unsafe impl<T: Send> Send for VBox<T> {}

impl<T> Deref for VBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().unwrap().as_ref() }
    }
}

impl<T> DerefMut for VBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut().unwrap().as_mut() }
    }
}

impl<T: Serialize> Serialize for VBox<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.deref().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for VBox<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let t = T::deserialize(deserializer)?;
        Ok(VBox::new(t))
    }
}

impl<T: Clone> Clone for VBox<T> {
    fn clone(&self) -> Self {
        VBox::new(self.deref().clone())
    }
}
impl<T: Debug> Debug for VBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}
impl<T: Display> Display for VBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}
impl<T: PartialEq> PartialEq for VBox<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<T: Copy> Copy for VBox<T> {}

impl <T>From<T> for VBox<T>{
    fn from(arg: T) -> Self {
        VBox::new(arg)
    }
}