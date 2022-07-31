use std::borrow::{Cow, Borrow};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, Index};

use serde::{Deserializer, Serializer};
use std::cmp::Ordering::Less;
use rbson::{Document, Timestamp};

/// convert Value to Value
pub trait AsProxy {
    fn i32(&self) -> i32;
    fn i64(&self) -> i64;
    fn u32(&self) -> u32;
    fn u64(&self) -> u64;
    fn f64(&self) -> f64;
    fn str(&self) -> &str;
    fn string(&self) -> String;
    fn bool(&self) -> bool;
    fn array(&self) -> Option<&rbson::Array>;
    fn object(&self) -> Option<&Document>;

    //is
    fn is_empty(&self) -> bool;
    fn is_null(&self) -> bool;
    fn is_array(&self) -> bool;
    fn is_document(&self) -> bool;
    fn is_object(&self) -> bool;// is_document = is_object

    //try to any string
    fn cast_string(&self) -> String;
    fn cast_i64(&self) -> i64;
    fn cast_f64(&self) -> f64;
    fn cast_u64(&self) -> u64;
    /// bracket(xxxx) inner data
    fn bracket(&self) -> &str;
    /// bracket(xxxx) inner data
    fn inner(&self) -> &str{
       self.bracket()
    }
}

/// proxy rbson::Document struct,support Deserializer, Serializer
/// use Cow Optimize unnecessary clones
/// This structure has a certain amount of computing power
pub type Value = rbson::Bson;


pub fn as_timestamp(arg: &Timestamp) -> i64 {
    let upper = (arg.time.to_le() as u64) << 32;
    let lower = arg.increment.to_le() as u64;
    (upper | lower) as i64
}

impl AsProxy for Value {
    fn i32(&self) -> i32 {
        return match self {
            Value::Double(v) => { *v as i32 }
            Value::UInt32(v) => { *v as i32 }
            Value::UInt64(v) => { *v as i32 }
            Value::Int32(v) => { *v }
            Value::Int64(v) => { *v as i32 }
            _ => { 0 }
        };
    }

    fn i64(&self) -> i64 {
        return match self {
            Value::Double(v) => { *v as i64 }
            Value::UInt32(v) => { *v as i64 }
            Value::UInt64(v) => { *v as i64 }
            Value::Int32(v) => { *v as i64 }
            Value::Int64(v) => { *v }
            _ => { 0 }
        };
    }

    fn u32(&self) -> u32 {
        return match self {
            Value::Double(v) => { *v as u32 }
            Value::Int32(v) => { *v as u32 }
            Value::Int64(v) => { *v as u32 }
            Value::UInt32(v) => { *v }
            Value::UInt64(v) => { *v as u32 }
            _ => { 0 }
        };
    }

    fn u64(&self) -> u64 {
        return match self {
            Value::Double(v) => { *v as u64 }
            Value::Int32(v) => { *v as u64 }
            Value::Int64(v) => { *v as u64 }
            Value::UInt32(v) => { *v as u64 }
            Value::UInt64(v) => { *v }
            _ => { 0 }
        };
    }

    fn f64(&self) -> f64 {
        return match self {
            Value::Double(v) => { *v }
            Value::Int32(v) => { *v as f64 }
            Value::Int64(v) => { *v as f64 }
            Value::UInt32(v) => { *v as f64 }
            Value::UInt64(v) => { *v as f64 }
            _ => { 0.0 }
        };
    }

    fn str(&self) -> &str {
        self.as_str().unwrap_or_default()
    }

    fn string(&self) -> String {
        self.str().to_string()
    }

    fn cast_string(&self) -> String {
        match self {
            Value::Binary(b) => { String::from_utf8(b.bytes.clone()).unwrap_or_default() }
            Value::Double(d) => { d.to_string() }
            Value::String(d) => { d.to_string() }
            Value::Boolean(d) => { d.to_string() }
            Value::Null => { "".to_string() }
            Value::Int32(i) => { i.to_string() }
            Value::Int64(d) => { d.to_string() }
            Value::Timestamp(d) => { as_timestamp(d).to_string() }
            Value::DateTime(d) => { d.to_string() }
            Value::Decimal128(d) => { d.to_string() }
            _ => {
                String::new()
            }
        }
    }

