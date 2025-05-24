use proc_macro2::TokenStream;
use crate::codegen::loader_html::Element;

pub mod sql_tag_node;
pub mod include_tag_node;
pub mod mapper_tag_node;
pub mod if_tag_node;
pub mod trim_tag_node;
pub mod bind_tag_node;
pub mod where_tag_node;
pub mod choose_tag_node;
pub mod when_tag_node;
pub mod otherwise_tag_node;
pub mod foreach_tag_node;
pub mod set_tag_node;
pub mod continue_tag_node;
pub mod break_tag_node;
pub mod select_tag_node;
pub mod update_tag_node;
pub mod insert_tag_node;
pub mod delete_tag_node;

// Re-export all node structs for easier access
pub use sql_tag_node::SqlTagNode;
pub use include_tag_node::IncludeTagNode;
pub use mapper_tag_node::MapperTagNode;
pub use if_tag_node::IfTagNode;
pub use trim_tag_node::TrimTagNode;
pub use bind_tag_node::BindTagNode;
pub use where_tag_node::WhereTagNode;
pub use choose_tag_node::ChooseTagNode;
pub use when_tag_node::WhenTagNode;
pub use otherwise_tag_node::OtherwiseTagNode;
pub use foreach_tag_node::ForeachTagNode;
pub use set_tag_node::SetTagNode;
pub use continue_tag_node::ContinueTagNode;
pub use break_tag_node::BreakTagNode;
pub use select_tag_node::SelectTagNode;
pub use update_tag_node::UpdateTagNode;
pub use insert_tag_node::InsertTagNode;
pub use delete_tag_node::DeleteTagNode;

/// Context passed around during token generation.
/// FChildParser is a type parameter for the function that parses child elements.
pub struct NodeContext<'a, FChildParser>
where
    FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
{
    pub methods: &'a mut TokenStream, // For accumulating helper methods (e.g., for CRUD operations)
    pub fn_name: &'a str,            // The name of the main function being generated
    pub child_parser: FChildParser,   // The function to call to parse child Elements
}

impl<'a, FChildParser> NodeContext<'a, FChildParser>
where
    FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream,
{
    /// Helper method to parse child elements using the provided child_parser function.
    /// The `ignore` vector is passed directly here for flexibility with constructs like <foreach>.
    pub fn parse_children(&mut self, children: &[Element], ignore: &mut Vec<String>) -> TokenStream {
        (self.child_parser)(children, self.methods, ignore, self.fn_name)
    }
}

/// Trait for all HTML abstract syntax tree (AST) nodes.
/// Defines how a node is created from an `Element` and how it generates Rust TokenStream.
pub trait HtmlAstNode {
    /// Returns the XML tag name for this node type (e.g., "if", "select").
    fn node_tag_name() -> &'static str
    where
        Self: Sized;

    /// Creates an instance of the node from a generic `Element`.
    /// This method will extract necessary attributes and validate them.
    /// Can panic if attributes are missing, similar to original code's expect().
    fn from_element(element: &Element) -> Self
    where
        Self: Sized;

    /// Generates the Rust `TokenStream` for this specific AST node.
    /// The `ignore` vector is passed directly to allow modification by calling nodes (e.g. for <foreach> scope).
    fn generate_tokens<FChildParser>(&self, context: &mut NodeContext<FChildParser>, ignore: &mut Vec<String>) -> TokenStream
    where
        FChildParser: FnMut(&[Element], &mut TokenStream, &mut Vec<String>, &str) -> TokenStream;
} 