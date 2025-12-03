use rbatis_codegen::IntoSql;
use rbs::Value;

#[test]
fn test_value_into_sql() {
    let value = Value::String("test".to_string());
    let sql = value.sql();
    assert_eq!(sql, "'test'");
    
    let value = Value::I32(42);
    let sql = value.sql();
    assert_eq!(sql, "42");
    
    let value = Value::I64(100);
    let sql = value.sql();
    assert_eq!(sql, "100");
    
    let value = Value::F64(3.14);
    let sql = value.sql();
    assert!(sql.contains("3.14"));
    
    let value = Value::Bool(true);
    let sql = value.sql();
    assert!(sql.contains("true"));
    
    let value = Value::Bool(false);
    let sql = value.sql();
    assert!(sql.contains("false"));
}

#[test]
fn test_value_array_into_sql() {
    let arr = vec![
        Value::String("a".to_string()),
        Value::String("b".to_string()),
        Value::String("c".to_string()),
    ];
    
    let value = Value::Array(arr);
    let sql = value.sql();
    assert!(sql.contains("'a'"));
    assert!(sql.contains("'b'"));
    assert!(sql.contains("'c'"));
    assert!(sql.starts_with("("));
    assert!(sql.ends_with(")"));
}

#[test]
fn test_string_into_sql() {
    let value = "test string".to_string();
    let sql = value.sql();
    assert_eq!(sql, "test string");
}

#[test]
fn test_str_into_sql() {
    let value = "test str";
    let sql = value.sql();
    assert_eq!(sql, "test str");
}

#[test]
fn test_number_into_sql() {
    let value: i32 = 42;
    let sql = value.sql();
    assert_eq!(sql, "42");
    
    let value: i64 = 100;
    let sql = value.sql();
    assert_eq!(sql, "100");
    
    let value: u32 = 10;
    let sql = value.sql();
    assert_eq!(sql, "10");
    
    let value: u64 = 20;
    let sql = value.sql();
    assert_eq!(sql, "20");
    
    let value: f64 = 3.14;
    let sql = value.sql();
    assert!(sql.contains("3.14"));
    
    let value: f32 = 2.71;
    let sql = value.sql();
    assert!(sql.contains("2.71"));
}

#[test]
fn test_bool_into_sql() {
    let value = true;
    let sql = value.sql();
    assert!(sql.contains("true"));
    
    let value = false;
    let sql = value.sql();
    assert!(sql.contains("false"));
}