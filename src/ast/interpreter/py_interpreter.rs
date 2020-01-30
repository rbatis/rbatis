use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;
use std::sync::{Mutex, RwLock};

use serde_json::json;
use serde_json::Value;

use crate::ast::ast::Ast;
use crate::ast::config_holder::ConfigHolder;
use crate::ast::node::foreach_node::ForEachNode;
use crate::ast::node::if_node::IfNode;
use crate::ast::node::node_type::NodeType;
use crate::ast::node::string_node::StringNode;
use crate::ast::node::trim_node::TrimNode;
use crate::engine::parser::parser;
use crate::utils::bencher::Bencher;

lazy_static! {
  static ref ParserMap: Mutex<HashMap<String,Vec<NodeType>>> = Mutex::new(HashMap::new());
}

pub struct PyInterpreter {}


impl PyInterpreter {

    //编译缓存
    pub fn parser_by_cache(arg: &str) -> Result<Vec<NodeType>, String> {
        // RwLock //let ParserMap: Mutex<HashMap<String, Vec<NodeType>>> = Mutex::new(HashMap::new());
        let mut rd = ParserMap.lock().unwrap();
        let nodes = rd.get(&arg.to_string());
        if nodes.is_some() {
            return Ok(nodes.unwrap().clone());
        } else {
            let nods = PyInterpreter::parser(arg)?;
            rd.insert(arg.to_string(), nods.clone());
            return Ok(nods);
        }
    }

    pub fn parser(arg: &str) -> Result<Vec<NodeType>, String> {
        let mut pys = vec![];
        let ls = arg.lines();

        let mut skip_line = -1;

        let mut space_index = -1;
        let mut line_index = -1;
        for x in ls {
            line_index += 1;
            if x.is_empty() {
                continue;
            }
            if skip_line != -1 && line_index <= skip_line {
                continue;
            }

            let count_index = PyInterpreter::count_space(x);
            if space_index == -1 {
                space_index = count_index;
            }
            if count_index > space_index {
                let (child_str, skip) = PyInterpreter::find_child_str(line_index, space_index, arg);
                if skip != -1 {
                    skip_line = skip;
                }
                if !child_str.is_empty() && pys.last_mut().is_some() {
                    let last: &mut NodeType = pys.last_mut().unwrap();
                    let parserd = PyInterpreter::parser(child_str.as_str())?;
                    if !parserd.is_empty() {
                        match last {
                            NodeType::NTrim(node) => {
                                node.childs = parserd;
                            }
                            NodeType::NIf(node) => {
                                node.childs = parserd;
                            }
                            NodeType::NForEach(node) => {
                                node.childs = parserd;
                            }
                            NodeType::NOtherwise(node) => {
                                node.childs = parserd;
                            }
                            NodeType::NWhen(node) => {
                                node.childs = parserd;
                            }
                            NodeType::NInclude(node) => {
                                node.childs = parserd;
                            }
                            NodeType::NSet(node) => {
                                node.childs = parserd;
                            }
                            NodeType::NWhere(node) => {
                                node.childs = parserd;
                            }
                            _ => {}
                        }
                    }
                }
            }
            if skip_line != -1 && line_index <= skip_line {
                continue;
            }
            pys.push(PyInterpreter::parser_node(x)?);
            //当前node
        }
        return Ok(pys);
    }
    fn parser_node(x: &str) -> Result<NodeType, String> {
        let mut trim_x = x.trim();
        if trim_x.ends_with(":") {
            trim_x = trim_x[0..trim_x.len() - 1].trim();
            if trim_x.starts_with("if ") {
                trim_x = trim_x["if ".len()..].trim();
                return Ok(NodeType::NIf(IfNode {
                    childs: vec![],
                    test: trim_x.to_string(),
                }));
            } else if trim_x.starts_with("for ") {
                if !trim_x.contains(" in ") {
                    return Err("[rbatis] parser express fail:".to_string() + trim_x);
                }
                return Ok(NodeType::NForEach(ForEachNode {
                    childs: vec![],
                    collection: trim_x[trim_x.find(" in ").unwrap()..].trim().to_string(),
                    index: "".to_string(),
                    item: trim_x[..trim_x.find(" in ").unwrap()].trim().to_string(),
                    open: "".to_string(),
                    close: "".to_string(),
                    separator: "".to_string(),
                }));
            } else if trim_x.starts_with("trim ") {
                trim_x = trim_x["trim ".len()..].trim();
                if trim_x.starts_with("'") && trim_x.ends_with("'") {
                    return Ok(NodeType::NTrim(TrimNode {
                        childs: vec![],
                        prefix: "".to_string(),
                        suffix: "".to_string(),
                        suffix_overrides: "".to_string(),
                        prefix_overrides: "".to_string(),
                    }));
                } else {
                    return Err("[rbatis] parser express fail:".to_string() + trim_x);
                }
            } else {
                // unkonw tag
                return Err("[rbatis] unknow tag with:".to_string() + trim_x);
            }
        } else {
            //string
            return Ok(NodeType::NString(StringNode::new(x)));
        }
    }

    pub fn count_space(arg: &str) -> i32 {
        let cs = arg.chars();
        let mut index = 0;
        for x in cs {
            match x {
                ' ' => {
                    index += 1;
                }
                _ => {}
            }
        }
        return index;
    }

    ///find_child_str
    fn find_child_str(line_index: i32, space_index: i32, arg: &str) -> (String, i32) {
        let mut result = String::new();
        let mut skip_line = -1;
        let mut index = -1;
        let lines = arg.lines();
        for x in lines {
            index += 1;
            if index >= line_index {
                if PyInterpreter::count_space(x) >= space_index {
                    result = result + x + "\n";
                    skip_line = index;
                } else {
                    break;
                }
            }
        }
        return (result, skip_line);
    }
}


#[test]
pub fn test_py_interpreter_parser() {
    let s = "
    select * from biz_activity
    if  name!=null:
      and delete_flag = #{del}
      and version = 1
      if  age!=1:
        and version = 1
      and version = 1
    and a = 0
    for item in ids:
      #{item}
    trim 'and':
      and delete_flag = #{del2}
    where id  = '2';";
    //println!("{}", s);
    let pys = PyInterpreter::parser(s);
    println!("{:?}", pys);
}

#[test]
pub fn test_exec() {
    let s = "
    select * from biz_activity
    if  name!=null:
      name = #{name}
    and delete_flag1 = #{del}
    if  age!=1:
       and age = 2
       if  age!=1:
         and age = 3
    trim 'and ':
      and delete_flag2 = #{del}
    where id  = '2';";

    let pys = PyInterpreter::parser(s).unwrap();
    println!("{:?}", pys);
    println!("pys:len:{}", pys.len());


    let mut arg_array = vec![];
    let mut holder = ConfigHolder::new();
    let mut env = json!({
        "name": "1",
        "age": 27,
        "del":1
    });
    let r = crate::ast::node::node::do_child_nodes(&pys, &mut env, &mut holder, &mut arg_array).unwrap();
    println!("{}", r);
}

//cargo test --release --color=always --package rbatis --lib ast::interpreter::py_interpreter::bench_exec --all-features -- --nocapture --exact
#[test]
pub fn bench_exec() {
    let mut b = Bencher::new(1000000);
    b.iter(|| {
        let s = "
    select * from biz_activity
    if  name!=null:
      name = #{name}
    and delete_flag1 = #{del}
    if  age!=1:
       and age = 2
       if  age!=1:
         and age = 3
    trim 'and ':
      and delete_flag2 = #{del}
    where id  = '2';";
        let pys = PyInterpreter::parser_by_cache(s);
    });
}