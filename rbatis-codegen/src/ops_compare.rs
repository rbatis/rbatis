//! Shared value comparison semantics for `PartialEq` and `PartialOrd`.

use rbs::Value;
use std::cmp::Ordering;

#[inline]
fn value_without_ext(mut value: &Value) -> &Value {
    while let Value::Ext(_, inner) = value {
        value = inner;
    }
    value
}

#[inline]
pub(crate) fn integer_value(value: &Value) -> Option<i128> {
    match value_without_ext(value) {
        Value::I32(value) => Some(*value as i128),
        Value::I64(value) => Some(*value as i128),
        Value::U32(value) => Some(*value as i128),
        Value::U64(value) => Some(*value as i128),
        _ => None,
    }
}

#[inline]
fn floating_value(value: &Value) -> Option<f64> {
    match value_without_ext(value) {
        Value::I32(value) => Some(*value as f64),
        Value::I64(value) => Some(*value as f64),
        Value::U32(value) => Some(*value as f64),
        Value::U64(value) => Some(*value as f64),
        Value::F32(value) => Some(*value as f64),
        Value::F64(value) => Some(*value),
        _ => None,
    }
}

/// Compares two values using the same numeric rules as primitive comparisons.
pub(crate) fn value_partial_cmp(left: &Value, right: &Value) -> Option<Ordering> {
    let left = value_without_ext(left);
    let right = value_without_ext(right);

    match (left, right) {
        (Value::Null, Value::Null) => Some(Ordering::Equal),
        (Value::Bool(left), Value::Bool(right)) => Some(left.cmp(right)),
        (Value::String(left), Value::String(right)) => Some(left.cmp(right)),
        _ => {
            if let (Some(left), Some(right)) = (integer_value(left), integer_value(right)) {
                return Some(left.cmp(&right));
            }
            floating_value(left)?.partial_cmp(&floating_value(right)?)
        }
    }
}

/// Tests two values for equality using numeric comparison where applicable.
pub(crate) fn value_eq(left: &Value, right: &Value) -> bool {
    let left = value_without_ext(left);
    let right = value_without_ext(right);

    if let (Some(left), Some(right)) = (integer_value(left), integer_value(right)) {
        return left == right;
    }
    if let (Some(left), Some(right)) = (floating_value(left), floating_value(right)) {
        return left == right;
    }
    left == right
}

#[inline]
fn value_cmp_i128(value: &Value, rhs: i128) -> Option<Ordering> {
    match value_without_ext(value) {
        Value::I32(value) => Some((*value as i128).cmp(&rhs)),
        Value::I64(value) => Some((*value as i128).cmp(&rhs)),
        Value::U32(value) => Some((*value as i128).cmp(&rhs)),
        Value::U64(value) => Some((*value as i128).cmp(&rhs)),
        Value::F32(value) => (*value as f64).partial_cmp(&(rhs as f64)),
        Value::F64(value) => value.partial_cmp(&(rhs as f64)),
        _ => None,
    }
}

#[inline]
fn value_cmp_f64_inner(value: &Value, rhs: f64) -> Option<Ordering> {
    floating_value(value)?.partial_cmp(&rhs)
}

#[inline]
fn value_cmp_bool_inner(value: &Value, rhs: bool) -> Option<Ordering> {
    match value_without_ext(value) {
        Value::Bool(value) => Some(value.cmp(&rhs)),
        _ => None,
    }
}

macro_rules! impl_value_scalar_compare_helpers {
    ($($compare:ident, $eq:ident, $reverse:ident, $rhs:ty, $cmp_inner:ident);* $(;)?) => {
        $(
            #[inline]
            pub(crate) fn $compare(value: &Value, rhs: $rhs) -> Option<Ordering> {
                $cmp_inner(value, rhs as _)
            }

            #[inline]
            pub(crate) fn $eq(value: &Value, rhs: $rhs) -> bool {
                matches!($compare(value, rhs), Some(Ordering::Equal))
            }

            #[inline]
            pub(crate) fn $reverse(value: &Value, rhs: $rhs) -> Option<Ordering> {
                $compare(value, rhs).map(Ordering::reverse)
            }
        )*
    };
}

impl_value_scalar_compare_helpers! {
    value_cmp_i64, value_eq_i64, value_cmp_i64_reverse, i64, value_cmp_i128;
    value_cmp_u64, value_eq_u64, value_cmp_u64_reverse, u64, value_cmp_i128;
    value_cmp_f64, value_eq_f64, value_cmp_f64_reverse, f64, value_cmp_f64_inner;
    value_cmp_bool, value_eq_bool, value_cmp_bool_reverse, bool, value_cmp_bool_inner;
}