    fn cast_i64(&self) -> i64 {
        match self {
            Value::Binary(b) => {
                String::from_utf8(b.bytes.clone()).unwrap_or_default()
                    .parse().unwrap_or_default()
            }
            Value::Double(d) => {
                *d as i64
            }
            Value::String(d) => { d.to_string().parse().unwrap_or_default() }
            Value::Boolean(d) => {
                if *d == true {
                    return 1;
                } else {
                    return 0;
                }
            }
            Value::Null => { 0 }
            Value::UInt32(i) => { *i as i64 }
            Value::UInt64(d) => {
                *d as i64
            }
            Value::Int32(i) => { *i as i64 }
            Value::Int64(d) => { *d }
            Value::Timestamp(d) => {
                as_timestamp(d)
            }
            Value::DateTime(d) => {
                d.timestamp_millis()
            }
            Value::Decimal128(d) => { d.to_string().parse().unwrap_or_default() }
            _ => {
                0
            }
        }
    }

    fn cast_u64(&self) -> u64 {
        match self {
            Value::Binary(b) => {
                String::from_utf8(b.bytes.clone()).unwrap_or_default()
                    .parse().unwrap_or_default()
            }
            Value::Double(d) => {
                *d as u64
            }
            Value::String(d) => { d.to_string().parse().unwrap_or_default() }
            Value::Boolean(d) => {
                if *d == true {
                    return 1;
                } else {
                    return 0;
                }
            }
            Value::Null => { 0 }
            Value::Int32(i) => { *i as u64 }
            Value::Int64(d) => { *d as u64 }
            Value::UInt32(i) => { *i as u64 }
            Value::UInt64(d) => { *d }
            Value::Timestamp(d) => {
                as_timestamp(d) as u64
            }
            Value::DateTime(d) => {
                d.timestamp_millis() as u64
            }
            Value::Decimal128(d) => { d.to_string().parse().unwrap_or_default() }
            _ => {
                0
            }
        }
    }

    fn cast_f64(&self) -> f64 {
        match self {
            Value::Binary(b) => {
                String::from_utf8(b.bytes.clone()).unwrap_or_default()
                    .parse().unwrap_or_default()
            }
            Value::Double(d) => {
                *d as f64
            }
            Value::String(d) => { d.to_string().parse().unwrap_or_default() }
            Value::Boolean(d) => {
                if *d == true {
                    return 1.0;
                } else {
                    return 0.0;
                }
            }
            Value::Null => { 0.0 }
            Value::Int32(i) => { *i as f64 }
            Value::Int64(d) => { *d as f64 }
            Value::Timestamp(d) => {
                as_timestamp(d) as f64
            }
            Value::DateTime(d) => {
                d.timestamp_millis() as f64
            }
            Value::Decimal128(d) => { d.to_string().parse().unwrap_or_default() }
            _ => { 0.0 }
        }
    }

    fn bool(&self) -> bool {
        self.as_bool().unwrap_or_default()
    }
    fn is_empty(&self) -> bool {
        return match self {
            Value::Null => {
                true
            }
            Value::String(s) => {
                s.is_empty()
            }
            Value::Array(arr) => {
                arr.is_empty()
            }
            Value::Document(m) => {
                m.is_empty()
            }
            _ => {
                return false;
            }
        };
    }

    fn is_null(&self) -> bool {
        return match self {
            Value::Null => { true }
            _ => { false }
        };
    }

    fn is_array(&self) -> bool {
        return match self {
            Value::Array(_) => { true }
            _ => { false }
        };
    }

    fn array(&self) -> Option<&rbson::Array> {
        return match self {
            Value::Array(arr) => { Some(arr) }
            _ => { None }
        };
    }

    fn is_document(&self) -> bool {
        return match self {
            Value::Document(_) => { true }
            _ => { false }
        };
    }

    fn is_object(&self) -> bool {
        return self.is_document();
    }

    fn object(&self) -> Option<&Document> {
        return match self {
            Value::Document(d) => { Some(d) }
            _ => { None }
        };
    }

