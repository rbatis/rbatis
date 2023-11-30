use crate::ops::BitOr;
use rbs::Value;

impl BitOr for Value {
    type Output = bool;

    fn op_bitor(self, rhs: Self) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<Value> for bool {
    type Output = bool;

    fn op_bitor(self, rhs: Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}

//ref
impl BitOr<Value> for &Value {
    type Output = bool;

    fn op_bitor(self, rhs: Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&Value> for &Value {
    type Output = bool;

    fn op_bitor(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&&Value> for &Value {
    type Output = bool;

    fn op_bitor(self, rhs: &&Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<bool> for &Value {
    type Output = bool;

    fn op_bitor(self, rhs: bool) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs
    }
}

//rhs ref
impl BitOr<&Value> for Value {
    type Output = bool;

    fn op_bitor(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<&Value> for bool {
    type Output = bool;

    fn op_bitor(self, rhs: &Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<&&Value> for Value {
    type Output = bool;

    fn op_bitor(self, rhs: &&Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<&&Value> for bool {
    type Output = bool;

    fn op_bitor(self, rhs: &&Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}



fn op_bit_or_u64(v: &Value, other: u64) -> u64 {
    std::ops::BitAnd::bitand(v.as_u64().unwrap_or_default(),other)
}

fn op_bit_or_i64(v: &Value, other: i64) -> i64 {
    std::ops::BitAnd::bitand(v.as_i64().unwrap_or_default(),other)
}


macro_rules! impl_numeric_bitor {
    ($($eq:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl BitOr<$ty> for Value {
                type Output = $return_ty;
                fn op_bitor(self, other: $ty) -> Self::Output {
                    $eq(&self, other as _)
                }
            }

            impl BitOr<&$ty> for Value {
                type Output = $return_ty;
                fn op_bitor(self, other: &$ty) -> Self::Output {
                    $eq(&self, *other as _)
                }
            }

            impl<'a> BitOr<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_bitor(self, other: $ty) -> Self::Output {
                    $eq(self, other as _)
                }
            }

            impl<'a> BitOr<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_bitor(self, other: &$ty) -> Self::Output {
                    $eq(self, *other as _)
                }
            }

            impl BitOr<Value> for $ty {
                type Output = $return_ty;
                fn op_bitor(self, other: Value) -> Self::Output {
                    $eq(&other, self as _)
                }
            }

            impl BitOr<&Value> for $ty {
                type Output = $return_ty;
                fn op_bitor(self, other: &Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }
            impl BitOr<&&Value> for $ty {
                type Output = $return_ty;
                fn op_bitor(self, other: &&Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }
        )*)*
    }
}

impl_numeric_bitor! {
    op_bit_or_u64[u8 u16 u32 u64] -> u64
    op_bit_or_i64[i8 i16 i32 i64 isize] -> i64
}