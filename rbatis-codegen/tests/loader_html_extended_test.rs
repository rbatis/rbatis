use rbatis_codegen::codegen::loader_html::{load_html, Element};

#[test]
fn test_load_html_with_empty_string() {
    let html = "";
    let result = load_html(html);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_load_html_with_text_only() {
    let html = "SELECT * FROM users";
    let result = load_html(html);
    assert!(result.is_ok());
    
    let elements = result.unwrap();
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "");
    assert_eq!(elements[0].data, "SELECT * FROM users");
}

#[test]
fn test_load_html_with_nested_elements() {
    let html = r#"<select><if>test</if></select>"#;
    let result = load_html(html);
    assert!(result.is_ok());
    
    let elements = result.unwrap();
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "select");
    assert_eq!(elements[0].childs.len(), 1);
    assert_eq!(elements[0].childs[0].tag, "if");
    assert_eq!(elements[0].childs[0].childs[0].data, "test");
}

#[test]
fn test_load_html_with_attributes() {
    let html = r#"<select id="find_user" test="true">content</select>"#;
    let result = load_html(html);
    assert!(result.is_ok());
    
    let elements = result.unwrap();
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "select");
    assert_eq!(elements[0].attrs.get("id").unwrap(), "find_user");
    assert_eq!(elements[0].attrs.get("test").unwrap(), "true");
    assert_eq!(elements[0].childs[0].data, "content");
}

#[test]
fn test_load_html_with_break_tag() {
    let html = r#"<select><break>content</break></select>"#;
    let result = load_html(html);
    assert!(result.is_ok());
    
    let elements = result.unwrap();
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "select");
    assert_eq!(elements[0].childs.len(), 1);
    assert_eq!(elements[0].childs[0].tag, "break"); // Should be converted from "bk"
    assert_eq!(elements[0].childs[0].childs[0].data, "content");
}

#[test]
fn test_load_html_with_mixed_content() {
    let html = r#"text<select id="test">content</select>more text"#;
    let result = load_html(html);
    assert!(result.is_ok());
    
    let elements = result.unwrap();
    assert_eq!(elements.len(), 3);
    
    // First text element
    assert_eq!(elements[0].tag, "");
    assert_eq!(elements[0].data, "text");
    
    // Select element
    assert_eq!(elements[1].tag, "select");
    assert_eq!(elements[1].attrs.get("id").unwrap(), "test");
    
    // Second text element
    assert_eq!(elements[2].tag, "");
    assert_eq!(elements[2].data, "more text");
}

#[test]
fn test_element_child_strings() {
    let child1 = Element {
        tag: "".to_string(),
        data: "child1".to_string(),
        attrs: std::collections::HashMap::new(),
        childs: vec![],
    };
    
    let child2 = Element {
        tag: "span".to_string(),
        data: "".to_string(),
        attrs: std::collections::HashMap::new(),
        childs: vec![],
    };
    
    let grandchild = Element {
        tag: "".to_string(),
        data: "grandchild".to_string(),
        attrs: std::collections::HashMap::new(),
        childs: vec![],
    };
    
    let mut child2_with_child = child2.clone();
    child2_with_child.childs.push(grandchild);
    
    let parent = Element {
        tag: "div".to_string(),
        data: "".to_string(),
        attrs: std::collections::HashMap::new(),
        childs: vec![child1, child2_with_child],
    };
    
    let strings = parent.child_strings();
    assert_eq!(strings.len(), 2); // "child1", "grandchild"
    assert!(strings.contains(&"child1"));
    assert!(strings.contains(&"grandchild"));
}

#[test]
fn test_element_child_string_cup() {
    let child1 = Element {
        tag: "".to_string(),
        data: "hello".to_string(),
        attrs: std::collections::HashMap::new(),
        childs: vec![],
    };
    
    let child2 = Element {
        tag: "".to_string(),
        data: "world".to_string(),
        attrs: std::collections::HashMap::new(),
        childs: vec![],
    };
    
    let parent = Element {
        tag: "div".to_string(),
        data: "".to_string(),
        attrs: std::collections::HashMap::new(),
        childs: vec![child1, child2],
    };
    
    let cup = parent.child_string_cup();
    assert_eq!(cup, 10); // "hello".len() + "world".len() = 5 + 5 = 10
}

#[test]
fn test_element_display_complex() {
    let child = Element {
        tag: "".to_string(),
        data: "child content".to_string(),
        attrs: std::collections::HashMap::new(),
        childs: vec![],
    };
    
    let mut attrs = std::collections::HashMap::new();
    attrs.insert("id".to_string(), "test".to_string());
    attrs.insert("class".to_string(), "container".to_string());
    
    let parent = Element {
        tag: "div".to_string(),
        data: "".to_string(),
        attrs,
        childs: vec![child],
    };
    
    let display_str = format!("{}", parent);
    assert!(display_str.contains("<div"));
    assert!(display_str.contains("id=\"test\""));
    assert!(display_str.contains("class=\"container\""));
    assert!(display_str.contains("child content"));
    assert!(display_str.contains("</div>"));
}