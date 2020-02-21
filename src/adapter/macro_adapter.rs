use serde_json::{json, Value};

use crate::ast::lang::py::Py;
use crate::ast::node::node::do_child_nodes;
use crate::engine::node::NodeType;
use crate::engine::runtime::RbatisEngine;
use crate::rbatis::Rbatis;

impl Rbatis {}


// SELECT * FROM biz_activity
// if  name!=null:
//   AND name = #{name}
pub fn parser_sql(source: &str) -> fn(arg: &serde_json::Value) -> (String,Vec<serde_json::Value>) {
    |arg|{
        let arg_map=arg.as_object().unwrap();
        let mut args =vec![];
        let mut sql = String::new();
        //create sql line
        sql = sql + "SELECT * FROM biz_activity WHERE delete_flag = 1";
        //if node
        if arg_map.get("name").is_some(){
            //push args
            let name= arg_map.get("name").unwrap();
            args.push(name.clone());
            //create sql line
            sql = sql + "  AND name = ?" //replace
        }
        return (sql,args);
    }
}

//TODO impl the macro
// macro_rules! sql {
//     () => {};
// }
#[test]
pub fn test_macro_sql_code() {
    let sql_fn=parser_sql("
    SELECT * FROM biz_activity
    if  name!=null:
      AND name = #{name}
    ");
   let (sql,args) = sql_fn(&json!({
      "name":"jons"
    }));
    println!("sql:{}",sql);
    println!("args:{:?}",args);
}
