use rbatis_codegen::codegen::string_util::{find_convert_string, count_string_num, un_packing_string, concat_str};

#[test]
fn test_find_convert_string() {
    // Test with #{param}
    let result = find_convert_string("SELECT * FROM users WHERE id = #{id}");
    assert_eq!(result.len(), 1);
    let (key, value) = result.front().unwrap();
    assert_eq!(key, "id");
    assert_eq!(value, "#{id}");

    // Test with ${param}
    let result = find_convert_string("SELECT * FROM ${table} WHERE id = ${id}");
    assert_eq!(result.len(), 2);
    
    // Test with mixed #{} and ${}
    let result = find_convert_string("SELECT * FROM ${table} WHERE name = #{name} AND id = #{id}");
    assert_eq!(result.len(), 3);

    // Test with no convert strings
    let result = find_convert_string("SELECT * FROM users");
    assert_eq!(result.len(), 0);
    
    // Test with incomplete convert strings
    let result = find_convert_string("SELECT * FROM #{users WHERE id = #{id}");
    assert_eq!(result.len(), 1);
}

#[test]
fn test_count_string_num() {
    let s = "hello, world, hello";
    assert_eq!(count_string_num(&s.to_string(), 'h'), 2);
    assert_eq!(count_string_num(&s.to_string(), 'l'), 5);
    assert_eq!(count_string_num(&s.to_string(), 'o'), 3);
    assert_eq!(count_string_num(&s.to_string(), ','), 2);
    assert_eq!(count_string_num(&s.to_string(), 'x'), 0);
    
    // Test with empty string
    let s = "";
    assert_eq!(count_string_num(&s.to_string(), 'a'), 0);
}

#[test]
fn test_un_packing_string() {
    // Test with single quotes
    assert_eq!(un_packing_string("'hello'"), "hello");
    
    // Test with double quotes
    assert_eq!(un_packing_string("\"world\""), "world");
    
    // Test with backticks
    assert_eq!(un_packing_string("`table_name`"), "table_name");
    
    // Test with no quotes
    assert_eq!(un_packing_string("column"), "column");
    
    // Test with partially quoted strings
    assert_eq!(un_packing_string("'hello"), "'hello");
    assert_eq!(un_packing_string("world\""), "world\"");
    
    // Test with empty string
    assert_eq!(un_packing_string(""), "");
    
    // Test with single character
    assert_eq!(un_packing_string("a"), "a");
}

#[test]
fn test_concat_str() {
    // Test with empty base string
    let mut text = String::new();
    concat_str(&mut text, "hello");
    assert_eq!(text, "hello");
    
    // Test with non-empty string
    let mut text = "SELECT".to_string();
    concat_str(&mut text, "*");
    assert_eq!(text, "SELECT *");
    
    // Test with existing space
    let mut text = "SELECT ".to_string();
    concat_str(&mut text, "*");
    assert_eq!(text, "SELECT *");
    
    // Test with append starting with space
    let mut text = "SELECT".to_string();
    concat_str(&mut text, " *");
    assert_eq!(text, "SELECT *");
    
    // Test with comma
    let mut text = "id,".to_string();
    concat_str(&mut text, "name");
    assert_eq!(text, "id, name");
    
    // Test with parentheses
    let mut text = "WHERE (".to_string();
    concat_str(&mut text, "1=1");
    assert_eq!(text, "WHERE ( 1=1");
}