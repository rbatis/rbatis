use rbatis_codegen::ops::{
    Add, BitAnd, BitOr, BitShl, BitShr, BitXor, Div, Mul, PartialEq, PartialOrd, Rem, Sub,
};
use rbs::Value;
use std::cmp::Ordering;

#[test]
fn test_value_reference_matrix() {
    let left = Value::I64(8);
    let right = Value::I64(2);

    assert_eq!((&&left).op_add(&&right), Value::I64(10));
    assert_eq!((&&left).op_sub(&&right), Value::I64(6));
    assert_eq!((&&left).op_mul(&&right), Value::I64(16));
    assert_eq!((&&left).op_div(&&right), Value::I64(4));
    assert_eq!((&&left).op_rem(&&right), Value::I64(0));
    assert!(!(&&left).op_bitand(&&right));
    assert!(!(&&left).op_bitor(&&right));
    assert_eq!((&&left).op_bitxor(&&right), Value::I64(10));
    assert_eq!((&&left).op_shl(&&right), Value::I64(32));
    assert_eq!((&&left).op_shr(&&right), Value::I64(2));
}

#[test]
fn test_primitive_reference_matrix() {
    let value = Value::I64(2);

    assert_eq!((&8i64).op_add(&&value), 10i64);
    assert_eq!((&8i64).op_sub(&&value), 6i64);
    assert_eq!((&8i64).op_mul(&&value), 16i64);
    assert_eq!((&8i64).op_div(&&value), 4i64);
    assert_eq!((&8i64).op_rem(&&value), 0i64);
    assert_eq!((&8i64).op_bitand(&&value), 0i64);
    assert_eq!((&8i64).op_bitor(&&value), 10i64);
    assert_eq!((&8i64).op_bitxor(&&value), 10i64);
    assert_eq!((&8i64).op_shl(&&value), 32i64);
    assert_eq!((&8i64).op_shr(&&value), 2i64);
}

#[test]
fn test_bool_reference_matrix() {
    let left = Value::Bool(true);
    let right = Value::Bool(false);

    assert!(!(&&left).op_bitand(&&right));
    assert!((&&left).op_bitor(&&right));
    assert_eq!((&&left).op_bitxor(&&right), Value::Bool(true));
    assert!(!(&true).op_bitand(&&right));
    assert!((&true).op_bitor(&&right));
    assert!((&true).op_bitxor(&&right));
}

#[test]
fn test_comparison_matrix() {
    assert!(1isize.op_eq(&1u8));
    assert!(1.0f32.op_eq(&1i64));
    assert!(!u64::MAX.op_eq(&-1i64));
    assert_eq!(1isize.op_partial_cmp(&1u8), Some(Ordering::Equal));
    assert_eq!(1.0f32.op_partial_cmp(&1i64), Some(Ordering::Equal));
    assert_eq!(true.op_partial_cmp(&false), Some(Ordering::Greater));

    let i32_value = Value::I32(10);
    let i64_value = Value::I64(10);
    let f64_value = Value::F64(10.0);
    assert!(i32_value.op_eq(&i64_value));
    assert!(i32_value.op_eq(&f64_value));
    assert_eq!(i32_value.op_partial_cmp(&i64_value), Some(Ordering::Equal));
    assert_eq!(i32_value.op_partial_cmp(&f64_value), Some(Ordering::Equal));

    assert!(Value::I64(10).op_eq(&10u32));
    assert_eq!(Value::I64(10).op_partial_cmp(&10u32), Some(Ordering::Equal));
    assert!(!Value::I64(-1).op_eq(&u64::MAX));
    assert_eq!(
        Value::I64(-1).op_partial_cmp(&u64::MAX),
        Some(Ordering::Less)
    );
    assert!(!Value::U64(u64::MAX).op_eq(&Value::I64(-1)));
    assert_eq!(
        Value::U64(u64::MAX).op_partial_cmp(&Value::I64(-1)),
        Some(Ordering::Greater)
    );

    let precise = Value::I64(9_007_199_254_740_993);
    let rounded = Value::I64(9_007_199_254_740_992);
    assert!(!precise.op_eq(&rounded));
    assert_eq!(precise.op_partial_cmp(&rounded), Some(Ordering::Greater));

    assert!(Value::Null.op_eq(&Value::Null));
    assert!(!Value::Null.op_eq(&Value::I32(0)));
    assert_eq!(
        Value::Null.op_partial_cmp(&Value::Null),
        Some(Ordering::Equal)
    );
    assert_eq!(Value::Null.op_partial_cmp(&Value::I32(0)), None);

    let nan = Value::F64(f64::NAN);
    assert!(!nan.op_eq(&nan));
    assert_eq!(nan.op_partial_cmp(&nan), None);
}
