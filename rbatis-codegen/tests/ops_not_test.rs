use rbs::Value;
use rbatis_codegen::ops::Not;

#[test]
fn test_value_not() {
    // Value::op_not (布尔取反)
    assert_eq!(Value::Bool(true).op_not(), Value::Bool(false));
    assert_eq!(Value::Bool(false).op_not(), Value::Bool(true));
    
    // Value::op_not (位运算取反)
    assert_eq!(Value::I32(5).op_not(), Value::I32(!5));      // !5 = -6 (位运算)
    assert_eq!(Value::I64(10).op_not(), Value::I64(!10));    // !10 = -11 (位运算)
    assert_eq!(Value::U32(15).op_not(), Value::U32(!15));    // !15 = 4294967280 (位运算)
    assert_eq!(Value::U64(20).op_not(), Value::U64(!20));    // !20 = 18446744073709551595 (位运算)
    
    // 双重取反应该回到原值
    assert_eq!(Value::I32(5).op_not().op_not(), Value::I32(5));
    assert_eq!(Value::Bool(true).op_not().op_not(), Value::Bool(true));
}

#[test]
fn test_value_ref_not() {
    // &Value::op_not
    let v1 = Value::I32(5);
    let v2 = Value::Bool(true);
    let v3 = Value::U32(15);
    
    assert_eq!((&v1).op_not(), Value::I32(!5)); // !5 = -6
    assert_eq!((&v2).op_not(), Value::Bool(false));
    assert_eq!((&v3).op_not(), Value::U32(!15)); // !15 = 4294967280
    
    // &&Value::op_not
    assert_eq!((&&v1).op_not(), Value::I32(!5));
    assert_eq!((&&v2).op_not(), Value::Bool(false));
} 