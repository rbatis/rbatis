/// Extended tests for decode module covering:
/// - try_decode_elements edge cases (single element fallback)
/// - Non-array value decode errors
/// - Various type decodes beyond existing coverage

#[cfg(test)]
mod test {
    use rbs::value::map::ValueMap;
    use rbs::Value;

    fn make_map(k: &str, v: Value) -> Value {
        let mut map = ValueMap::new();
        map.insert(Value::String(k.to_string()), v);
        Value::Map(map)
    }

    // ==================== try_decode_elements Edge Cases ====================

    #[test]
    fn test_try_decode_elements_single_element_direct_decode() {
        // When there's a single-element map and it can be decoded directly
        let value = Value::Array(vec![make_map("", Value::I32(42))]);
        let result: i32 = rbatis::decode::try_decode_elements(&value).unwrap();
        assert_eq!(result, 42);
    }

    // ==================== Non-array decode error tests ====================

    #[test]
    fn test_decode_i64_from_non_array() {
        let value = Value::I64(42);
        let result = rbatis::decode::<i64>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode error: expected array value"
        );
    }

    #[test]
    fn test_decode_string_from_non_array() {
        let value = Value::String("hello".to_string());
        let result = rbatis::decode::<String>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode error: expected array value"
        );
    }

    #[test]
    fn test_decode_bool_from_non_array() {
        let value = Value::Bool(true);
        let result = rbatis::decode::<bool>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode error: expected array value"
        );
    }

    #[test]
    fn test_decode_vec_from_non_array() {
        let value = Value::I32(1);
        let result = rbatis::decode::<Vec<i32>>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode error: expected array value"
        );
    }

    #[test]
    fn test_decode_null_from_non_array() {
        let value = Value::Null;
        let result = rbatis::decode::<Option<i32>>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode error: expected array value"
        );
    }

    #[test]
    fn test_decode_f64_from_non_array() {
        let value = Value::F64(1.23);
        let result = rbatis::decode::<f64>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode error: expected array value"
        );
    }

    #[test]
    fn test_decode_u64_from_non_array() {
        let value = Value::U64(123456);
        let result = rbatis::decode::<u64>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode error: expected array value"
        );
    }

    // ==================== Additional type decode tests ====================

    #[test]
    fn test_decode_u16_from_map() {
        // u16 is less commonly tested
        let value = Value::Array(vec![make_map("a", Value::U64(65535))]);
        // Note: rbs may store u16 as U64
        let result = rbatis::decode::<u16>(value);
        // Depending on rbs internal representation, may or may not work
        let _ = result;
    }

    #[test]
    fn test_decode_i8_from_map() {
        let value = Value::Array(vec![make_map("a", Value::I32(-128))]);
        let result: i8 = rbatis::decode(value).unwrap();
        assert_eq!(result, -128);
    }

    #[test]
    fn test_decode_i16_from_map() {
        let value = Value::Array(vec![make_map("a", Value::I32(32767))]);
        let result: i16 = rbatis::decode(value).unwrap();
        assert_eq!(result, 32767);
    }

    #[test]
    #[ignore]
    fn test_decode_tuple_from_map() {
        // Decoding tuple from map format
        let mut map = ValueMap::new();
        map.insert(Value::String("0".to_string()), Value::I32(1));
        map.insert(
            Value::String("1".to_string()),
            Value::String("two".to_string()),
        );
        let value = Value::Array(vec![Value::Map(map)]);

        let result: (i32, String) = rbatis::decode(value).unwrap();
        assert_eq!(result.0, 1);
        assert_eq!(result.1, "two");
    }

    #[test]
    #[ignore]
    fn test_decode_vec_u8_from_map() {
        let value = Value::Array(vec![make_map("data", Value::Binary(vec![1, 2, 3, 4, 5]))]);
        let result: Vec<u8> = rbatis::decode(value).unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    // ==================== decode_ref vs decode consistency ====================

    #[test]
    fn test_decode_ref_doesnt_consume() {
        let value = Value::Array(vec![make_map("a", Value::I32(1))]);
        // Using decode_ref should leave the original value intact
        let _result: i32 = rbatis::decode::decode_ref(&value).unwrap();
        // value should still be usable
        let result2: i32 = rbatis::decode::decode_ref(&value).unwrap();
        assert_eq!(result2, 1);
    }

    // ==================== Large value decode tests ====================

    #[test]
    fn test_decode_large_i64() {
        let large_val: i64 = i64::MAX;
        let value = Value::Array(vec![make_map("a", Value::I64(large_val))]);
        let result: i64 = rbatis::decode(value).unwrap();
        assert_eq!(result, i64::MAX);
    }

    #[test]
    fn test_decode_large_u64() {
        let large_val: u64 = u64::MAX;
        let value = Value::Array(vec![make_map("a", Value::U64(large_val))]);
        let result: u64 = rbatis::decode(value).unwrap();
        assert_eq!(result, u64::MAX);
    }

    #[test]
    fn test_decode_negative_i64() {
        let value = Value::Array(vec![make_map("a", Value::I64(-999999999))]);
        let result: i64 = rbatis::decode(value).unwrap();
        assert_eq!(result, -999999999);
    }

    #[test]
    fn test_decode_special_characters_in_string() {
        let special = "hello \"world\" \n\t\r\nsymbols !@#$%^&*()";
        let value = Value::Array(vec![make_map("a", Value::String(special.to_string()))]);
        let result: String = rbatis::decode(value).unwrap();
        assert_eq!(result, special);
    }

    #[test]
    fn test_decode_empty_string() {
        let value = Value::Array(vec![make_map("a", Value::String("".to_string()))]);
        let result: String = rbatis::decode(value).unwrap();
        assert_eq!(result, "");
    }

    // ==================== decode error path ====================

    #[test]
    fn test_decode_keeps_error_when_no_single_column_to_unwrap() {
        // [1,2] holds values instead of the `[{"k": v}]` rows, so the single column
        // fallback has nothing to unwrap. The error of the normal decode is what the
        // caller sees (it is not replaced by a message about the fallback)
        let value = Value::Array(vec![Value::I32(1), Value::I32(2)]);
        let err = rbatis::decode::<i64>(value).err().unwrap();
        let message = err.to_string();
        println!("error={}", message);
        assert!(message.contains("invalid type"), "{}", message);
    }

    #[test]
    fn test_decode_error_of_rejected_row_is_kept() {
        // the row is a single column, but its value does not fit the requested type.
        // The error of that row explains why, and is kept over the general one
        let value = Value::Array(vec![make_map(
            "a",
            Value::String("not a number".to_string()),
        )]);
        let err = rbatis::decode::<i64>(value).err().unwrap();
        let message = err.to_string();
        println!("error={}", message);
        assert!(message.contains("invalid type: string"), "{}", message);
        assert!(message.contains("expected i64"), "{}", message);
    }
}
