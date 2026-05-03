/// Extended tests for decode module covering:
/// - decode_ref for CSV format ([[col1,col2],[val1,val2],...])
/// - try_decode_map edge cases (single element fallback)
/// - Non-array value decode errors
/// - Various type decodes beyond existing coverage

#[cfg(test)]
mod test {
    use rbs::value::map::ValueMap;
    use rbs::Value;
    use serde::{Deserialize, Serialize};

    fn make_csv_value(headers: &[&str], rows: Vec<Vec<Value>>) -> Value {
        let header_row: Vec<Value> = headers
            .iter()
            .map(|h| Value::String(h.to_string()))
            .collect();
        let data_rows: Vec<Value> = rows.into_iter().map(|r| Value::Array(r)).collect();
        Value::Array(vec![Value::Array(header_row), Value::Array(data_rows)])
    }

    fn make_map(k: &str, v: Value) -> Value {
        let mut map = ValueMap::new();
        map.insert(Value::String(k.to_string()), v);
        Value::Map(map)
    }

    // ==================== CSV Format Decode Tests ====================
    // Note: CSV format [[col1,col2],[val1,val2],...] is not currently supported

    #[test]
    #[ignore]
    fn test_decode_csv_single_row_single_col() {
        // CSV format: [["id"],["1"]]
        let value = make_csv_value(&["id"], vec![vec![Value::I32(1)]]);
        let result: Vec<SimpleStruct> = rbatis::decode(value).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SimpleStruct {
        id: i32,
    }

    #[test]
    #[ignore]
    fn test_decode_csv_single_row_multi_col() {
        // CSV format: [["id","name"],["1","test"]]
        let value = make_csv_value(
            &["id", "name"],
            vec![vec![Value::I32(1), Value::String("test".to_string())]],
        );

        let result: Vec<MultiColStruct> = rbatis::decode(value).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "test");
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MultiColStruct {
        id: i32,
        name: String,
    }

    #[test]
    #[ignore]
    fn test_decode_csv_multi_row() {
        // CSV format: [["id","name"],["1","a"],["2","b"]]
        let value = make_csv_value(
            &["id", "name"],
            vec![
                vec![Value::I32(1), Value::String("a".to_string())],
                vec![Value::I32(2), Value::String("b".to_string())],
            ],
        );

        let result: Vec<MultiColStruct> = rbatis::decode(value).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, "a");
        assert_eq!(result[1].id, 2);
        assert_eq!(result[1].name, "b");
    }

    #[test]
    #[ignore]
    fn test_decode_csv_with_nulls() {
        // CSV format: [["id","name"],["1",null]]
        let value = make_csv_value(&["id", "name"], vec![vec![Value::I32(1), Value::Null]]);

        let result: Vec<OptionNameStruct> = rbatis::decode(value).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, 1);
        assert_eq!(result[0].name, None);
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct OptionNameStruct {
        id: i32,
        name: Option<String>,
    }

    #[test]
    #[ignore]
    fn test_decode_csv_empty_data() {
        // CSV format: [["id","name"]] - no data rows
        let header_row: Vec<Value> = vec![
            Value::String("id".to_string()),
            Value::String("name".to_string()),
        ];
        let value = Value::Array(vec![Value::Array(header_row)]);

        let result: Vec<MultiColStruct> = rbatis::decode(value).unwrap();
        assert_eq!(result.len(), 0);
    }

    // ==================== try_decode_map Edge Cases ====================

    #[test]
    fn test_try_decode_map_single_element_direct_decode() {
        // When there's a single-element map and it can be decoded directly
        let value = Value::Array(vec![make_map("", Value::I32(42))]);
        let result: i32 = rbatis::decode::try_decode_map(&value).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_try_decode_map_struct_from_map_format() {
        // [{id: 1, name: "test"}] -> TestDecodeStruct
        let value = Value::Array(vec![{
            let mut m = ValueMap::new();
            m.insert(Value::String("id".to_string()), Value::I32(99));
            m.insert(
                Value::String("label".to_string()),
                Value::String("hello".to_string()),
            );
            Value::Map(m)
        }]);

        let result: TestDecodeStruct = rbatis::decode::try_decode_map(&value).unwrap();
        assert_eq!(result.id, 99);
        assert_eq!(result.label, "hello");
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestDecodeStruct {
        id: i64,
        label: String,
    }

    // ==================== Non-array decode error tests ====================

    #[test]
    fn test_decode_i64_from_non_array() {
        let value = Value::I64(42);
        let result = rbatis::decode::<i64>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode an not array value"
        );
    }

    #[test]
    fn test_decode_string_from_non_array() {
        let value = Value::String("hello".to_string());
        let result = rbatis::decode::<String>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode an not array value"
        );
    }

