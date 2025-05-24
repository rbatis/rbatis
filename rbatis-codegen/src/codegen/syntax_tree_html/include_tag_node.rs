use std::collections::{BTreeMap, HashMap};
use proc_macro2::TokenStream;
use crate::codegen::loader_html::{Element, load_html};
use super::{HtmlAstNode, NodeContext, SqlTagNode};
use url::Url;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::error::Error;

// Constants copied from parser_html.rs for local use in include logic
const SQL_TAG: &str = "sql";
const MAPPER_TAG: &str = "mapper";

/// Represents an <include> tag node in the HTML AST.
#[derive(Debug, Clone)]
pub struct IncludeTagNode {
    /// Extracted from the "refid" attribute.
    pub refid: String,
    // Unlike other nodes, childs for include are typically empty, as content comes from the refid.
    // However, we keep attrs and childs for completeness and potential edge cases.
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>, 
    // Resolved element after include logic. This is not part of the initial parsing
    // but populated during a specific include resolution step.
    // For now, generate_tokens will have to re-resolve or this struct needs to be
    // created *after* include resolution.
    // Let's stick to the original structure and re-resolve in generate_tokens for now.
}

impl IncludeTagNode {
    /// Duplicated from parser_html.rs to avoid circular imports
    fn load_mapper_vec(html: &str) -> Result<Vec<Element>, Error> {
        let elements = load_html(html).map_err(|e| Error::from(e.to_string()))?;
    
        let mut mappers = Vec::new();
        for element in elements {
            if element.tag == MAPPER_TAG {
                mappers.extend(element.childs);
            } else {
                mappers.push(element);
            }
        }
    
        Ok(mappers)
    }

    /// Processes an include element by resolving its reference
    /// This method is used by the include_replace function in parser_html.rs
    pub fn process_include(&self, sql_map: &BTreeMap<String, Element>) -> Element {
        let ref_id = &self.refid;

        let url = if ref_id.contains("://") {
            Url::parse(ref_id).unwrap_or_else(|_| panic!(
                "[rbatis-codegen] parse <include refid=\"{}\"> fail!", ref_id
            ))
        } else {
            Url::parse(&format!("current://current?refid={}", ref_id)).unwrap_or_else(|_| panic!(
                "[rbatis-codegen] parse <include refid=\"{}\"> fail!", ref_id
            ))
        };

        match url.scheme() {
            "file" => self.handle_file_include(&url, ref_id),
            "current" => self.handle_current_include(&url, ref_id, sql_map),
            _ => panic!("Unimplemented scheme <include refid=\"{}\">", ref_id),
        }
    }

    /// Handles file-based includes
    fn handle_file_include(&self, url: &Url, ref_id: &str) -> Element {
        let mut manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
            .expect("Failed to read CARGO_MANIFEST_DIR");
        manifest_dir.push('/');

        let path = url.host_str().unwrap_or_default().to_string() +
            url.path().trim_end_matches(&['/', '\\'][..]);
        let mut file_path = PathBuf::from(&path);

        if file_path.is_relative() {
            file_path = PathBuf::from(format!("{}{}", manifest_dir, path));
        }

        let ref_id = url.query_pairs()
            .find(|(k, _)| k == "refid")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| {
                panic!("No ref_id found in URL {}", ref_id);
            });

        let mut file = File::open(&file_path).unwrap_or_else(|_| panic!(
            "[rbatis-codegen] can't find file='{}', url='{}'",
            file_path.to_str().unwrap_or_default(),
            url
        ));

        let mut html = String::new();
        file.read_to_string(&mut html).expect("Failed to read file");

        Self::load_mapper_vec(&html).expect("Failed to parse HTML")
            .into_iter()
            .find(|e| e.tag == SQL_TAG && e.attrs.get("id") == Some(&ref_id))
            .unwrap_or_else(|| panic!(
                "No ref_id={} found in file={}",
                ref_id,
                file_path.to_str().unwrap_or_default()
            ))
    }

    /// Handles current document includes
    fn handle_current_include(&self, url: &Url, ref_id: &str, sql_map: &BTreeMap<String, Element>) -> Element {
        let ref_id = url.query_pairs()
            .find(|(k, _)| k == "refid")
            .map(|(_, v)| v.to_string())
            .unwrap_or(ref_id.to_string());

        sql_map.get(&ref_id).unwrap_or_else(|| panic!(
            "[rbatis-codegen] cannot find element <include refid=\"{}\">!",
            ref_id
        )).clone()
    }
}

impl HtmlAstNode for IncludeTagNode {
    fn node_tag_name() -> &'static str { "include" }

