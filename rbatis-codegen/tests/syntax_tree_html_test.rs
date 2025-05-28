use rbatis_codegen::codegen::syntax_tree_html::*;
use rbatis_codegen::codegen::loader_html::Element;
use std::collections::HashMap;

#[test]
fn test_if_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("test".to_string(), "user_id != null".to_string());
    
    let element = Element {
        tag: "if".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = IfTagNode::from_element(&element);
    assert_eq!(node.test, "user_id != null");
    assert_eq!(IfTagNode::node_tag_name(), "if");
}

#[test]
fn test_bind_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("name".to_string(), "user_name".to_string());
    attrs.insert("value".to_string(), "#{name}".to_string());
    
    let element = Element {
        tag: "bind".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = BindTagNode::from_element(&element);
    assert_eq!(node.name, "user_name");
    assert_eq!(node.value, "#{name}");
    assert_eq!(BindTagNode::node_tag_name(), "bind");
}

#[test]
fn test_where_tag_node_creation() {
    let attrs = HashMap::new();
    let element = Element {
        tag: "where".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
      let node = WhereTagNode::from_element(&element);
    assert_eq!(WhereTagNode::node_tag_name(), "where");
    // Verify the node was created successfully
    assert_eq!(node.attrs.len(), 0);
}

#[test]
fn test_set_tag_node_creation() {
    let attrs = HashMap::new();
    let element = Element {
        tag: "set".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
      let node = SetTagNode::from_element(&element);
    assert_eq!(SetTagNode::node_tag_name(), "set");
    // Verify the node was created successfully
    assert_eq!(node.attrs.len(), 0);
}

#[test]
fn test_trim_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("prefix".to_string(), "WHERE".to_string());
    attrs.insert("prefixOverrides".to_string(), "AND |OR ".to_string());
    
    let element = Element {
        tag: "trim".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = TrimTagNode::from_element(&element);
    assert_eq!(node.prefix, "WHERE");
    assert_eq!(node.prefix_overrides, "AND |OR ");
    assert_eq!(TrimTagNode::node_tag_name(), "trim");
}

#[test]
fn test_foreach_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("collection".to_string(), "list".to_string());
    attrs.insert("item".to_string(), "item".to_string());
    attrs.insert("index".to_string(), "index".to_string());
    attrs.insert("open".to_string(), "(".to_string());
    attrs.insert("close".to_string(), ")".to_string());
    attrs.insert("separator".to_string(), ",".to_string());
    
    let element = Element {
        tag: "foreach".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
      let node = ForeachTagNode::from_element(&element);
    assert_eq!(node.collection, "list");
    assert_eq!(node.item, "item");
    assert_eq!(node.index, "index");
    assert_eq!(node.open, "(");
    assert_eq!(node.close, ")");
    assert_eq!(node.separator, ",");
    assert_eq!(ForeachTagNode::node_tag_name(), "foreach");
}

#[test]
fn test_choose_tag_node_creation() {
    let attrs = HashMap::new();
    let element = Element {
        tag: "choose".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
      let node = ChooseTagNode::from_element(&element);
    assert_eq!(ChooseTagNode::node_tag_name(), "choose");
    // Verify the node was created successfully
    assert_eq!(node.attrs.len(), 0);
}

#[test]
fn test_when_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("test".to_string(), "type == 'admin'".to_string());
    
    let element = Element {
        tag: "when".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = WhenTagNode::from_element(&element);
    assert_eq!(node.test, "type == 'admin'");
    assert_eq!(WhenTagNode::node_tag_name(), "when");
}

#[test]
fn test_otherwise_tag_node_creation() {
    let attrs = HashMap::new();
    let element = Element {
        tag: "otherwise".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
      let node = OtherwiseTagNode::from_element(&element);
    assert_eq!(OtherwiseTagNode::node_tag_name(), "otherwise");
    // Verify the node was created successfully
    assert_eq!(node.attrs.len(), 0);
}

#[test]
fn test_select_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "selectUser".to_string());
    attrs.insert("resultType".to_string(), "User".to_string());
    
    let element = Element {
        tag: "select".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = SelectTagNode::from_element(&element);
    assert!(node.attrs.contains_key("id"));
    assert_eq!(node.attrs.get("id").unwrap(), "selectUser");
    assert_eq!(SelectTagNode::node_tag_name(), "select");
}

#[test]
fn test_insert_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "insertUser".to_string());
    
    let element = Element {
        tag: "insert".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = InsertTagNode::from_element(&element);
    assert!(node.attrs.contains_key("id"));
    assert_eq!(node.attrs.get("id").unwrap(), "insertUser");
    assert_eq!(InsertTagNode::node_tag_name(), "insert");
}

#[test]
fn test_update_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "updateUser".to_string());
    
    let element = Element {
        tag: "update".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = UpdateTagNode::from_element(&element);
    assert!(node.attrs.contains_key("id"));
    assert_eq!(node.attrs.get("id").unwrap(), "updateUser");
    assert_eq!(UpdateTagNode::node_tag_name(), "update");
}

#[test]
fn test_delete_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "deleteUser".to_string());
    
    let element = Element {
        tag: "delete".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = DeleteTagNode::from_element(&element);
    assert!(node.attrs.contains_key("id"));
    assert_eq!(node.attrs.get("id").unwrap(), "deleteUser");
    assert_eq!(DeleteTagNode::node_tag_name(), "delete");
}

