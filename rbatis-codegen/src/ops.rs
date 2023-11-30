use rbs::Value;
use std::cmp::Ordering;
pub use std::ops::Index;

/// convert Value to Value
pub trait AsProxy {
    fn i32(&self) -> i32;
    fn i64(&self) -> i64;
    fn u32(&self) -> u32;
    fn u64(&self) -> u64;
    fn f64(&self) -> f64;
    fn bool(&self) -> bool;

    fn string(&self) -> String;
    fn as_binary(&self) -> Vec<u8>;
}

impl AsProxy for Value {
    fn i32(&self) -> i32 {
        self.as_i64().unwrap_or_default() as i32
    }

    fn i64(&self) -> i64 {
        self.as_i64().unwrap_or_default()
    }

    fn u32(&self) -> u32 {
        self.as_u64().unwrap_or_default() as u32
    }

    fn u64(&self) -> u64 {
        self.as_u64().unwrap_or_default()
    }

    fn f64(&self) -> f64 {
        self.as_f64().unwrap_or_default()
    }

    fn string(&self) -> String {
        match self {
            Value::String(v) => v.to_string(),
            Value::Ext(_, ext) => ext.as_string().unwrap_or_default(),
            _ => self.to_string(),
        }
    }

    fn bool(&self) -> bool {
        self.as_bool().unwrap_or_default()
    }

    fn as_binary(&self) -> Vec<u8> {
        match self {
            Value::Binary(s) => s.to_owned(),
            _ => vec![],
        }
    }
}

impl AsProxy for bool {
    fn i32(&self) -> i32 {
        if *self {
            1
        } else {
            0
        }
    }

    fn i64(&self) -> i64 {
        if *self {
            1
        } else {
            0
        }
    }

    fn u32(&self) -> u32 {
        if *self {
            1
        } else {
            0
        }
    }

    fn u64(&self) -> u64 {
        if *self {
            1
        } else {
            0
        }
    }

    fn f64(&self) -> f64 {
        if *self {
            1.0
        } else {
            0.0
        }
    }

    fn bool(&self) -> bool {
        *self
    }

    fn string(&self) -> String {
        self.to_string()
    }

    fn as_binary(&self) -> Vec<u8> {
        if *self {
            vec![1u8]
        } else {
            vec![0u8]
        }
    }
}


macro_rules! as_number {
    ($ty:ty,$bool_expr:expr) => {
        impl AsProxy for $ty {
            fn i32(&self) -> i32 {
                *self as i32
            }

            fn i64(&self) -> i64 {
                *self as i64
            }

            fn u32(&self) -> u32 {
                *self as u32
            }

            fn u64(&self) -> u64 {
                *self as u64
            }

            fn f64(&self) -> f64 {
                *self as f64
            }

            fn string(&self) -> String {
                self.to_string()
            }

            fn bool(&self) -> bool {
                //*self==1
                if *self == $bool_expr {
                    true
                } else {
                    false
                }
            }

            fn as_binary(&self) -> Vec<u8> {
                self.to_string().into_bytes()
            }
        }
    };
}

as_number!(i8, 1i8);
as_number!(i16, 1i16);
as_number!(i32, 1i32);
as_number!(i64, 1i64);
as_number!(isize, 1isize);

as_number!(u8, 1u8);
as_number!(u16, 1u16);
as_number!(u32, 1u32);
as_number!(u64, 1u64);
as_number!(usize, 1usize);

as_number!(f32, 1.0);
as_number!(f64, 1.0);

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
    ///
    ///
    /// use std::cmp::Ordering;
    /// use rbatis_codegen::ops::PartialOrd;
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
    /// use rbatis_codegen::ops::PartialOrd;
    /// let result = f64::NAN.op_partial_cmp(&1.0);
    /// assert_eq!(result, None);
    /// ```
    #[must_use]
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
    fn op_lt(&self, other: &Rhs) -> bool {
        self.op_partial_cmp(other).eq(&Some(Ordering::Less))
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
    fn op_le(&self, other: &Rhs) -> bool {
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

pub trait Neg {
    /// The resulting type after applying the `-` operator.
    type Output;

    /// Performs the unary `-` operation.
    ///
    /// # Example
    ///
    /// ```
    /// let x: i32 = 12;
    /// assert_eq!(-x, -12);
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    fn neg(self) -> Self::Output;
}

#[cfg(test)]
mod test {
    use crate::ops::AsProxy;
    use rbs::to_value;

    #[test]
    fn test_cast() {
        let b = to_value!(u64::MAX);
        assert_eq!(b.i64(), -1);
        let b = to_value!(100u64);
        assert_eq!(b.i64(), 100i64);
    }
}
