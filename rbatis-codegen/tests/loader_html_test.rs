use rbatis_codegen::codegen::loader_html::{load_html, Element};
use std::collections::HashMap;

#[test]
fn test_load_html_simple() {
    let html = r#"<select id="find_user">SELECT * FROM users WHERE id = #{id}</select>"#;
    let result = load_html(html).expect("Failed to load HTML");
    assert_eq!(result.len(), 1);
    
    let element = &result[0];
    assert_eq!(element.tag, "select");
    assert_eq!(element.attrs.get("id").unwrap(), "find_user");
    
    // Text content is stored as a child node, not in the data field
    assert_eq!(element.childs.len(), 1);
    assert_eq!(element.childs[0].data, "SELECT * FROM users WHERE id = #{id}");
}

#[test]
fn test_load_html_with_children() {
    let html = r#"
    <select id="find_user">
        SELECT * FROM users
        <if test="name != null">
            WHERE name = #{name}
        </if>
    </select>
    "#;
    
    let result = load_html(html).expect("Failed to load HTML");
    assert_eq!(result.len(), 1);
    
    let select_element = &result[0];
    assert_eq!(select_element.tag, "select");
    assert_eq!(select_element.attrs.get("id").unwrap(), "find_user");
    assert_eq!(select_element.childs.len(), 2);
    
    let text_node = &select_element.childs[0];
    assert_eq!(text_node.tag, "");
    assert!(text_node.data.contains("SELECT * FROM users"));
    
    let if_element = &select_element.childs[1];
    assert_eq!(if_element.tag, "if");
    assert_eq!(if_element.attrs.get("test").unwrap(), "name != null");
    assert_eq!(if_element.childs.len(), 1);
    assert!(if_element.childs[0].data.contains("WHERE name = #{name}"));
}

#[test]
fn test_load_html_multiple_elements() {
    let html = r#"
    <select id="find_user">SELECT * FROM users WHERE id = #{id}</select>
    <insert id="create_user">INSERT INTO users(name) VALUES(#{name})</insert>
    "#;
    
    let result = load_html(html).expect("Failed to load HTML");
    assert_eq!(result.len(), 2);
    
    let select_element = &result[0];
    assert_eq!(select_element.tag, "select");
    assert_eq!(select_element.attrs.get("id").unwrap(), "find_user");
    
    let insert_element = &result[1];
    assert_eq!(insert_element.tag, "insert");
    assert_eq!(insert_element.attrs.get("id").unwrap(), "create_user");
}

#[test]
fn test_element_display() {
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "test_id".to_string());
    attrs.insert("test".to_string(), "condition".to_string());
    
    // For display, the content is stored in a child element if the tag is not empty
    let element = Element {
        tag: "select".to_string(),
        data: "SELECT * FROM users".to_string(),
        attrs,
        childs: vec![],
    };
    
    let display_str = format!("{}", element);
    assert!(display_str.contains("<select"));
    assert!(display_str.contains("id=\"test_id\""));
    assert!(display_str.contains("test=\"condition\""));
    assert!(display_str.contains("</select>"));
    
    // Since data is not displayed for non-empty tags, we'll create a text element instead
    let text_element = Element {
        tag: "".to_string(),
        data: "SELECT * FROM users".to_string(),
        attrs: HashMap::new(),
        childs: vec![],
    };
    
    let text_display = format!("{}", text_element);
    assert!(text_display.contains("SELECT * FROM users"));
}

#[test]
fn test_element_with_children_display() {
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "test_id".to_string());
    
    let child_element = Element {
        tag: "".to_string(),
        data: "WHERE name = 'test'".to_string(),
        attrs: HashMap::new(),
        childs: vec![],
    };
    
    let element = Element {
        tag: "select".to_string(),
        data: "SELECT * FROM users".to_string(),
        attrs,
        childs: vec![child_element],
    };
    
    let display_str = format!("{}", element);
    assert!(display_str.contains("<select"));
    assert!(display_str.contains("id=\"test_id\""));
    assert!(display_str.contains("WHERE name = 'test'"));
    assert!(display_str.contains("</select>"));
}

#[test]
fn test_element_with_nested_children() {
    let html = r#"
    <select id="find_user">
        SELECT * FROM users
        <if test="name != null">
            <if test="name != ''">
                WHERE name = #{name}
            </if>
        </if>
    </select>
    "#;
    
    let result = load_html(html).expect("Failed to load HTML");
    assert_eq!(result.len(), 1);
    
    let select_element = &result[0];
    // There may be more child elements due to whitespace text nodes
    assert!(select_element.childs.len() >= 2);
    
    // Find the first if element (skip text nodes)
    let if_element = select_element.childs.iter()
        .find(|e| e.tag == "if")
        .expect("Should find an if element");
    
    // The if element should have a nested if element
    let nested_if_element = if_element.childs.iter()
        .find(|e| e.tag == "if")
        .expect("Should find a nested if element");
    
    assert_eq!(nested_if_element.tag, "if");
    assert!(nested_if_element.childs.len() >= 1);
    assert!(nested_if_element.childs.iter()
        .any(|c| c.data.contains("WHERE name = #{name}")));
}