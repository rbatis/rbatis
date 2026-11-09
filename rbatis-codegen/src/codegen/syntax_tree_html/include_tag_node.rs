use super::{HtmlAstNode, NodeContext};
use crate::codegen::loader_html::{load_mapper_vec, Element};
use proc_macro2::TokenStream;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use url::Url;

// Constants copied from parser_html.rs for local use in include logic
const SQL_TAG: &str = "sql";

/// Represents an `<include>` tag node in the HTML AST.
///
/// `<include refid="...">` tags are resolved at load time by
/// `parser_html::load_mapper_map` (via `include_replace`), which swaps each
/// `<include>` element with the content of the referenced `<sql id="...">`
/// fragment before any SQL code is generated.
#[derive(Debug, Clone)]
pub struct IncludeTagNode {
    /// Extracted from the "refid" attribute.
    pub refid: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl IncludeTagNode {
    /// Resolves this include element to the referenced `<sql>` fragment.
    ///
    /// `refid` can be either:
    /// - a plain id defined in the current document (`<sql id="...">`), or
    /// - a `file://{path}?refid={id}` URL referencing a fragment in another mapper file.
    pub fn process_include(&self, sql_map: &BTreeMap<String, Element>) -> Element {
        let ref_id = &self.refid;

        let url = if ref_id.contains("://") {
            Url::parse(ref_id).unwrap_or_else(|_| {
                panic!(
                    "[rbatis-codegen] parse <include refid=\"{}\"> fail!",
                    ref_id
                )
            })
        } else {
            Url::parse(&format!("current://current?refid={}", ref_id)).unwrap_or_else(|_| {
                panic!(
                    "[rbatis-codegen] parse <include refid=\"{}\"> fail!",
                    ref_id
                )
            })
        };

        match url.scheme() {
            "file" => self.handle_file_include(&url, ref_id),
            "current" => self.handle_current_include(&url, ref_id, sql_map),
            _ => panic!("Unimplemented scheme <include refid=\"{}\">", ref_id),
        }
    }

    /// Handles file-based includes, e.g. `<include refid="file://example.html?refid=a"></include>`
    fn handle_file_include(&self, url: &Url, ref_id: &str) -> Element {
        let mut manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Failed to read CARGO_MANIFEST_DIR");
        manifest_dir.push('/');

        let path = url.host_str().unwrap_or_default().to_string()
            + url.path().trim_end_matches(&['/', '\\'][..]);
        let mut file_path = PathBuf::from(&path);

        if file_path.is_relative() {
            file_path = PathBuf::from(format!("{}{}", manifest_dir, path));
        }

        let ref_id = url
            .query_pairs()
            .find(|(k, _)| k == "refid")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| {
                panic!("No ref_id found in URL {}", ref_id);
            });

        let mut file = File::open(&file_path).unwrap_or_else(|_| {
            panic!(
                "[rbatis-codegen] can't find file='{}', url='{}'",
                file_path.to_str().unwrap_or_default(),
                url
            )
        });

        let mut html = String::new();
        file.read_to_string(&mut html).expect("Failed to read file");

        load_mapper_vec(&html)
            .unwrap_or_else(|e| {
                panic!(
                    "[rbatis-codegen] parse include file='{}' fail: {}",
                    file_path.to_str().unwrap_or_default(),
                    e
                )
            })
            .into_iter()
            .find(|e| e.tag == SQL_TAG && e.attrs.get("id") == Some(&ref_id))
            .unwrap_or_else(|| {
                panic!(
                    "[rbatis-codegen] No <sql id=\"{}\"> found in file='{}'",
                    ref_id,
                    file_path.to_str().unwrap_or_default()
                )
            })
    }

    /// Handles includes referencing a `<sql>` fragment defined in the same document
    fn handle_current_include(
        &self,
        url: &Url,
        ref_id: &str,
        sql_map: &BTreeMap<String, Element>,
    ) -> Element {
        let ref_id = url
            .query_pairs()
            .find(|(k, _)| k == "refid")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| ref_id.to_string());

        sql_map
            .get(&ref_id)
            .unwrap_or_else(|| {
                panic!(
                    "[rbatis-codegen] cannot find element <include refid=\"{}\">!",
                    ref_id
                )
            })
            .clone()
    }
}

impl HtmlAstNode for IncludeTagNode {
    fn node_tag_name() -> &'static str {
        "include"
    }

    fn from_element(element: &Element) -> Self {
        let refid = element
            .attrs
            .get("refid")
            .expect("[rbatis-codegen] <include> element must have attr <include refid=\"\">!")
            .clone();
        Self {
            refid,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(
        &self,
        context: &mut NodeContext<FChildParser>,
        ignore: &mut Vec<String>,
    ) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // `<include>` is normally resolved before token generation by `include_replace`
        // (inside `load_mapper_map`), which replaces it with the referenced `<sql>` fragment.
        // If an unresolved `<include>` reaches this point, just parse its own children.
        context.parse_children(&self.childs, ignore)
    }
}
