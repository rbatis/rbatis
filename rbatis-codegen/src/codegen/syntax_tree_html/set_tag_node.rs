use std::collections::HashMap;
use proc_macro2::TokenStream;
use crate::codegen::loader_html::Element;
use super::{HtmlAstNode, NodeContext, TrimTagNode, ForeachTagNode, IfTagNode, ContinueTagNode};

/// Represents a <set> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct SetTagNode {
    pub collection: Option<String>,
    pub skip_null: Option<String>,
    pub skips: String, // Default to "id"
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl HtmlAstNode for SetTagNode {
    fn node_tag_name() -> &'static str where Self: Sized { "set" }

    fn from_element(element: &Element) -> Self {
        let collection = element.attrs.get("collection").cloned();
        let skip_null = element.attrs.get("skip_null").cloned();
        let skips = element.attrs.get("skips").cloned().unwrap_or_else(|| "id".to_string());

        Self {
            collection,
            skip_null,
            skips,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(), // Original children if not using collection logic
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        if let Some(collection_name) = &self.collection {
            // Logic from `make_sets` in original parser_html.rs
            let is_skip_null = self.skip_null.as_deref().map_or(true, |v| v != "false");
            let skip_strs: Vec<&str> = self.skips.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();

            let mut for_each_child_elements: Vec<Element> = skip_strs.iter().map(|x| Element {
                tag: IfTagNode::node_tag_name().to_string(),
                data: String::new(),
                attrs: {
                    let mut attr = HashMap::new();
                    attr.insert("test".to_string(), format!("k == '{}'", x));
                    attr
                },
                childs: vec![Element {
                    tag: ContinueTagNode::node_tag_name().to_string(),
                    data: String::new(),
                    attrs: HashMap::new(),
                    childs: vec![],
                }],
            }).collect::<Vec<_>>();

            if is_skip_null {
                for_each_child_elements.push(Element {
                    tag: IfTagNode::node_tag_name().to_string(),
                    data: String::new(),
                    attrs: {
                        let mut attr = HashMap::new();
                        attr.insert("test".to_string(), "v == null".to_string());
                        attr
                    },
                    childs: vec![Element {
                        tag: ContinueTagNode::node_tag_name().to_string(),
                        data: String::new(),
                        attrs: HashMap::new(),
                        childs: vec![],
                    }],
                });
            }

            for_each_child_elements.push(Element {
                tag: "".to_string(), // Represents a text node
                data: "${k}=#{v},".to_string(),
                attrs: HashMap::new(),
                childs: vec![],
            });
            
            let foreach_element = Element {
                tag: ForeachTagNode::node_tag_name().to_string(),
                data: String::new(),
                attrs: {
                    let mut attr = HashMap::new();
                    attr.insert("collection".to_string(), collection_name.clone());
                    attr.insert("index".to_string(), "k".to_string());
                    attr.insert("item".to_string(), "v".to_string());
                    attr
                },
                childs: for_each_child_elements,
            };

            let inner_trim_element = Element {
                tag: TrimTagNode::node_tag_name().to_string(),
                data: String::new(),
                attrs: {
                    let mut attr = HashMap::new();
                    attr.insert("prefix".to_string(), "".to_string());
                    attr.insert("suffix".to_string(), "".to_string());
                    attr.insert("start".to_string(), ",".to_string()); //trim_start_matches for comma
                    attr.insert("end".to_string(), ",".to_string());   //trim_end_matches for comma
                    attr
                },
                childs: vec![foreach_element],
            };

            let outer_trim_element = Element {
                tag: TrimTagNode::node_tag_name().to_string(),
                data: String::new(),
                attrs: {
                    let mut attr = HashMap::new();
                    attr.insert("prefix".to_string(), " set ".to_string());
                    attr.insert("suffix".to_string(), " ".to_string());
                    // These were blank in original `make_sets` outer trim, meaning no override trimming at this level
                    attr.insert("start".to_string(), " ".to_string()); 
                    attr.insert("end".to_string(), " ".to_string());
                    attr
                },
                childs: vec![inner_trim_element],
            };

            // Now, create a TrimTagNode from outer_trim_element and generate its tokens.
            let trim_node = TrimTagNode::from_element(&outer_trim_element);
            trim_node.generate_tokens(context, ignore)

        } else {
            // Default behavior: acts like a <trim prefix=" set " suffix=" " prefixOverrides="," suffixOverrides=",">
            // This is slightly different from original parser_html which used " |," for overrides.
            // Let's use the exact overrides from original: " |," means trim leading/trailing spaces and commas.
            let trim_node = TrimTagNode {
                prefix: " set ".to_string(),
                suffix: " ".to_string(),
                prefix_overrides: " |,".to_string(),
                suffix_overrides: " |,".to_string(),
                attrs: self.attrs.clone(), // Keep original attrs if any, though usually none for this path
                childs: self.childs.clone(),
            };
            trim_node.generate_tokens(context, ignore)
        }
    }
} 