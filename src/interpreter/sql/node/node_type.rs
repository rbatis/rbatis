use crate::core::convert::StmtConvert;
use crate::interpreter::sql::ast::RbatisAST;
use crate::interpreter::sql::node::bind_node::BindNode;
use crate::interpreter::sql::node::choose_node::ChooseNode;
use crate::interpreter::sql::node::foreach_node::ForEachNode;
use crate::interpreter::sql::node::if_node::IfNode;
use crate::interpreter::sql::node::otherwise_node::OtherwiseNode;
use crate::interpreter::sql::node::print_node::PrintNode;
use crate::interpreter::sql::node::proxy_node::ProxyNode;
use crate::interpreter::sql::node::set_node::SetNode;
use crate::interpreter::sql::node::string_node::StringNode;
use crate::interpreter::sql::node::trim_node::TrimNode;
use crate::interpreter::sql::node::when_node::WhenNode;
use crate::interpreter::sql::node::where_node::WhereNode;
use rexpr::runtime::RExprRuntime;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub enum NodeType {
    Null,
    NString(StringNode),
    NIf(IfNode),
    NTrim(TrimNode),
    NForEach(ForEachNode),
    NChoose(ChooseNode),
    NOtherwise(OtherwiseNode),
    NWhen(WhenNode),
    NBind(BindNode),
    NSet(SetNode),
    NWhere(WhereNode),
    NPrint(PrintNode),
    NCustom(ProxyNode),
}

impl NodeType {
    pub fn childs(&self) -> Option<&Vec<NodeType>> {
        match self {
            NodeType::Null => return None,
            NodeType::NString(node) => return None,
            NodeType::NIf(node) => return Some(&node.childs),
            NodeType::NTrim(node) => return Some(&node.childs),
            NodeType::NForEach(node) => return Some(&node.childs),
            NodeType::NChoose(node) => return None,
            NodeType::NOtherwise(node) => return Some(&node.childs),
            NodeType::NWhen(node) => return Some(&node.childs),
            NodeType::NBind(node) => return None,
            NodeType::NSet(node) => return Some(&node.childs),
            NodeType::NWhere(node) => return Some(&node.childs),
            NodeType::NPrint(node) => return Some(&node.childs),
            NodeType::NCustom(node) => return Some(&node.childs),
        }
    }
    pub fn childs_mut(&mut self) -> Option<&mut Vec<NodeType>> {
        match self {
            NodeType::Null => return None,
            NodeType::NString(node) => return None,
            NodeType::NIf(node) => return Some(&mut node.childs),
            NodeType::NTrim(node) => return Some(&mut node.childs),
            NodeType::NForEach(node) => return Some(&mut node.childs),
            NodeType::NChoose(node) => return None,
            NodeType::NOtherwise(node) => return Some(&mut node.childs),
            NodeType::NWhen(node) => return Some(&mut node.childs),
            NodeType::NBind(node) => return None,
            NodeType::NSet(node) => return Some(&mut node.childs),
            NodeType::NWhere(node) => return Some(&mut node.childs),
            NodeType::NPrint(node) => return Some(&mut node.childs),
            NodeType::NCustom(node) => return Some(&mut node.childs),
        }
    }
}

impl<'a> RbatisAST for NodeType {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "node_type"
    }

    fn eval(
        &self,
        convert: &dyn crate::interpreter::sql::StringConvert,
        env: &mut Value,
        engine: &RExprRuntime,
        arg_array: &mut Vec<Value>,
        arg_sql: &mut String,
    ) -> Result<serde_json::Value, crate::core::Error> {
        match self {
            NodeType::Null => return Result::Ok(serde_json::Value::Null),
            NodeType::NString(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NIf(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NTrim(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NForEach(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NChoose(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NOtherwise(node) => {
                return node.eval(convert, env, engine, arg_array, arg_sql)
            }
            NodeType::NWhen(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NBind(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NSet(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NWhere(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NPrint(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
            NodeType::NCustom(node) => return node.eval(convert, env, engine, arg_array, arg_sql),
        }
    }
}
