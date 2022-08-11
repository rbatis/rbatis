use std::collections::{HashMap, VecDeque};

pub trait IntoOne<V> {
    fn into_one(self) -> Option<V>;
}

impl<V> IntoOne<V> for Option<V> {
    fn into_one(self) -> Option<V> {
        self
    }
}

impl<V> IntoOne<V> for Vec<V> {
    fn into_one(self) -> Option<V> {
        self.into_iter().next()
    }
}

impl<V> IntoOne<V> for VecDeque<V> {
    fn into_one(self) -> Option<V> {
        self.into_iter().next()
    }
}

impl<K, V> IntoOne<(K, V)> for HashMap<K, V> {
    fn into_one(self) -> Option<(K, V)> {
        self.into_iter().next()
    }
}

