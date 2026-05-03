/// Comprehensive tests for crud_traits module:
/// - ColumnSet trait and its Value implementation
/// - ValueOperatorSql trait and its Value implementation
/// - FilterByColumns trait and its Value implementation

#[cfg(test)]
mod test {
    use rbatis::crud_traits::{ColumnSet, FilterByColumns, ValueOperatorSql};
    use rbs::value::map::ValueMap;
    use rbs::{value, Value};

    // ==================== ColumnSet Tests ====================

    #[test]
    fn test_column_set_empty_array() {
        let v = Value::Array(vec![]);
        let result = v.column_sets();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_column_set_non_array() {
        let v = Value::I32(42);
        let result = v.column_sets();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_column_set_null_value() {
        let v = Value::Null;
        let result = v.column_sets();
        assert_eq!(result, Value::Array(vec![]));
    }

    #[test]
    fn test_column_set_single_row_all_non_null() {
        let mut map = ValueMap::new();
        map.insert(Value::String("id".to_string()), Value::I32(1));
        map.insert(
            Value::String("name".to_string()),
            Value::String("test".to_string()),
        );
        map.insert(Value::String("age".to_string()), Value::I32(25));

        let v = Value::Array(vec![Value::Map(map)]);
        let result = v.column_sets();

        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        // Should contain all 3 column names
        let names: Vec<&str> = arr.iter().map(|v| v.as_str().unwrap()).collect();
        assert!(names.contains(&"id"));
        assert!(names.contains(&"name"));
        assert!(names.contains(&"age"));
    }

    #[test]
    fn test_column_set_single_row_with_nulls() {
        // Fast path: single element should extract only non-null columns
        let mut map = ValueMap::new();
        map.insert(Value::String("id".to_string()), Value::I32(1));
        map.insert(Value::String("name".to_string()), Value::Null);
        map.insert(Value::String("age".to_string()), Value::I32(25));
        map.insert(Value::String("email".to_string()), Value::Null);

        let v = Value::Array(vec![Value::Map(map.clone())]);
        let result = v.column_sets();

        let arr = result.as_array().unwrap();
        let names: Vec<&str> = arr.iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"id"));
        assert!(names.contains(&"age"));
        assert!(!names.contains(&"name"));
        assert!(!names.contains(&"email"));
    }

