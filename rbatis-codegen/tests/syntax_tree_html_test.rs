use rbatis_codegen::codegen::syntax_tree_html::{ForeachTagNode, TrimTagNode, SetTagNode, HtmlAstNode};
use rbatis_codegen::codegen::loader_html::Element;
use std::collections::HashMap;
use quote::quote;

#[test]
fn test_foreach_tag_node_from_element() {
    let mut attrs = HashMap::new();
    attrs.insert("collection".to_string(), "items".to_string());
    attrs.insert("item".to_string(), "it".to_string());
    attrs.insert("index".to_string(), "idx".to_string());
    attrs.insert("open".to_string(), "(".to_string());
    attrs.insert("close".to_string(), ")".to_string());
    attrs.insert("separator".to_string(), ",".to_string());
    
    let element = Element {
        tag: "foreach".to_string(),
        data: String::new(),
        attrs,
        childs: vec![
            Element {
                tag: "".to_string(),
                data: "#{it}".to_string(),
                attrs: HashMap::new(),
                childs: vec![],
            }
        ],
    };
    
    let node = ForeachTagNode::from_element(&element);
    
    assert_eq!(node.collection, "items");
    assert_eq!(node.item, "it");
    assert_eq!(node.index, "idx");
    assert_eq!(node.open, "(");
    assert_eq!(node.close, ")");
    assert_eq!(node.separator, ",");
    assert_eq!(node.childs.len(), 1);
}

