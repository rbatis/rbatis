use super::{HtmlAstNode, NodeContext};
use crate::codegen::loader_html::{load_mapper_vec, Element};
use crate::error::Error;
use proc_macro2::{Span, TokenStream};
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
    /// source line of the tag in the template, `0` when unknown
    pub line: usize,
    /// source text of that line, used in diagnostics
    pub line_text: String,
}

impl IncludeTagNode {
    fn code_frame(&self, hint: &str) -> String {
        crate::codegen::loader_html::code_frame(self.line, &self.line_text, hint)
    }

    /// Resolves this include element to the referenced `<sql>` fragment.
    ///
    /// `refid` can be either:
    /// - a plain id defined in the current document (`<sql id="...">`), or
    /// - a `file://{path}?refid={id}` URL referencing a fragment in another mapper file.
    pub fn try_process_include(
        &self,
        sql_map: &BTreeMap<String, Element>,
    ) -> Result<Element, Error> {
        let ref_id = &self.refid;

        let url = if ref_id.contains("://") {
            Url::parse(ref_id).map_err(|_| {
                Error::from(format!(
                    "[rbatis-codegen] parse <include refid=\"{}\"> fail!{}",
                    ref_id,
                    self.code_frame("refid must be a valid URL")
                ))
            })?
        } else {
            Url::parse(&format!("current://current?refid={}", ref_id)).map_err(|_| {
                Error::from(format!(
                    "[rbatis-codegen] parse <include refid=\"{}\"> fail!{}",
                    ref_id,
                    self.code_frame("refid must be a valid URL")
                ))
            })?
        };

        match url.scheme() {
            "file" => self.try_handle_file_include(&url, ref_id),
            "current" => self.try_handle_current_include(&url, ref_id, sql_map),
            _ => Err(Error::from(format!(
                "[rbatis-codegen] unsupported include URL scheme in <include refid=\"{}\">{}",
                ref_id,
                self.code_frame("use file://path?refid=id or a plain <sql id>")
            ))),
        }
    }

    /// Panicking compatibility wrapper around [`IncludeTagNode::try_process_include`].
    pub fn process_include(&self, sql_map: &BTreeMap<String, Element>) -> Element {
        self.try_process_include(sql_map)
            .unwrap_or_else(|e| panic!("{}", e))
    }

    /// Handles file-based includes, e.g. `<include refid="file://example.html?refid=a"></include>`
    fn try_handle_file_include(&self, url: &Url, ref_id: &str) -> Result<Element, Error> {
        let mut manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|_| Error::from("Failed to read CARGO_MANIFEST_DIR"))?;
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
            .ok_or_else(|| {
                Error::from(format!(
                    "[rbatis-codegen] No ref_id found in URL '{}'{}",
                    ref_id,
                    self.code_frame("add the ?refid=id query parameter")
                ))
            })?;

        let mut file = File::open(&file_path).map_err(|_| {
            Error::from(format!(
                "[rbatis-codegen] can't find file='{}', url='{}'{}",
                file_path.to_str().unwrap_or_default(),
                url,
                self.code_frame("check the include file path")
            ))
        })?;

        let mut html = String::new();
        file.read_to_string(&mut html).map_err(|e| {
            Error::from(format!(
                "[rbatis-codegen] failed to read include file='{}': {}{}",
                file_path.to_str().unwrap_or_default(),
                e,
                self.code_frame("check the included file")
            ))
        })?;

        load_mapper_vec(&html)?
            .into_iter()
            .find(|e| e.tag == SQL_TAG && e.attrs.get("id") == Some(&ref_id))
            .ok_or_else(|| {
                Error::from(format!(
                    "[rbatis-codegen] No <sql id=\"{}\"> found in file='{}'{}",
                    ref_id,
                    file_path.to_str().unwrap_or_default(),
                    self.code_frame("add the missing <sql id=\"...\"> to the included file")
                ))
            })
    }

    /// Handles includes referencing a `<sql>` fragment defined in the same document
    fn try_handle_current_include(
        &self,
        url: &Url,
        ref_id: &str,
        sql_map: &BTreeMap<String, Element>,
    ) -> Result<Element, Error> {
        let ref_id = url
            .query_pairs()
            .find(|(k, _)| k == "refid")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| ref_id.to_string());

        sql_map.get(&ref_id).cloned().ok_or_else(|| {
            Error::from(format!(
                "[rbatis-codegen] cannot find element <sql id=\"{}\"> referenced by <include>{}",
                ref_id,
                self.code_frame("add the referenced <sql id=\"...\"> fragment")
            ))
        })
    }
}

impl HtmlAstNode for IncludeTagNode {
    fn node_tag_name() -> &'static str {
        "include"
    }

    fn try_from_element(element: &Element, span: Span) -> syn::Result<Self> {
        let refid = match element.attrs.get("refid") {
            Some(refid) => refid.clone(),
            None => {
                return Err(syn::Error::new(
                    span,
                    format!(
                        "[rbatis-codegen] <include> is missing the required \"refid\" attribute{}",
                        element.code_frame("add refid=\"...\"")
                    ),
                ))
            }
        };
        if refid.trim().is_empty() {
            return Err(syn::Error::new(
                span,
                format!(
                    "[rbatis-codegen] <include> has an empty \"refid\" attribute{}",
                    element.code_frame("refid must not be empty")
                ),
            ));
        }
        Ok(Self {
            refid,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
            line: element.line,
            line_text: element.line_text.clone(),
        })
    }

    fn try_generate_tokens<FChildParser>(
        &self,
        context: &mut NodeContext<FChildParser>,
        ignore: &mut Vec<String>,
    ) -> syn::Result<TokenStream>
    where
        FChildParser:
            FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> syn::Result<TokenStream>,
    {
        // `<include>` is normally resolved before token generation by `include_replace`
        // (inside `load_mapper_map`), which replaces it with the referenced `<sql>` fragment.
        // If an unresolved `<include>` reaches this point, just parse its own children.
        context.parse_children(&self.childs, ignore)
    }
}
