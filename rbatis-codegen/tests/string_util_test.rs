use rbatis_codegen::codegen::string_util::{find_convert_string, count_string_num, un_packing_string};

#[test]
fn test_find_convert_string() {
    let sql = "select * from user where id = #{id} and name = ${name}";
    let list = find_convert_string(sql);
    
    assert_eq!(list.len(), 2);
    
    let items: Vec<(String, String)> = list.into_iter().collect();
    assert_eq!(items[0].0, "id");
    assert_eq!(items[0].1, "#{id}");
    assert_eq!(items[1].0, "name");
    assert_eq!(items[1].1, "${name}");
    
    // 测试嵌套表达式
    let sql = "select * from user where id = #{id} and name like #{prefix}%";
    let list = find_convert_string(sql);
    
    assert_eq!(list.len(), 2);
    
    let items: Vec<(String, String)> = list.into_iter().collect();
    assert_eq!(items[0].0, "id");
    assert_eq!(items[0].1, "#{id}");
    assert_eq!(items[1].0, "prefix");
    assert_eq!(items[1].1, "#{prefix}");
    
    // 测试空字符串
    let sql = "";
    let list = find_convert_string(sql);
    assert_eq!(list.len(), 0);
}

#[test]
fn test_count_string_num() {
    let s = "hello".to_string();
    assert_eq!(count_string_num(&s, 'l'), 2);
    
    let s = "".to_string();
    assert_eq!(count_string_num(&s, 'a'), 0);
    
    let s = "aaa".to_string();
    assert_eq!(count_string_num(&s, 'a'), 3);
    
    let s = "abcabc".to_string();
    assert_eq!(count_string_num(&s, 'a'), 2);
    assert_eq!(count_string_num(&s, 'b'), 2);
    assert_eq!(count_string_num(&s, 'c'), 2);
    assert_eq!(count_string_num(&s, 'd'), 0);
}

#[test]
fn test_un_packing_string() {
    assert_eq!(un_packing_string("'test'"), "test");
    assert_eq!(un_packing_string("`test`"), "test");
    assert_eq!(un_packing_string("\"test\""), "test");
    
    // 测试不完整的引号不会被去除
    assert_eq!(un_packing_string("'test"), "'test");
    assert_eq!(un_packing_string("test'"), "test'");
    
    // 测试没有引号的字符串
    assert_eq!(un_packing_string("test"), "test");
    
    // 测试空字符串
    assert_eq!(un_packing_string(""), "");
    
    // 测试只有一个字符的字符串
    assert_eq!(un_packing_string("a"), "a");
    
    // 测试有引号但长度小于2的字符串
    assert_eq!(un_packing_string("'"), "'");
} 