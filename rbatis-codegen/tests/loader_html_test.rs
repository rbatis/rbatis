use rbatis_codegen::codegen::loader_html::{Element, load_html, as_element};
use std::collections::HashMap;
use html_parser::Dom;

#[test]
fn test_element_display() {
    // 测试空元素（只有文本数据）的显示
    let text_element = Element {
        tag: "".to_string(),
        data: "这是一个文本节点".to_string(),
        attrs: HashMap::new(),
        childs: vec![],
    };
    assert_eq!(text_element.to_string(), "这是一个文本节点");
    
    // 测试带标签的元素的显示
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "test_id".to_string());
    attrs.insert("class".to_string(), "test_class".to_string());
    
    let tag_element = Element {
        tag: "div".to_string(),
        data: "".to_string(),
        attrs,
        childs: vec![
            Element {
                tag: "".to_string(),
                data: "子文本".to_string(),
                attrs: HashMap::new(),
                childs: vec![],
            }
        ],
    };
    
    // 注意：由于HashMap的无序性，属性的顺序可能不同，所以我们不能直接比较完整的字符串
    let display = tag_element.to_string();
    assert!(display.contains("<div"));
    assert!(display.contains("id=\"test_id\""));
    assert!(display.contains("class=\"test_class\""));
    assert!(display.contains(">子文本</div>"));
}

#[test]
fn test_load_html() {
    // 简单的HTML
    let html = "<div>测试文本</div>";
    let elements = load_html(html).unwrap();
    
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "div");
    assert_eq!(elements[0].childs.len(), 1);
    assert_eq!(elements[0].childs[0].data, "测试文本");
    
    // 测试break标签的替换
    let html = "<break>测试break标签</break>";
    let elements = load_html(html).unwrap();
    
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "break");
    assert_eq!(elements[0].childs.len(), 1);
    assert_eq!(elements[0].childs[0].data, "测试break标签");
    
    // 测试带id属性的HTML
    let html = "<div id=\"test_id\">带id属性的div</div>";
    let elements = load_html(html).unwrap();
    
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "div");
    assert!(elements[0].attrs.contains_key("id"));
    assert_eq!(elements[0].attrs.get("id").unwrap(), "test_id");
    assert_eq!(elements[0].childs.len(), 1);
    assert_eq!(elements[0].childs[0].data, "带id属性的div");
}

#[test]
fn test_element_child_strings() {
    // 创建一个嵌套的元素结构
    let element = Element {
        tag: "div".to_string(),
        data: "".to_string(),
        attrs: HashMap::new(),
        childs: vec![
            Element {
                tag: "".to_string(),
                data: "文本节点1".to_string(),
                attrs: HashMap::new(),
                childs: vec![],
            },
            Element {
                tag: "p".to_string(),
                data: "".to_string(),
                attrs: HashMap::new(),
                childs: vec![
                    Element {
                        tag: "".to_string(),
                        data: "文本节点2".to_string(),
                        attrs: HashMap::new(),
                        childs: vec![],
                    }
                ],
            },
        ],
    };
    
    let strings = element.child_strings();
    assert_eq!(strings.len(), 2);
    assert_eq!(strings[0], "文本节点1");
    assert_eq!(strings[1], "文本节点2");
    
    let string_capacity = element.child_string_cup();
    assert_eq!(string_capacity, "文本节点1".len() + "文本节点2".len());
}

#[test]
fn test_as_element() {
    // 准备Node数据
    let html = "<div>文本<p>段落</p></div>";
    let dom = Dom::parse(html).unwrap();
    
    let elements = as_element(&dom.children);
    
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].tag, "div");
    assert_eq!(elements[0].childs.len(), 2);
    assert_eq!(elements[0].childs[0].data, "文本");
    assert_eq!(elements[0].childs[1].tag, "p");
    assert_eq!(elements[0].childs[1].childs[0].data, "段落");
} 