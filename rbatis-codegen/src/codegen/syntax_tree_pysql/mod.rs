/// this the py_sql syntax tree
pub mod bind_node;
pub mod break_node;
pub mod choose_node;
pub mod continue_node;
pub mod error;
pub mod foreach_node;
pub mod if_node;
pub mod otherwise_node;
pub mod set_node;
pub mod sql_node;
pub mod string_node;
pub mod trim_node;
pub mod when_node;
pub mod where_node;
pub mod to_html;

use crate::codegen::syntax_tree_pysql::bind_node::BindNode;
use crate::codegen::syntax_tree_pysql::break_node::BreakNode;
use crate::codegen::syntax_tree_pysql::choose_node::ChooseNode;
use crate::codegen::syntax_tree_pysql::continue_node::ContinueNode;
use crate::codegen::syntax_tree_pysql::foreach_node::ForEachNode;
use crate::codegen::syntax_tree_pysql::if_node::IfNode;
use crate::codegen::syntax_tree_pysql::otherwise_node::OtherwiseNode;
use crate::codegen::syntax_tree_pysql::set_node::SetNode;
use crate::codegen::syntax_tree_pysql::sql_node::SqlNode;
use crate::codegen::syntax_tree_pysql::string_node::StringNode;
use crate::codegen::syntax_tree_pysql::trim_node::TrimNode;
use crate::codegen::syntax_tree_pysql::when_node::WhenNode;
use crate::codegen::syntax_tree_pysql::where_node::WhereNode;

/// PySQL Syntax Tree
/// 
/// The syntax of PySQL is based on Python-like indentation and line structure.
/// Each node type below represents a different structure in the PySQL language.
/// 
/// Syntax Rules:
/// 
/// 1. Nodes that define a block end with a colon ':' and their children are indented.
/// 
/// 2. `NString` - Plain text or SQL fragments. Can preserve whitespace with backticks:
/// ```sql
/// SELECT * FROM users
/// `  SELECT   column1,    column2   FROM table  `
/// ```
/// 
/// 3. `NIf` - Conditional execution, similar to Python's if statement:
/// ```pysql
/// if condition:
///   SQL fragment
/// ```
/// 
/// 4. `NTrim` - Removes specified characters from start/end of the content:
/// ```pysql
/// trim ',':                   # Removes ',' from both start and end
/// trim start=',',end=')':     # Removes ',' from start and ')' from end
/// ```
/// 
/// 5. `NForEach` - Iterates over collections:
/// ```pysql
/// for item in items:          # Simple iteration
///   #{item}
/// for key,item in items:      # With key/index
///   #{key}: #{item}
/// ```
/// 
/// 6. `NChoose`/`NWhen`/`NOtherwise` - Switch-like structure:
/// ```pysql
/// choose:
///   when condition1:
///     SQL fragment 1
///   when condition2:
///     SQL fragment 2
///   otherwise:                # Or use '_:'
///     Default SQL fragment
/// ```
/// 
/// 7. `NBind` - Variable binding:
/// ```pysql
/// bind name = 'value':        # Or use 'let name = value:'
///   SQL using #{name}
/// ```
/// 
/// 8. `NSet` - For UPDATE statements, handles comma separation.
///    It can also define a collection to iterate over for generating SET clauses.
/// ```pysql
/// // Simple set for direct updates
/// set:
///   if name != null:
///     name = #{name},
///   if age != null:
///     age = #{age}
///
/// // Set with collection to iterate (e.g., from a map or struct)
/// // Assuming 'user_updates' is a map like {'name': 'new_name', 'status': 'active'}
/// set collection="user_updates" skips="id,created_at" skip_null="true":
///   // This will generate: name = #{user_updates.name}, status = #{user_updates.status}
///   // 'id' and 'created_at' fields from 'user_updates' will be skipped.
///   // If a value in 'user_updates' is null and skip_null is true, it will be skipped.
/// ```
/// 
/// 9. `NWhere` - For WHERE clauses, handles AND/OR prefixes:
/// ```pysql
/// where:
///   if id != null:
///     AND id = #{id}
///   if name != null:
///     AND name = #{name}
/// ```
/// 
/// 10. `NContinue`/`NBreak` - Loop control, must be inside a for loop:
/// ```pysql
/// for item in items:
///   if item == null:
///     break:
///   if item == 0:
///     continue:
/// ```
/// 
/// 11. `NSql` - Reusable SQL fragments with an ID:
/// ```pysql
/// sql id='userColumns':
///   id, name, age
/// ```
///     
/// Note: All control nodes require a colon at the end, and their child content
/// must be indented with more spaces than the parent node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeType {
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
    NContinue(ContinueNode),
    NBreak(BreakNode),
    NSql(SqlNode),
}

/// the node name
pub trait Name {
    fn name() -> &'static str;
}

/// node default name
pub trait DefaultName {
    fn default_name() -> &'static str;
}

/// Convert syntax tree to HTML deconstruction
pub trait ToHtml {
    fn as_html(&self) -> String;
}