#[test]
fn test_foreach_tag_node_with_defaults() {
    let mut attrs = HashMap::new();
    attrs.insert("collection".to_string(), "items".to_string());
    
    let element = Element {
        tag: "foreach".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = ForeachTagNode::from_element(&element);
    
    assert_eq!(node.collection, "items");
    assert_eq!(node.item, "item");  // Default value
    assert_eq!(node.index, "index");  // Default value
    assert_eq!(node.open, "");
    assert_eq!(node.close, "");
    assert_eq!(node.separator, "");
}

#[test]
#[should_panic(expected = "must have a 'collection' attribute")]
fn test_foreach_tag_node_no_collection() {
    let attrs = HashMap::new();
    
    let element = Element {
        tag: "foreach".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    ForeachTagNode::from_element(&element);
}

#[test]
fn test_trim_tag_node_from_element() {
    let mut attrs = HashMap::new();
    attrs.insert("prefix".to_string(), "WHERE".to_string());
    attrs.insert("suffix".to_string(), "".to_string());
    attrs.insert("prefixOverrides".to_string(), "AND |OR".to_string());
    attrs.insert("suffixOverrides".to_string(), ",".to_string());
    
    let element = Element {
        tag: "trim".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = TrimTagNode::from_element(&element);
    
    assert_eq!(node.prefix, "WHERE");
    assert_eq!(node.suffix, "");
    assert_eq!(node.prefix_overrides, "AND |OR");
    assert_eq!(node.suffix_overrides, ",");
}

#[test]
fn test_trim_tag_node_with_legacy_attributes() {
    let mut attrs = HashMap::new();
    attrs.insert("prefix".to_string(), "WHERE".to_string());
    attrs.insert("suffix".to_string(), "".to_string());
    attrs.insert("start".to_string(), "AND |OR".to_string());
    attrs.insert("end".to_string(), ",".to_string());
    
    let element = Element {
        tag: "trim".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = TrimTagNode::from_element(&element);
    
    assert_eq!(node.prefix, "WHERE");
    assert_eq!(node.suffix, "");
    assert_eq!(node.prefix_overrides, "AND |OR");
    assert_eq!(node.suffix_overrides, ",");
}

#[test]
fn test_set_tag_node_from_element() {
    let mut attrs = HashMap::new();
    attrs.insert("collection".to_string(), "data".to_string());
    attrs.insert("skip_null".to_string(), "true".to_string());
    attrs.insert("skips".to_string(), "id,created_at".to_string());
    
    let element = Element {
        tag: "set".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = SetTagNode::from_element(&element);
    
    assert_eq!(node.collection, Some("data".to_string()));
    assert_eq!(node.skip_null, Some("true".to_string()));
    assert_eq!(node.skips, "id,created_at");
}

#[test]
fn test_set_tag_node_with_defaults() {
    let attrs = HashMap::new();
    
    let element = Element {
        tag: "set".to_string(),
        data: String::new(),
        attrs,
        childs: vec![],
    };
    
    let node = SetTagNode::from_element(&element);
    
    assert_eq!(node.collection, None);
    assert_eq!(node.skip_null, None);
    assert_eq!(node.skips, "id");
}

#[test]
fn test_trim_tag_node_token_generation() {
    let mut attrs = HashMap::new();
    attrs.insert("prefix".to_string(), "WHERE".to_string());
    attrs.insert("suffix".to_string(), "".to_string());
    attrs.insert("prefixOverrides".to_string(), "AND |OR".to_string());
    
    let element = Element {
        tag: "trim".to_string(),
        data: String::new(),
        attrs,
        childs: vec![
            Element {
                tag: "".to_string(),
                data: "AND name = #{name}".to_string(),
                attrs: HashMap::new(),
                childs: vec![],
            }
        ],
    };
    
    let node = TrimTagNode::from_element(&element);
    
    let mut ignore = Vec::new();
    let mut methods = quote::quote!();
    
    fn dummy_parser(
        _: &[Element], 
        _: &mut proc_macro2::TokenStream, 
        _: &mut Vec<String>, 
        _: &str
    ) -> proc_macro2::TokenStream {
        quote!()
    }
    
    let mut context = rbatis_codegen::codegen::syntax_tree_html::NodeContext {
        methods: &mut methods,
        fn_name: "test",
        child_parser: &mut dummy_parser,
    };
    
    let tokens = node.generate_tokens(&mut context, &mut ignore);
    let token_str = tokens.to_string();
    
    // Check that the prefix is added
    assert!(token_str.contains("WHERE"));
    
    // Check that prefixOverrides is handled
    assert!(token_str.contains("trim_start_matches"));
    assert!(token_str.contains("AND"));
}

#[test]
fn test_foreach_tag_node_token_generation() {
    let mut attrs = HashMap::new();
    attrs.insert("collection".to_string(), "items".to_string());
    attrs.insert("item".to_string(), "it".to_string());
    attrs.insert("index".to_string(), "idx".to_string());
    attrs.insert("open".to_string(), "(".to_string());
    attrs.insert("close".to_string(), ")".to_string());
    attrs.insert("separator".to_string(), ",".to_string());
    
    let element = Element {
        tag: "foreach".to_string(),
        data: String::new(),
        attrs,
        childs: vec![
            Element {
                tag: "".to_string(),
                data: "#{it}".to_string(),
                attrs: HashMap::new(),
                childs: vec![],
            }
        ],
    };
    
    let node = ForeachTagNode::from_element(&element);
    
    let mut ignore = Vec::new();
    let mut methods = quote::quote!();
    
    fn dummy_parser(
        _: &[Element], 
        _: &mut proc_macro2::TokenStream, 
        _: &mut Vec<String>, 
        _: &str
    ) -> proc_macro2::TokenStream {
        quote!()
    }
    
    let mut context = rbatis_codegen::codegen::syntax_tree_html::NodeContext {
        methods: &mut methods,
        fn_name: "test",
        child_parser: &mut dummy_parser,
    };
    
    let tokens = node.generate_tokens(&mut context, &mut ignore);
    let token_str = tokens.to_string();
    
    // Check that for loop is generated
    assert!(token_str.contains("for"));
    
    // Check that open/close/separator are handled
    assert!(token_str.contains("("));
    assert!(token_str.contains(")"));
    assert!(token_str.contains(","));
    
    // Check that the collection expression is included
    assert!(token_str.contains("items"));
}