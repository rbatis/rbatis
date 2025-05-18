#[cfg(test)]
mod tests {
    use rbs::{to_value, Value};
    use rbs::value::map::ValueMap;
    
    #[test]
    fn test_to_value_basic_literals() {
        assert_eq!(to_value!(Option::<i32>::None), Value::Null);
        assert_eq!(to_value!(true), Value::Bool(true));
        assert_eq!(to_value!(false), Value::Bool(false));
        assert_eq!(to_value!(123), Value::I32(123));
        assert_eq!(to_value!(-123), Value::I32(-123));
        assert_eq!(to_value!(123i64), Value::I64(123));
        assert_eq!(to_value!(123u32), Value::U32(123));
        assert_eq!(to_value!(123u64), Value::U64(123));
        assert_eq!(to_value!(1.23f32), Value::F32(1.23));
        assert_eq!(to_value!(1.23f64), Value::F64(1.23));
        assert_eq!(to_value!("hello"), Value::String("hello".to_string()));
        
        let s = "world".to_string();
        assert_eq!(to_value!(s.clone()), Value::String("world".to_string())); // Test with variable

        let n = 42;
        assert_eq!(to_value!(n), Value::I32(42));
    }


    #[test]
    fn test_to_value_vec_i32() {
        let bytes_vec: Vec<i32> = vec![4, 5, 6];
        assert_eq!(to_value!(bytes_vec), to_value![4, 5, 6]);
    }