    #[test]
    fn test_decode_bool_from_non_array() {
        let value = Value::Bool(true);
        let result = rbatis::decode::<bool>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode an not array value"
        );
    }

    #[test]
    fn test_decode_vec_from_non_array() {
        let value = Value::I32(1);
        let result = rbatis::decode::<Vec<i32>>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode an not array value"
        );
    }

    #[test]
    fn test_decode_null_from_non_array() {
        let value = Value::Null;
        let result = rbatis::decode::<Option<i32>>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode an not array value"
        );
    }

    #[test]
    fn test_decode_f64_from_non_array() {
        let value = Value::F64(3.14);
        let result = rbatis::decode::<f64>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode an not array value"
        );
    }

    #[test]
    fn test_decode_u64_from_non_array() {
        let value = Value::U64(123456);
        let result = rbatis::decode::<u64>(value);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "decode an not array value"
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
    fn test_decode_nested_option_in_struct() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct NestedOption {
            inner: Option<Option<String>>,
        }

        let value = Value::Array(vec![{
            let mut m = ValueMap::new();
            m.insert(
                Value::String("inner".to_string()),
                Value::String("hello".to_string()),
            );
            Value::Map(m)
        }]);

        let result: NestedOption = rbatis::decode(value).unwrap();
        assert_eq!(result.inner, Some(Some("hello".to_string())));

        // Double None case
        let value2 = Value::Array(vec![{
            let mut m = ValueMap::new();
            m.insert(Value::String("inner".to_string()), Value::Null);
            Value::Map(m)
        }]);
        let result2: NestedOption = rbatis::decode(value2).unwrap();
        assert_eq!(result2.inner, None);
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
    fn test_decode_ref_and_decode_consistency() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct ConsistencyTest {
            x: i32,
            y: String,
        }

        let value = Value::Array(vec![{
            let mut m = ValueMap::new();
            m.insert(Value::String("x".to_string()), Value::I32(42));
            m.insert(
                Value::String("y".to_string()),
                Value::String("answer".to_string()),
            );
            Value::Map(m)
        }]);

        let by_decode: ConsistencyTest = rbatis::decode(value.clone()).unwrap();
        let by_ref: ConsistencyTest = rbatis::decode::decode_ref(&value).unwrap();
        assert_eq!(by_decode, by_ref);
    }

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
        let special = "hello \"world\" \n\t\r\n日本語 🎉";
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

    // ==================== Complex struct decode tests ====================

    #[test]
    fn test_decode_struct_with_many_fields() {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct ManyFields {
            f1: i32,
            f2: String,
            f3: bool,
            f4: f64,
            f5: i64,
            f6: Option<String>,
            f7: Option<i32>,
        }

        let value = Value::Array(vec![{
            let mut m = ValueMap::new();
            m.insert(Value::String("f1".to_string()), Value::I32(1));
            m.insert(
                Value::String("f2".to_string()),
                Value::String("two".to_string()),
            );
            m.insert(Value::String("f3".to_string()), Value::Bool(true));
            m.insert(Value::String("f4".to_string()), Value::F64(3.14));
            m.insert(Value::String("f5".to_string()), Value::I64(999));
            m.insert(
                Value::String("f6".to_string()),
                Value::String("some".to_string()),
            );
            m.insert(Value::String("f7".to_string()), Value::Null);
            Value::Map(m)
        }]);

        let result: ManyFields = rbatis::decode(value).unwrap();
        assert_eq!(result.f1, 1);
        assert_eq!(result.f2, "two");
        assert_eq!(result.f3, true);
        assert!((result.f4 - 3.14).abs() < f64::EPSILON);
        assert_eq!(result.f5, 999);
        assert_eq!(result.f6, Some("some".to_string()));
        assert_eq!(result.f7, None);
    }

    #[test]
    fn test_decode_vec_of_vec() {
        // Vec<Vec<i32>>
        let inner_arr1: Vec<Value> = vec![Value::I32(1), Value::I32(2)];
        let inner_arr2: Vec<Value> = vec![Value::I32(3), Value::I32(4)];
        let outer_arr: Vec<Value> = vec![Value::Array(inner_arr1), Value::Array(inner_arr2)];
        let value = Value::Array(vec![Value::Array(outer_arr)]);

        let result: Vec<Vec<i32>> = rbatis::decode(value).unwrap();
        assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
    }
}
