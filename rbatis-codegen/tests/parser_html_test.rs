use rbatis_codegen::codegen::parser_html::{load_mapper_map, load_mapper_vec};

#[test]
fn test_load_mapper_map_basic() {
    let html = r#"<select id="find_user">SELECT * FROM users WHERE id = #{id}</select>"#;
    let result = load_mapper_map(html);
    assert!(result.is_ok());
    
    let map = result.unwrap();
    assert_eq!(map.len(), 1);
    assert!(map.contains_key("find_user"));
    
    let element = map.get("find_user").unwrap();
    assert_eq!(element.tag, "select");
    assert_eq!(element.attrs.get("id").unwrap(), "find_user");
}

#[test]
fn test_load_mapper_map_multiple() {
    let html = r#"
    <mapper>
        <select id="find_user">SELECT * FROM users WHERE id = #{id}</select>
        <insert id="create_user">INSERT INTO users(name) VALUES(#{name})</insert>
    </mapper>
    "#;
    let result = load_mapper_map(html);
    assert!(result.is_ok());
    
    let map = result.unwrap();
    assert_eq!(map.len(), 2);
    assert!(map.contains_key("find_user"));
    assert!(map.contains_key("create_user"));
}

#[test]
fn test_load_mapper_vec_basic() {
    let html = r#"<select id="find_user">SELECT * FROM users WHERE id = #{id}</select>"#;
    let result = load_mapper_vec(html);
    assert!(result.is_ok());
    
    let elements = result.unwrap();
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "select");
    assert_eq!(elements[0].attrs.get("id").unwrap(), "find_user");
}

#[test]
fn test_load_mapper_vec_with_wrapper() {
    let html = r#"
    <mapper>
        <select id="find_user">SELECT * FROM users WHERE id = #{id}</select>
    </mapper>
    "#;
    let result = load_mapper_vec(html);
    assert!(result.is_ok());
    
    let elements = result.unwrap();
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "select");
    assert_eq!(elements[0].attrs.get("id").unwrap(), "find_user");
}

#[test]
fn test_load_mapper_vec_no_wrapper() {
    let html = r#"<select id="find_user">SELECT * FROM users WHERE id = #{id}</select>"#;
    let result = load_mapper_vec(html);
    assert!(result.is_ok());
    
    let elements = result.unwrap();
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "select");
    assert_eq!(elements[0].attrs.get("id").unwrap(), "find_user");
}

#[test]
fn test_load_mapper_vec_no_id() {
    let html = r#"<select>SELECT * FROM users</select>"#;
    let result = load_mapper_vec(html);
    assert!(result.is_ok());
    
    let elements = result.unwrap();
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "select");
    assert_eq!(elements[0].attrs.get("id"), None);
}