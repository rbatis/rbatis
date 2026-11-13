use crate::codegen::loader_html::Element;
use proc_macro2::{Span, TokenStream};

pub mod bind_tag_node;
pub mod break_tag_node;
pub mod choose_tag_node;
pub mod continue_tag_node;
pub mod delete_tag_node;
pub mod foreach_tag_node;
pub mod if_tag_node;
pub mod include_tag_node;
pub mod insert_tag_node;
pub mod mapper_tag_node;
pub mod otherwise_tag_node;
pub mod select_tag_node;
pub mod set_tag_node;
pub mod sql_tag_node;
pub mod trim_tag_node;
pub mod update_tag_node;
pub mod when_tag_node;
pub mod where_tag_node;

// Re-export all node structs for easier access
pub use bind_tag_node::BindTagNode;
pub use break_tag_node::BreakTagNode;
pub use choose_tag_node::ChooseTagNode;
pub use continue_tag_node::ContinueTagNode;
pub use delete_tag_node::DeleteTagNode;
pub use foreach_tag_node::ForeachTagNode;
pub use if_tag_node::IfTagNode;
pub use include_tag_node::IncludeTagNode;
pub use insert_tag_node::InsertTagNode;
pub use mapper_tag_node::MapperTagNode;
pub use otherwise_tag_node::OtherwiseTagNode;
pub use select_tag_node::SelectTagNode;
pub use set_tag_node::SetTagNode;
pub use sql_tag_node::SqlTagNode;
pub use trim_tag_node::TrimTagNode;
pub use update_tag_node::UpdateTagNode;
pub use when_tag_node::WhenTagNode;
pub use where_tag_node::WhereTagNode;

/// Context passed around during token generation.
/// FChildParser is a type parameter for the function that parses child elements.
pub struct NodeContext<'a, FChildParser>
where
    FChildParser:
        FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> syn::Result<TokenStream>,
{
    pub methods: &'a mut TokenStream, // For accumulating helper methods (e.g., for CRUD operations)
    pub fn_name: &'a str,             // The name of the main function being generated
    /// span of the template this node comes from, used to report errors at the
    /// `#[py_sql]` / `#[html_sql]` attribute instead of panicking without a location
    pub span: Span,
    pub child_parser: FChildParser, // The function to call to parse child Elements
}

impl<'a, FChildParser> NodeContext<'a, FChildParser>
where
    FChildParser:
        FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> syn::Result<TokenStream>,
{
    /// Helper method to parse child elements using the provided child_parser function.
    /// The `ignore` vector is passed directly here for flexibility with constructs like `<foreach>`.
    pub fn parse_children(
        &mut self,
        children: &[Element],
        ignore: &mut Vec<String>,
    ) -> syn::Result<TokenStream> {
        (self.child_parser)(children, self.methods, ignore, self.fn_name)
    }
}

/// Trait for all HTML abstract syntax tree (AST) nodes.
///
/// The `try_*` methods are the fallible implementation: they return a [`syn::Error`]
/// that carries the span of the template, so a bad template is reported by rustc at
/// the `#[py_sql]` / `#[html_sql]` attribute instead of a text-only panic. The
/// [`from_element`](HtmlAstNode::from_element) and
/// [`generate_tokens`](HtmlAstNode::generate_tokens) wrappers are kept for callers
/// that only need the old panicking API (tests, downstream tools); they panic with
/// the message of the returned [`syn::Error`].
pub trait HtmlAstNode {
    /// Returns the XML tag name for this node type (e.g., "if", "select").
    fn node_tag_name() -> &'static str;

    /// Creates an instance of the node from a generic `Element`, reporting missing
    /// or invalid attributes as a spanned [`syn::Error`].
    fn try_from_element(element: &Element, span: Span) -> syn::Result<Self>
    where
        Self: Sized;

    /// Generates the Rust `TokenStream` for this specific AST node.
    /// The `ignore` vector is passed directly to allow modification by calling nodes (e.g. for `<foreach>` scope).
    fn try_generate_tokens<FChildParser>(
        &self,
        context: &mut NodeContext<FChildParser>,
        ignore: &mut Vec<String>,
    ) -> syn::Result<TokenStream>
    where
        FChildParser:
            FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> syn::Result<TokenStream>;

    /// Panicking compatibility wrapper around [`HtmlAstNode::try_from_element`].
    fn from_element(element: &Element) -> Self
    where
        Self: Sized,
    {
        Self::try_from_element(element, Span::call_site()).unwrap_or_else(|e| panic!("{}", e))
    }

    /// Panicking compatibility wrapper around [`HtmlAstNode::try_generate_tokens`].
    fn generate_tokens<FChildParser>(
        &self,
        context: &mut NodeContext<FChildParser>,
        ignore: &mut Vec<String>,
    ) -> TokenStream
    where
        FChildParser:
            FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> syn::Result<TokenStream>,
    {
        self.try_generate_tokens(context, ignore)
            .unwrap_or_else(|e| panic!("{}", e))
    }
}