    #[test]
    fn test_to_value_basic_use() {
        let v = rbs::to_value! {
            "id": 1,
            "user": {
                "name": "Alice"
            }
        };
        assert_eq!(to_value!(v).to_string(), r#"{"id":1,"user":{"name":"Alice"}}"#);
    }
    

    #[test]
    fn test_to_value_simple_map_implicit_braces() {
        // This form is shown in documentation: to_value! { "key": "value" }
        // It seems to be handled by the ($($k:tt: $v:expr),* $(,)?) arm
        let val = to_value! {
            "name": "Alice",
            "age": 30,
            "city": "New York"
        };

        let mut expected_map = ValueMap::new();
        expected_map.insert(Value::String("name".to_string()), Value::String("Alice".to_string()));
        expected_map.insert(Value::String("age".to_string()), Value::I32(30));
        expected_map.insert(Value::String("city".to_string()), Value::String("New York".to_string()));

        assert_eq!(val, Value::Map(expected_map));
    }

    #[test]
    fn test_to_value_simple_map_explicit_braces_in_parens() {
        // This form to_value!({ "key": "value" })
        // It matches the ({$($k:tt: $v:tt),* $(,)*}) arm
        let val = to_value!({
            "name": "Bob",
            "age": 25i64, // Use i64 for variety
            "active": true
        });

        let mut expected_map = ValueMap::new();
        expected_map.insert(Value::String("name".to_string()), Value::String("Bob".to_string()));
        expected_map.insert(Value::String("age".to_string()), Value::I64(25)); 
        expected_map.insert(Value::String("active".to_string()), Value::Bool(true));

        assert_eq!(val, Value::Map(expected_map));
    }

    #[test]
    fn test_to_value_simple_map_direct_kv_in_parens() {
        // This form to_value!(key: value, key2: value2)
        // It matches the ($($k:tt: $v:expr),* $(,)?) arm
        let name_val = "Charlie";
        let age_val = 40u32;
        let val = to_value!(
            "name": name_val,
            "age": age_val,
            "verified": false
        );

        let mut expected_map = ValueMap::new();
        expected_map.insert(Value::String("name".to_string()), Value::String("Charlie".to_string()));
        expected_map.insert(Value::String("age".to_string()), Value::U32(age_val));
        expected_map.insert(Value::String("verified".to_string()), Value::Bool(false));

        assert_eq!(val, Value::Map(expected_map));
    }

    #[test]
    fn test_to_value_map_with_trailing_comma() {
        let val = to_value! {
            "key1": "value1",
            "key2": 123,
        };
        let mut expected_map = ValueMap::new();
        expected_map.insert(Value::String("key1".to_string()), Value::String("value1".to_string()));
        expected_map.insert(Value::String("key2".to_string()), Value::I32(123));
        assert_eq!(val, Value::Map(expected_map));

        let val2 = to_value!({
            "a": 1.0f32,
            "b": true,
        });
        let mut expected_map2 = ValueMap::new();
        expected_map2.insert(Value::String("a".to_string()), Value::F32(1.0));
        expected_map2.insert(Value::String("b".to_string()), Value::Bool(true));
        assert_eq!(val2, Value::Map(expected_map2));
    }

    #[test]
    fn test_to_value_empty_map() {
        let val_implicit_braces = to_value!{}; // Should use the ($($k:tt: $v:expr),*) arm with zero repetitions
        let expected_empty_map = Value::Map(ValueMap::new());
        assert_eq!(val_implicit_braces, expected_empty_map);

        let val_explicit_braces = to_value!({}); // Should use the ({$($k:tt: $v:tt),*}) arm with zero repetitions
        assert_eq!(val_explicit_braces, expected_empty_map);
        
        // to_value!() is ambiguous and might call the ($arg:expr) arm with an empty tuple if not careful,
        // but given the macro rules, it's more likely to be a compile error or match the map rule.
        // If it matches `($($k:tt: $v:expr),* $(,)?)` with nothing, it should produce an empty map.
        // Let's test `to_value!()` specifically if it compiles.
        // It seems to_value!() by itself leads to compile error `unexpected end of macro invocation`
        // So we only test to_value!{} and to_value!({}).
    }

    #[test]
    fn test_to_value_nested_map_implicit_braces() {
        let val = to_value! {
            "id": 1,
            "user": to_value!{
                "name": "Alice",
                "details": to_value!{
                    "verified": true,
                    "score": 100u64
                }
            },
            "product": to_value!{
                "id": "P123",
                "price": 99.99f32
            }
        };

        let mut user_details_map = ValueMap::new();
        user_details_map.insert(Value::String("verified".to_string()), Value::Bool(true));
        user_details_map.insert(Value::String("score".to_string()), Value::U64(100));

        let mut user_map = ValueMap::new();
        user_map.insert(Value::String("name".to_string()), Value::String("Alice".to_string()));
        user_map.insert(Value::String("details".to_string()), Value::Map(user_details_map));

        let mut product_map = ValueMap::new();
        product_map.insert(Value::String("id".to_string()), Value::String("P123".to_string()));
        product_map.insert(Value::String("price".to_string()), Value::F32(99.99));

        let mut expected_map = ValueMap::new();
        expected_map.insert(Value::String("id".to_string()), Value::I32(1));
        expected_map.insert(Value::String("user".to_string()), Value::Map(user_map));
        expected_map.insert(Value::String("product".to_string()), Value::Map(product_map));

        assert_eq!(val, Value::Map(expected_map));
    }

    #[test]
    fn test_to_value_nested_map_explicit_braces_in_parens() {
         let val = to_value!{
            "level1_key": "level1_val",
            "nested": to_value!{
                "level2_key": 123,
                "deeper_nested": to_value!{
                    "level3_key": true
                }
            }
        };

        let mut deeper_nested_map = ValueMap::new();
        deeper_nested_map.insert(Value::String("level3_key".to_string()), Value::Bool(true));

        let mut nested_map = ValueMap::new();
        nested_map.insert(Value::String("level2_key".to_string()), Value::I32(123));
        nested_map.insert(Value::String("deeper_nested".to_string()), Value::Map(deeper_nested_map));

        let mut expected_map = ValueMap::new();
        expected_map.insert(Value::String("level1_key".to_string()), Value::String("level1_val".to_string()));
        expected_map.insert(Value::String("nested".to_string()), Value::Map(nested_map));

        assert_eq!(val, Value::Map(expected_map));
    }

    #[test]
    fn test_nested_map_from_documentation_example() {
        // Example from the macro documentation
        let val = to_value! {
            "id": 1, 
            "user": to_value!{
                "name": "Alice",
                "address": to_value!{
                    "city": "Beijing",
                    "street": to_value!{
                        "number": 123
                    }
                }
            }
        };

        let mut street_map = ValueMap::new();
        street_map.insert(Value::String("number".to_string()), Value::I32(123));

        let mut address_map = ValueMap::new();
        address_map.insert(Value::String("city".to_string()), Value::String("Beijing".to_string()));
        address_map.insert(Value::String("street".to_string()), Value::Map(street_map));

        let mut user_map = ValueMap::new();
        user_map.insert(Value::String("name".to_string()), Value::String("Alice".to_string()));
        user_map.insert(Value::String("address".to_string()), Value::Map(address_map));

        let mut expected_map = ValueMap::new();
        expected_map.insert(Value::String("id".to_string()), Value::I32(1));
        expected_map.insert(Value::String("user".to_string()), Value::Map(user_map));

        assert_eq!(val, Value::Map(expected_map));
    }

    #[test]
    fn test_to_value_map_with_array_value() {
        let arr_val = Value::Array(vec![Value::I32(1), Value::String("two".to_string())]);
        let val = to_value! {
            "data": arr_val.clone(), // Use an existing Value::Array
            "id": 123
        };

        let mut expected_map = ValueMap::new();
        expected_map.insert(Value::String("id".to_string()), Value::I32(123));
        expected_map.insert(Value::String("data".to_string()), arr_val);
        
        assert_eq!(val, Value::Map(expected_map));

        // Test with an expression that evaluates to a serializable vec
        let my_vec = vec![true, false];
        let val2 = to_value! {
            "flags": my_vec.clone() // my_vec will be passed to to_value(my_vec)
        };
        let mut expected_map2 = ValueMap::new();
        // to_value(vec![true, false]) will create Value::Array(vec![Value::Bool(true), Value::Bool(false)])
        let expected_arr_val = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
        expected_map2.insert(Value::String("flags".to_string()), expected_arr_val);
        assert_eq!(val2, Value::Map(expected_map2));
    }

    #[test]
    fn test_to_value_map_with_non_string_literal_keys() {
        let key_name_str = "my_key";
        // Test with implicit braces form: to_value! { key: value }
        let val = to_value! {
            key_name_str: "value_for_ident_key", // key_name_str (a variable) will be to_value!(key_name_str)
            123: "value_for_numeric_key",      // 123 (a literal) will be to_value!(123)
            "string_lit_key": key_name_str // ensure string literal key also works with var value
        };

        let mut expected_map = ValueMap::new();
        // to_value!(key_name_str) -> Value::String("my_key")
        expected_map.insert(Value::String(key_name_str.to_string()), Value::String("value_for_ident_key".to_string()));
        // to_value!(123) -> Value::I32(123)
        expected_map.insert(Value::I32(123), Value::String("value_for_numeric_key".to_string()));
        expected_map.insert(Value::String("string_lit_key".to_string()), Value::String(key_name_str.to_string()));
        
        assert_eq!(val, Value::Map(expected_map));

        // Test with the explicit braces in parens form: to_value!({ key: value })
        let key_name_str_2 = "my_key_2"; // use a different variable to avoid shadowing issues if any confusion
        let val2 = to_value!({
            key_name_str_2: true,
            456u32: 1.23f64, // Using u32 for key type variety
            "another_lit_key": false
        });
        let mut expected_map2 = ValueMap::new();
        expected_map2.insert(Value::String(key_name_str_2.to_string()), Value::Bool(true));
        // to_value!(456u32) -> Value::U32(456)
        expected_map2.insert(Value::U32(456), Value::F64(1.23)); 
        expected_map2.insert(Value::String("another_lit_key".to_string()), Value::Bool(false));
        assert_eq!(val2, Value::Map(expected_map2));
    }

    #[test]
    fn test_to_value_special_nested_arm_direct_match() {
        // This should match {$($k:tt: {$($ik:tt: $iv:tt),* $(,)*}),* $(,)*}} rule directly
        // Syntax: to_value! { outer_key1: { ik1: iv1 }, outer_key2: { ik2: iv2 } }
        let val = to_value! {
            "user_profile": { // Inner part is a brace-enclosed map
                "name": "Eve",
                "level": 5
            }, // Comma separating top-level entries
            "settings": { // Inner part is a brace-enclosed map
                "theme": "dark",
                "notifications": true
            } // No trailing comma for the last top-level entry, should be fine
        };
        
        let mut user_profile_map = ValueMap::new();
        // Inside this arm, keys and values are recursively passed to to_value!
        // For the value of "user_profile", `to_value!({ "name": "Eve", "level": 5 })` will be called.
        user_profile_map.insert(to_value!("name"), to_value!("Eve"));
        user_profile_map.insert(to_value!("level"), to_value!(5));

        let mut settings_map = ValueMap::new();
        // For "settings", `to_value!({ "theme": "dark", "notifications": true })` will be called.
        settings_map.insert(to_value!("theme"), to_value!("dark"));
        settings_map.insert(to_value!("notifications"), to_value!(true));

        let mut expected_map = ValueMap::new();
        expected_map.insert(to_value!("user_profile"), Value::Map(user_profile_map));
        expected_map.insert(to_value!("settings"), Value::Map(settings_map));

        assert_eq!(val, Value::Map(expected_map));

        // Single top-level entry matching this arm
        let val_single = to_value! {
            "data_points": {
                "point_x": 10.5f32,
                "point_y": 20.0f32, // trailing comma in inner map
                "label": "Sample"
            } 
        };
        let mut data_points_map = ValueMap::new();
        data_points_map.insert(to_value!("point_x"), Value::F32(10.5));
        data_points_map.insert(to_value!("point_y"), Value::F32(20.0));
        data_points_map.insert(to_value!("label"), to_value!("Sample"));
        
        let mut expected_single = ValueMap::new();
        expected_single.insert(to_value!("data_points"), Value::Map(data_points_map));
        assert_eq!(val_single, Value::Map(expected_single));

        // Test this arm with an empty inner map for one of the keys
        let val_empty_inner = to_value! {
            "config": {
                "retries": 3
            },
            "empty_section": {} // Empty inner map
        };
        
        let mut config_map = ValueMap::new();
        config_map.insert(to_value!("retries"), to_value!(3));
        
        // The inner call for "empty_section" will be to_value!({})
        let empty_inner_map = ValueMap::new(); 

        let mut expected_empty_inner = ValueMap::new();
        expected_empty_inner.insert(to_value!("config"), Value::Map(config_map));
        expected_empty_inner.insert(to_value!("empty_section"), Value::Map(empty_inner_map)); // This becomes Value::Map(ValueMap {})
        assert_eq!(val_empty_inner, Value::Map(expected_empty_inner));
    }

    #[test]
    fn test_to_value_nested_call_syntax() {
        // 测试不同形式的嵌套 to_value! 调用语法
        
        // 形式1：内部使用 to_value!{...}（推荐用于嵌套调用）
        let val1 = to_value! {
            "nested": to_value!{
                "foo": "bar"
            }
        };
        
        // 形式2：内部使用 to_value!(...)（等价于形式1）
        let val2 = to_value! {
            "nested": to_value!(
                "foo": "bar"
            )
        };
        
        // 两种形式应该产生相同的结果
        let mut inner_map = ValueMap::new();
        inner_map.insert(Value::String("foo".to_string()), Value::String("bar".to_string()));
        
        let mut expected_map = ValueMap::new();
        expected_map.insert(Value::String("nested".to_string()), Value::Map(inner_map));
        
        assert_eq!(val1, Value::Map(expected_map.clone()));
        assert_eq!(val2, Value::Map(expected_map));
        assert_eq!(val1, val2);
        
        // 注意：形式3 to_value!({...}) 在嵌套时可能导致 linter 错误
        // 但实际编译和运行应该也是正确的
    }
} 