#[test]
fn test_continue_tag_node_creation() {
    let attrs = HashMap::new();
    let element = Element {
        tag: "continue".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
      let node = ContinueTagNode::from_element(&element);
    assert_eq!(ContinueTagNode::node_tag_name(), "continue");
    // Verify the node was created successfully
    assert_eq!(node.attrs.len(), 0);
}

#[test]
fn test_break_tag_node_creation() {
    let attrs = HashMap::new();
    let element = Element {
        tag: "break".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
      let node = BreakTagNode::from_element(&element);
    assert_eq!(BreakTagNode::node_tag_name(), "break");
    // Verify the node was created successfully
    assert_eq!(node.attrs.len(), 0);
}

#[test]
fn test_sql_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "commonColumns".to_string());
    
    let element = Element {
        tag: "sql".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = SqlTagNode::from_element(&element);
    assert_eq!(node.id, "commonColumns");
    assert_eq!(SqlTagNode::node_tag_name(), "sql");
}

#[test]
fn test_include_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("refid".to_string(), "commonColumns".to_string());
    
    let element = Element {
        tag: "include".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = IncludeTagNode::from_element(&element);
    assert_eq!(node.refid, "commonColumns");
    assert_eq!(IncludeTagNode::node_tag_name(), "include");
}

#[test]
fn test_mapper_tag_node_creation() {
    let mut attrs = HashMap::new();
    attrs.insert("namespace".to_string(), "UserMapper".to_string());
    
    let element = Element {
        tag: "mapper".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = MapperTagNode::from_element(&element);
    assert!(node.attrs.contains_key("namespace"));
    assert_eq!(node.attrs.get("namespace").unwrap(), "UserMapper");
    assert_eq!(MapperTagNode::node_tag_name(), "mapper");
}

#[test]
#[should_panic(expected = "<if> element must have test field!")]
fn test_if_tag_node_missing_test_attribute() {
    let attrs = HashMap::new();
    let element = Element {
        tag: "if".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    IfTagNode::from_element(&element);
}

#[test]
#[should_panic(expected = "<bind> element must have name!")]
fn test_bind_tag_node_missing_name_attribute() {
    let mut attrs = HashMap::new();
    attrs.insert("value".to_string(), "#{name}".to_string());
    
    let element = Element {
        tag: "bind".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    BindTagNode::from_element(&element);
}

#[test]
#[should_panic(expected = "[rbatis-codegen] <foreach> element must have a 'collection' attribute.")]
fn test_foreach_tag_node_missing_collection_attribute() {
    let mut attrs = HashMap::new();
    attrs.insert("item".to_string(), "item".to_string());
    
    let element = Element {
        tag: "foreach".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    ForeachTagNode::from_element(&element);
}