use rbs::Value;
use rbatis_codegen::into_sql::IntoSql;
use std::borrow::Cow;

#[test]
fn test_into_sql_primitives() {
    // 测试基本类型实现
    assert_eq!(true.sql(), "true");
    assert_eq!(false.sql(), "false");
    assert_eq!("test".sql(), "test");
    assert_eq!("test".to_string().sql(), "test");
    assert_eq!(123.sql(), "123");
    assert_eq!(123i64.sql(), "123");
    assert_eq!(123.5f32.sql(), "123.5");
    assert_eq!(123.5f64.sql(), "123.5");
    assert_eq!(123u32.sql(), "123");
    assert_eq!(123u64.sql(), "123");
}

#[test]
fn test_value_into_sql() {
    // 测试Value类型
    assert_eq!(Value::String("test".into()).sql(), "'test'");
    assert_eq!(Value::I32(123).sql(), "123");
    assert_eq!(Value::Bool(true).sql(), "true");
    
    // 测试数组
    let arr = Value::Array(vec![
        Value::I32(1),
        Value::I32(2),
        Value::String("test".into()),
    ]);
    assert_eq!(arr.sql(), "(1,2,'test')");
    
    // 测试空数组
    let empty_arr = Value::Array(vec![]);
    assert_eq!(empty_arr.sql(), "()");
    
    // 测试Map
    let mut map = rbs::value::map::ValueMap::new();
    map.insert(Value::String("key1".into()), Value::I32(123));
    map.insert(Value::String("key2".into()), Value::String("value".into()));
    
    let map_value = Value::Map(map);
    // 注意：Map顺序不确定，所以这里只能检查包含关系而不是完全相等
    let sql = map_value.sql();
    assert!(sql.contains("key1123") || sql.contains("key2'value'"));
}

#[test]
fn test_value_ref_into_sql() {
    // 测试Value引用类型
    let val = Value::String("test".into());
    assert_eq!((&val).sql(), "'test'");
    
    // 测试Cow<Value>
    let cow = Cow::Borrowed(&val);
    assert_eq!(cow.sql(), "'test'");
}

#[test]
fn test_complex_value() {
    // 测试复杂嵌套结构
    let mut inner_map = rbs::value::map::ValueMap::new();
    inner_map.insert(Value::String("inner_key".into()), Value::I32(456));
    
    let mut map = rbs::value::map::ValueMap::new();
    map.insert(Value::String("key1".into()), Value::Map(inner_map));
    map.insert(Value::String("key2".into()), Value::Array(vec![Value::I32(1), Value::I32(2)]));
    
    let complex = Value::Map(map);
    let sql = complex.sql();
    
    // 验证生成的SQL包含所有必要元素
    assert!(sql.contains("key1"));
    assert!(sql.contains("key2"));
    assert!(sql.contains("(1,2)"));
} 