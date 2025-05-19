use rbatis_codegen::codegen::parser_html::parse_html;

#[test]
fn test_parse_html_simple() {
    let html = "<mapper><select id=\"test\">select * from user</select></mapper>";
    let fn_name = "test";
    let tokens = parse_html(html, fn_name, &mut vec![]);
    let result = tokens.to_string();
    assert!(result.contains("select * from user"));
}

#[test]
fn test_parse_html_with_if() {
    let html = "<mapper><select id=\"test\">select * from user <if test=\"name != null\">where name = #{name}</if></select></mapper>";
    let fn_name = "test";
    let tokens = parse_html(html, fn_name, &mut vec![]);
    let result = tokens.to_string();
    println!("{:?}", result);
    assert!(result.contains("if"));
    assert!(result.contains("arg [\"name\"]) . op_ne (& rbs :: Value :: Null)"));
}

#[test]
fn test_parse_html_with_foreach() {
    let html = "<mapper><select id=\"test\">select * from user <foreach collection=\"ids\" item=\"item\" index=\"index\">where id = #{item}</foreach></select></mapper>";
    let fn_name = "test";
    let tokens = parse_html(html, fn_name, &mut vec![]);
    let result = tokens.to_string();
    assert!(result.contains("for"));
    assert!(result.contains("ids"));
}

#[test]
fn test_parse_html_with_choose() {
    let html = "<mapper><select id=\"test\">select * from user <choose><when test=\"id != null\">where id = #{id}</when><otherwise>where id = 0</otherwise></choose></select></mapper>";
    let fn_name = "test";
    let tokens = parse_html(html, fn_name, &mut vec![]);
    let result = tokens.to_string();
    println!("{}", result);
    assert!(result.contains("if"));
}

#[test]
fn test_parse_html_with_trim() {
    let html = "<mapper><select id=\"test\">select * from user <trim prefixOverrides=\"and\" suffixOverrides=\",\">where id = #{id}</trim></select></mapper>";
    let fn_name = "test";
    let tokens = parse_html(html, fn_name, &mut vec![]);
    let result = tokens.to_string();
    assert!(result.contains("trim"));
}

#[test]
fn test_parse_html_with_bind() {
    let html = "<mapper><select id=\"test\"><bind name=\"pattern\" value=\"'%' + name + '%'\"/>select * from user where name like #{pattern}</select></mapper>";
    let fn_name = "test";
    let tokens = parse_html(html, fn_name, &mut vec![]);
    let result = tokens.to_string();
    print!("{}", result);
    assert!(result.contains("pattern"));
}

#[test]
fn test_parse_html_with_where() {
    let html = "<mapper><select id=\"test\">select * from user <where>id = #{id}</where></select></mapper>";
    let fn_name = "test";
    let tokens = parse_html(html, fn_name, &mut vec![]);
    let result = tokens.to_string();
    assert!(result.contains("where"));
}

#[test]
fn test_parse_html_with_set() {
    let html = "<mapper><update id=\"test\">update user <set>name = #{name}</set> where id = #{id}</update></mapper>";
    let fn_name = "test";
    let tokens = parse_html(html, fn_name, &mut vec![]);
    let result = tokens.to_string();
    assert!(result.contains("set"));
} 