    fn bracket(&self) -> &str {
        let bracket = self.as_str().unwrap_or_default();
        let start = bracket.find("(");
        let end = bracket.find(")");
        if let Some(start) = start {
            if let Some(end) = end {
                if end > (start + 1) {
                    return &bracket[start + 1..end];
                }
            }
        }
        return bracket;
    }
}

pub trait PartialEq<Rhs: ?Sized = Self> {
    /// This method tests for `self` and `other` values to be equal, and is used
    /// by `==`.
    #[must_use]
    //#[stable(feature = "rust1", since = "1.0.0")]
    fn op_eq(&self, other: &Rhs) -> bool;

    /// This method tests for `!=`.
    #[inline]
    #[must_use]
    //#[stable(feature = "rust1", since = "1.0.0")]
    fn op_ne(&self, other: &Rhs) -> bool {
        !self.op_eq(other)
    }
}

pub trait PartialOrd<Rhs: ?Sized = Self> {
    /// This method returns an ordering between `self` and `other` values if one exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::Ordering;
    ///
    /// let result = 1.0.op_partial_cmp(&2.0);
    /// assert_eq!(result, Some(Ordering::Less));
    ///
    /// let result = 1.0.op_partial_cmp(&1.0);
    /// assert_eq!(result, Some(Ordering::Equal));
    ///
    /// let result = 2.0.op_partial_cmp(&1.0);
    /// assert_eq!(result, Some(Ordering::Greater));
    /// ```
    ///
    /// When comparison is impossible:
    ///
    /// ```
    /// let result = f64::NAN.op_partial_cmp(&1.0);
    /// assert_eq!(result, None);
    /// ```
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    fn op_partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    /// This method tests less than (for `self` and `other`) and is used by the `<` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = 1.0 < 2.0;
    /// assert_eq!(result, true);
    ///
    /// let result = 2.0 < 1.0;
    /// assert_eq!(result, false);
    /// ```
    #[inline]
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    fn op_lt(&self, other: &Rhs) -> bool {
        self.op_partial_cmp(other).eq(&Some(Less))
    }

    /// This method tests less than or equal to (for `self` and `other`) and is used by the `<=`
    /// operator.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = 1.0 <= 2.0;
    /// assert_eq!(result, true);
    ///
    /// let result = 2.0 <= 2.0;
    /// assert_eq!(result, true);
    /// ```
    #[inline]
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    fn op_le(&self, other: &Rhs) -> bool {
        // Pattern `Some(Less | Eq)` optimizes worse than negating `None | Some(Greater)`.
        // FIXME: The root cause was fixed upstream in LLVM with:
        // https://github.com/llvm/llvm-project/commit/9bad7de9a3fb844f1ca2965f35d0c2a3d1e11775
        // Revert this workaround once support for LLVM 12 gets dropped.
        let v = self.op_partial_cmp(other);
        !v.eq(&None) | v.eq(&Some(Ordering::Greater))
    }

    /// This method tests greater than (for `self` and `other`) and is used by the `>` operator.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = 1.0 > 2.0;
    /// assert_eq!(result, false);
    ///
    /// let result = 2.0 > 2.0;
    /// assert_eq!(result, false);
    /// ```
    #[inline]
    // #[stable(feature = "rust1", since = "1.0.0")]
    fn op_gt(&self, other: &Rhs) -> bool {
        self.op_partial_cmp(other).eq(&Some(Ordering::Greater))
    }

    /// This method tests greater than or equal to (for `self` and `other`) and is used by the `>=`
    /// operator.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = 2.0 >= 1.0;
    /// assert_eq!(result, true);
    ///
    /// let result = 2.0 >= 2.0;
    /// assert_eq!(result, true);
    /// ```
    #[inline]
    #[must_use]
    // #[stable(feature = "rust1", since = "1.0.0")]
    fn op_ge(&self, other: &Rhs) -> bool {
        let v = self.op_partial_cmp(other);
        v.eq(&Some(Ordering::Greater)) | v.eq(&Some(Ordering::Equal))
    }
}

pub trait Add<Rhs = Self> {
    /// The resulting type after applying the `+` operator.
    //#[stable(feature = "rust1", since = "1.0.0")]
    type Output;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 + 1, 13);
    /// ```
    #[must_use]
    //#[stable(feature = "rust1", since = "1.0.0")]
    fn op_add(self, rhs: Rhs) -> Self::Output;
}