    fn from_element(element: &Element) -> Self {
        let refid = element.attrs.get("refid")
            .expect("[rbatis-codegen] <include> element must have attr <include refid=\"\">!")
            .clone();
        Self {
            refid,
            attrs: element.attrs.clone(),
            childs: element.childs.clone(),
        }
    }

    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
    {
        // The original `include_replace` function in `parser_html.rs` resolves the <include>
        // tag and replaces it with the content of the referenced <sql> tag. 
        // This resolution logic needs to be replicated here or called from here.
        // For now, we will replicate the resolving part of `handle_include_element` 
        // and then parse its children (which would be the children of the resolved element).

        // TODO: This sql_map is a temporary workaround. In a full refactor, 
        // the include resolution might happen earlier, or the map passed differently.
        // For now, it means <include> tags can only reference globally defined <sql> tags
        // if we don't have access to the dynamically built sql_map from the initial parsing phase.
        // This is a significant simplification and might not match original behavior perfectly without
        // a broader context of how `sql_map` is built and used during `include_replace`.
        // The original `include_replace` builds `sql_map` recursively.
        // For this standalone generate_tokens, we assume sql_map is not available, 
        // so only file includes or includes referencing globally known (hypothetical) sql tags would work.
        // For simplicity, and to avoid needing the full sql_map, we will *not* try to handle current document includes here.
        // This is a limitation that needs to be addressed in a fuller refactor.

        let url = if self.refid.contains("://") {
            Url::parse(&self.refid).unwrap_or_else(|_|
                panic!("[rbatis-codegen] parse <include refid=\"{}\"> fail!", self.refid)
            )
        } else {
            // This part is problematic without sql_map. The original code does:
            // Url::parse(&format!("current://current?refid={}", self.refid)).unwrap()
            // and then uses sql_map. Since we don't have sql_map here directly,
            // we'll assume non-URL refids are file paths relative to manifest dir for now,
            // or this part needs to be designed to have access to the sql_map.
            // For a more direct port, we'd need `sql_map` in `NodeContext` or similar.
            // Let's assume non-URL means it's a local ID that should have been pre-resolved
            // or it's a file path without `file://`.
            // Given the original `handle_include_element` structure, if it's not `file://`, it assumes `current://`
            // which requires the `sql_map`.
            // This part is tricky to replicate in isolation. 
            // The `include_replace` function fundamentally changes the `Element` tree *before* `parse_elements`.
            // So, by the time `parse_elements` (and thus a hypothetical `generate_tokens`) is called for an `<include>`
            // (if it wasn't replaced), it would mean the replacement logic might need to be invoked.
            // However, the design of `include_replace` suggests it *replaces* the include element.
            // This implies that `generate_tokens` for an `IncludeTagNode` might not even be called
            // if `include_replace` is run first as it was in the original `load_mapper_map`.

            // If `generate_tokens` *is* called on an `IncludeTagNode`, it implies that the
            // `include_replace` pass might not have happened or this node was generated differently.
            // Let's assume for now that `include_replace` has *already* modified the tree, 
            // and this `IncludeTagNode` should ideally contain its resolved children. 
            // This requires `IncludeTagNode::from_element` to be smarter or the tree to be pre-processed.
            // Sticking to the simplest path: if this method is called, the original `parser_html` code 
            // for an `<include>` tag inside `parse_elements` was to parse its children.
            // This implies that `include_replace` must have already put the correct children into this node.
            // So, we just parse `self.childs`.
            return context.parse_children(&self.childs, ignore); 
        };

        let resolved_element = match url.scheme() {
            "file" => {
                let mut manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to read CARGO_MANIFEST_DIR");
                manifest_dir.push('/');
                let path = url.host_str().unwrap_or_default().to_string() + url.path().trim_end_matches(&['/', '\\'][..]);
                let mut file_path = PathBuf::from(&path);
                if file_path.is_relative() {
                    file_path = PathBuf::from(format!("{}{}", manifest_dir, path));
                }
                let fragment_ref_id = url.query_pairs().find(|(k, _)| k == "refid").map(|(_, v)| v.to_string()).unwrap_or_else(|| panic!("No ref_id found in URL {}", self.refid));
                let mut file = File::open(&file_path).unwrap_or_else(|_| panic!("[rbatis-codegen] can't find file='{}', url='{}'", file_path.to_str().unwrap_or_default(), url));
                let mut html = String::new();
                file.read_to_string(&mut html).expect("Failed to read file");
                Self::load_mapper_vec(&html).expect("Failed to parse HTML from included file")
                    .into_iter()
                    .find(|e| e.tag == SqlTagNode::node_tag_name() && e.attrs.get("id") == Some(&fragment_ref_id))
                    .unwrap_or_else(|| panic!("No ref_id={} found in file={}", fragment_ref_id, file_path.to_str().unwrap_or_default()))
            }
            // "current" scheme would require sql_map, which we don't have here.
            // This indicates a design tension. The original `include_replace` modifies the element list.
            // If we are calling generate_tokens on an IncludeTagNode, it means that replacement did not happen
            // or we are trying to re-resolve.  The simplest interpretation is that children have already been resolved.
            _ => {
                 // If it's not a file and not pre-resolved, behavior is undefined in this isolated context.
                 // Original code panicked for unimplemented schemes.
                 // Let's assume children are already correct from a pre-processing step.
                 // So the line `return context.parse_children(&self.childs);` before the `match url.scheme()`
                 // is the most consistent interpretation if `include_replace` is considered part of the tree construction.
                 // Given the structure of `include_replace`, it actively *mutates* the element list or replaces elements.
                 // So, `parse_elements` would usually see the *result* of the include, not the include tag itself.

                 // For robustness, if we reach here, it implies an <include> tag that wasn't a file
                 // and wasn't handled by the early return for non-URL-like refids. 
                 // This path should ideally not be hit if pre-processing is done correctly.
                 // We will panic, similar to original code for unhandled schemes, or rely on the children being pre-filled.
                 // The most robust way is to assume `self.childs` are the resolved ones.
                 // The `return context.parse_children(&self.childs);` was added above to handle this common case.
                 // If we reach here, it's an unexpected state for a file scheme that failed the previous checks.
                 panic!("Unhandled include scheme or state for refid: {}. Ensure includes are pre-resolved or are valid file paths.", self.refid);
            }
        };

        // Now parse the children of the resolved element.
        context.parse_children(&resolved_element.childs, ignore)
    }
} 