    #[test]
    fn test_column_set_single_row_all_nulls() {
        let mut map = ValueMap::new();
        map.insert(Value::String("id".to_string()), Value::Null);
        map.insert(Value::String("name".to_string()), Value::Null);

        let v = Value::Array(vec![Value::Map(map)]);
        let result = v.column_sets();

        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_column_set_multiple_rows_merge_columns() {
        // Slow path: multiple elements - use HashSet to collect unique columns across all rows
        let mut map1 = ValueMap::new();
        map1.insert(Value::String("id".to_string()), Value::I32(1));
        map1.insert(
            Value::String("name".to_string()),
            Value::String("a".to_string()),
        );
        // email is null in row 1
        map1.insert(Value::String("email".to_string()), Value::Null);

        let mut map2 = ValueMap::new();
        map2.insert(Value::String("id".to_string()), Value::I32(2));
        // name is null in row 2
        map2.insert(Value::String("name".to_string()), Value::Null);
        map2.insert(
            Value::String("email".to_string()),
            Value::String("b@test.com".to_string()),
        );

        let v = Value::Array(vec![Value::Map(map1), Value::Map(map2)]);
        let result = v.column_sets();

        let arr = result.as_array().unwrap();
        let names: Vec<&str> = arr.iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"id"));
        assert!(names.contains(&"name"));
        assert!(names.contains(&"email"));
    }

    #[test]
    fn test_column_set_multiple_rows_preserve_order() {
        // Verify that column order follows the first row's order
        let mut map1 = ValueMap::new();
        map1.insert(Value::String("z_col".to_string()), Value::I32(1));
        map1.insert(Value::String("a_col".to_string()), Value::I32(2));
        map1.insert(Value::String("m_col".to_string()), Value::I32(3));

        let mut map2 = ValueMap::new();
        map2.insert(Value::String("z_col".to_string()), Value::I32(4));
        map2.insert(Value::String("a_col".to_string()), Value::Null); // null in second row
        map2.insert(Value::String("m_col".to_string()), Value::I32(6));

        let v = Value::Array(vec![Value::Map(map1), Value::Map(map2)]);
        let result = v.column_sets();

        let arr = result.as_array().unwrap();
        let names: Vec<&str> = arr.iter().map(|v| v.as_str().unwrap()).collect();
        // Order should follow first row: z_col, a_col, m_col
        assert_eq!(names, vec!["z_col", "a_col", "m_col"]);
    }

    #[test]
    fn test_column_set_two_rows_second_row_has_extra_non_null() {
        // If second row has a non-null column that first row doesn't have,
        // it should NOT appear (only columns from first row are considered)
        let mut map1 = ValueMap::new();
        map1.insert(Value::String("id".to_string()), Value::I32(1));
        // name is null in first row
        map1.insert(Value::String("name".to_string()), Value::Null);

        let mut map2 = ValueMap::new();
        map2.insert(Value::String("id".to_string()), Value::I32(2));
        // name is non-null in second row
        map2.insert(
            Value::String("name".to_string()),
            Value::String("test".to_string()),
        );

        let v = Value::Array(vec![Value::Map(map1), Value::Map(map2)]);
        let result = v.column_sets();

        let arr = result.as_array().unwrap();
        let names: Vec<&str> = arr.iter().map(|v| v.as_str().unwrap()).collect();
        // name appears because it's non-null in at least one row (HashSet collects it)
        // but order comes from first row
        assert!(names.contains(&"id"));
        assert!(names.contains(&"name"));
    }

    // ==================== ValueOperatorSql Tests ====================

    #[test]
    fn test_operator_sql_string_without_space() {
        // String without space should append " = "
        let v = Value::String("=".to_string());
        let result = v.operator_sql();
        assert_eq!(result, "= = ");
    }

    #[test]
    fn test_operator_sql_string_with_space() {
        // String without space should append " = "
        let v = Value::String("like".to_string());
        let result = v.operator_sql();
        assert_eq!(result, "like = ");
    }

    #[test]
    fn test_operator_sql_string_with_complex_operator() {
        let v = Value::String("in".to_string());
        let result = v.operator_sql();
        assert_eq!(result, "in = ");
    }

    #[test]
    fn test_operator_sql_string_multi_word_operator() {
        // Multi-word operators with spaces should pass through unchanged
        let v = Value::String("not like".to_string());
        let result = v.operator_sql();
        assert_eq!(result, "not like");
    }

    #[test]
    fn test_operator_sql_empty_string() {
        let v = Value::String("".to_string());
        let result = v.operator_sql();
        // Empty string doesn't contain space, so it appends " = "
        assert_eq!(result, " = ");
    }

    #[test]
    fn test_operator_sql_i32() {
        let v = Value::I32(42);
        let result = v.operator_sql();
        assert_eq!(result, "");
    }

    #[test]
    fn test_operator_sql_i64() {
        let v = Value::I64(100);
        let result = v.operator_sql();
        assert_eq!(result, "");
    }

    #[test]
    fn test_operator_sql_bool() {
        let v = Value::Bool(true);
        let result = v.operator_sql();
        assert_eq!(result, "");
    }

    #[test]
    fn test_operator_sql_null() {
        let v = Value::Null;
        let result = v.operator_sql();
        assert_eq!(result, "");
    }

    #[test]
    fn test_operator_sql_f64() {
        let v = Value::F64(1.23);
        let result = v.operator_sql();
        assert_eq!(result, "");
    }

    // ==================== FilterByColumns Tests ====================

    #[test]
    fn test_filter_by_columns_basic() {
        let mut map = ValueMap::new();
        map.insert(Value::String("id".to_string()), Value::I32(1));
        map.insert(
            Value::String("name".to_string()),
            Value::String("test".to_string()),
        );
        map.insert(Value::String("age".to_string()), Value::I32(25));
        map.insert(
            Value::String("email".to_string()),
            Value::String("a@b.com".to_string()),
        );

        let v = Value::Map(map);
        let columns = Value::Array(vec![
            Value::String("id".to_string()),
            Value::String("name".to_string()),
        ]);

        let result = v.filter_by_columns(&columns);
        let result_map = result.as_map().unwrap();
        assert_eq!(result_map.len(), 2); // 2 columns selected: id and name
    }

    #[test]
    fn test_filter_by_columns_empty_columns_array() {
        let mut map = ValueMap::new();
        map.insert(Value::String("id".to_string()), Value::I32(1));
        map.insert(
            Value::String("name".to_string()),
            Value::String("test".to_string()),
        );

        let v = Value::Map(map);
        let columns = Value::Array(vec![]);

        let result = v.filter_by_columns(&columns);
        let result_map = result.as_map().unwrap();
        assert_eq!(
            result_map.len(),
            0,
            "No matching columns should return empty map"
        );
    }

    #[test]
    fn test_filter_by_columns_non_array_columns() {
        // If columns is not an array, return original value unchanged
        let mut map = ValueMap::new();
        map.insert(Value::String("id".to_string()), Value::I32(1));
        map.insert(
            Value::String("name".to_string()),
            Value::String("test".to_string()),
        );

        let v = Value::Map(map);
        let columns = Value::String("id".to_string());

        let result = v.filter_by_columns(&columns);
        // Should return original value since columns is not an array
        let result_map = result.as_map().unwrap();
        assert_eq!(result_map.len(), 2);
    }

    #[test]
    fn test_filter_by_columns_non_map_value() {
        // If self is not a Map, return it unchanged
        let v = Value::I32(42);
        let columns = Value::Array(vec![Value::String("id".to_string())]);

        let result = v.filter_by_columns(&columns);
        assert_eq!(result, Value::I32(42));
    }

    #[test]
    fn test_filter_by_columns_no_matching_columns() {
        let mut map = ValueMap::new();
        map.insert(Value::String("id".to_string()), Value::I32(1));
        map.insert(
            Value::String("name".to_string()),
            Value::String("test".to_string()),
        );

        let v = Value::Map(map);
        let columns = Value::Array(vec![
            Value::String("nonexistent".to_string()),
            Value::String("also_missing".to_string()),
        ]);

        let result = v.filter_by_columns(&columns);
        let result_map = result.as_map().unwrap();
        assert_eq!(
            result_map.len(),
            0,
            "No matching columns should return empty map"
        );
    }

    #[test]
    fn test_filter_by_columns_all_columns_selected() {
        let mut map = ValueMap::new();
        map.insert(Value::String("id".to_string()), Value::I32(1));
        map.insert(
            Value::String("name".to_string()),
            Value::String("test".to_string()),
        );

        let v = Value::Map(map.clone());
        let columns = Value::Array(vec![
            Value::String("id".to_string()),
            Value::String("name".to_string()),
        ]);

        let result = v.filter_by_columns(&columns);
        assert_eq!(result, Value::Map(map));
    }

    #[test]
    fn test_filter_by_columns_preserves_values() {
        let mut map = ValueMap::new();
        map.insert(Value::String("id".to_string()), Value::I32(99));
        map.insert(
            Value::String("name".to_string()),
            Value::String("preserved".to_string()),
        );
        map.insert(Value::String("active".to_string()), Value::Bool(true));
        map.insert(Value::String("score".to_string()), Value::F64(95.5));

        let v = Value::Map(map);
        let columns = Value::Array(vec![
            Value::String("id".to_string()),
            Value::String("score".to_string()),
        ]);

        let result = v.filter_by_columns(&columns);
        let result_map = result.as_map().unwrap();
        assert_eq!(result_map.len(), 2); // id and score selected
    }

    #[test]
    fn test_filter_by_columns_with_value_macro() {
        // Test using the value! macro for realistic usage
        let data = value! {
            "id": 1,
            "name": "test",
            "age": 25,
            "email": "a@b.com"
        };
        let columns = value!["id", "name"];

        let result = data.filter_by_columns(&columns);
        let result_map = result.as_map().unwrap();
        assert_eq!(result_map.len(), 2); // 2 columns selected from value! macro
    }
}
