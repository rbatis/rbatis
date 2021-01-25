use rexpr::ast::Node;
use rexpr::token::TokenMap;

pub mod bind_node;
pub mod choose_node;
pub mod foreach_node;
pub mod if_node;
pub mod node;
pub mod node_type;
pub mod otherwise_node;
pub mod print_node;
pub mod proxy_node;
pub mod set_node;
pub mod string_node;
pub mod trim_node;
pub mod when_node;
pub mod where_node;

pub fn parse_node(arg: &str) -> rexpr::error::Result<Node> {
    rexpr::lexer::lexer_parse_node(arg, &TokenMap::new())
}
