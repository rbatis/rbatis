use rbatis_codegen::ops::{BitShl, BitShr};
use rbs::Value;

#[test]
fn test_shift_reference_matrix() {
    let shifted = Value::I64(8);
    let amount = Value::I64(2);

    assert_eq!((&shifted).op_shl(&&amount), Value::I64(32));
    assert_eq!((&shifted).op_shr(&&amount), Value::I64(2));
    assert_eq!((&&shifted).op_shl(&Value::I64(2)), Value::I64(32));
    assert_eq!((&&shifted).op_shr(2i64), 2i64);
}

#[test]
fn test_shift_primitive_matrix() {
    assert_eq!(8i64.op_shl(&Value::I64(2)), 32i64);
    assert_eq!((&8i64).op_shl(&&Value::I64(2)), 32i64);
    assert_eq!(8i64.op_shr(&Value::I64(2)), 2i64);
    assert_eq!((&8i64).op_shr(&&Value::I64(2)), 2i64);
}

#[test]
fn test_shift_primitive_overflow_is_safe() {
    assert_eq!(1u8.op_shl(8u8), 0u8);
    assert_eq!(1u8.op_shr(8u8), 0u8);
    assert_eq!(1i64.op_shl(-1i64), 0i64);
}
