use crate::codegen::syntax_tree::Name;

/// the string node
/// for example:
/// "xxxxxxx" or  `xxxxxxx`
#[derive(Clone, Debug)]
pub struct StringNode {
    pub value: String,
}

impl Name for String {
    fn name() -> &'static str {
        "string"
    }
}