pub trait Sub<Rhs = Self> {
    /// The resulting type after applying the `-` operator.
    type Output;

    /// Performs the `-` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 - 1, 11);
    /// ```
    #[must_use]
    fn op_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait Mul<Rhs = Self> {
    /// The resulting type after applying the `*` operator.
    type Output;

    /// Performs the `*` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 * 2, 24);
    /// ```
    #[must_use]
    fn op_mul(self, rhs: Rhs) -> Self::Output;
}

pub trait Div<Rhs = Self> {
    /// The resulting type after applying the `/` operator.
    type Output;

    /// Performs the `/` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 / 2, 6);
    /// ```
    #[must_use]
    fn op_div(self, rhs: Rhs) -> Self::Output;
}

pub trait Rem<Rhs = Self> {
    /// The resulting type after applying the `%` operator.
    type Output;

    /// Performs the `%` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 % 10, 2);
    /// ```
    #[must_use]
    fn op_rem(self, rhs: Rhs) -> Self::Output;
}

pub trait Not {
    /// The resulting type after applying the `!` operator.
    type Output;

    /// Performs the unary `!` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(!true, false);
    /// assert_eq!(!false, true);
    /// assert_eq!(!1u8, 254);
    /// assert_eq!(!0u8, 255);
    /// ```
    #[must_use]
    fn op_not(self) -> Self::Output;
}

pub trait BitAnd<Rhs = Self> {
    /// The resulting type after applying the `&` operator.
    type Output;

    /// Performs the `&` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(true & false, false);
    /// assert_eq!(true & true, true);
    /// assert_eq!(5u8 & 1u8, 1);
    /// assert_eq!(5u8 & 2u8, 0);
    /// ```
    #[must_use]
    fn op_bitand(self, rhs: Rhs) -> Self::Output;
}

pub trait BitOr<Rhs = Self> {
    /// The resulting type after applying the `|` operator.
    type Output;

    /// Performs the `|` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(true | false, true);
    /// assert_eq!(false | false, false);
    /// assert_eq!(5u8 | 1u8, 5);
    /// assert_eq!(5u8 | 2u8, 7);
    /// ```
    #[must_use]
    fn op_bitor(self, rhs: Rhs) -> Self::Output;
}

pub trait BitXor<Rhs = Self> {
    /// The resulting type after applying the `^` operator.
    type Output;

    /// Performs the `^` operation.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(true ^ false, true);
    /// assert_eq!(true ^ true, false);
    /// assert_eq!(5u8 ^ 1u8, 4);
    /// assert_eq!(5u8 ^ 2u8, 7);
    /// ```
    #[must_use]
    fn op_bitxor(self, rhs: Rhs) -> Self::Output;
}


pub trait OpsIndex<Idx: ?Sized> {
    /// The returned type after indexing.
    type Output: ?Sized;

    /// Performs the indexing (`container[index]`) operation.
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    #[track_caller]
    fn index(&self, index: Idx) -> &Self::Output;
}

pub trait OpsIndexMut<Idx: ?Sized>: OpsIndex<Idx> {
    /// Performs the mutable indexing (`container[index]`) operation.
    ///
    /// # Panics
    ///
    /// May panic if the index is out of bounds.
    #[track_caller]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}

pub trait From<T>: Sized {
    /// Performs the conversion.
    fn op_from(_: T) -> Self;
}

pub trait AsSql {
    /// Performs the conversion.
    fn as_sql(&self) -> String;
}


#[cfg(test)]
mod test {
    use rbson::{Bson, bson};
    use rbson::spec::BinarySubtype;
    use crate::ops::AsProxy;

    #[test]
    fn test_string() {
        let b = Bson::Binary(rbson::Binary {
            subtype: BinarySubtype::Generic,
            bytes: "s".as_bytes().to_owned(),
        });
        assert_eq!("s", b.string());
    }

    #[test]
    fn test_cast() {
        let b = bson!(u64::MAX);
        assert_eq!(b.cast_i64(), -1);
        let b = bson!(100u64);
        assert_eq!(b.cast_i64(), 100i64);
    }
}

