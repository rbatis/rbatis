use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;
use std::sync::{Mutex, RwLock};

use serde_json::json;
use serde_json::Value;

use rbatis_core::db::DriverType;

use crate::ast::ast::RbatisAST;
use crate::ast::node::bind_node::BindNode;
use crate::ast::node::choose_node::ChooseNode;
use crate::ast::node::foreach_node::ForEachNode;
use crate::ast::node::if_node::IfNode;
use crate::ast::node::node_type::NodeType;
use crate::ast::node::otherwise_node::OtherwiseNode;
use crate::ast::node::set_node::SetNode;
use crate::ast::node::string_node::StringNode;
use crate::ast::node::trim_node::TrimNode;
use crate::ast::node::when_node::WhenNode;
use crate::ast::node::where_node::WhereNode;
use crate::engine::parser::parse;
use crate::engine::runtime::RbatisEngine;
use crate::utils::bencher::Bencher;

lazy_static! {
  static ref PY_PARSER_MAP: RwLock<HashMap<String,Vec<NodeType>>> = RwLock::new(HashMap::new());
}

pub struct Py {}


impl Py {
    /// parser and cache py data sql,return an vec node type
    ///编译并且缓存py slq数据，返回node type 数组
    pub fn parse_and_cache(arg: &str) -> Result<Vec<NodeType>, rbatis_core::Error> {
        let rd = PY_PARSER_MAP.try_read();
        if rd.is_err() {
            let nods = Py::parse(arg)?;
            Py::try_cache_into(arg, nods.clone());
            return Ok(nods);
        } else {
            let rd = rd.unwrap();
            let nodes = rd.get(&arg.to_string());
            if nodes.is_some() {
                return Ok(nodes.unwrap().clone());
            } else {
                let nods = Py::parse(arg)?;
                drop(rd);
                Py::try_cache_into(arg, nods.clone());
                return Ok(nods);
            }
        }
    }

    fn try_cache_into(py: &str, arg: Vec<NodeType>) {
        let rd = PY_PARSER_MAP.try_write();
        if rd.is_ok() {
            rd.unwrap().insert(py.to_string(), arg);
        }
    }

    /// parser py string data
    /// 解析py语法
    pub fn parse(arg: &str) -> Result<Vec<NodeType>, rbatis_core::Error> {
        let line_space_map = Py::create_line_space_map(arg);
        let mut pys = vec![];
        let ls = arg.lines();
        let mut skip_line = -1;
        let mut space = -1;
        let mut line = -1;
        for x in ls {
            line += 1;
            if x.is_empty() {
                continue;
            }
            if skip_line != -1 && line <= skip_line {
                continue;
            }

            let count_index = *line_space_map.get(&line).unwrap();
            if space == -1 {
                space = count_index;
            }
            if count_index > space {
                let (child_str, skip) = Py::find_child_str(line, count_index, arg, &line_space_map);
                //println!("child_str: {},{}",skip,child_str.replace("\n",""));
                if skip != -1 {
                    skip_line = skip;
                }
                if !child_str.is_empty() && pys.last_mut().is_some() {
                    let last: &mut NodeType = pys.last_mut().unwrap();
                    let parserd = Py::parse(child_str.as_str())?;
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
                            NodeType::NSet(node) => {
                                node.childs = parserd;
                            }
                            NodeType::NWhere(node) => {
                                node.childs = parserd;
                            }
                            NodeType::NChoose(node) => {
                                for x in &parserd {
                                    match x {
                                        NodeType::NWhen(wnode) => {
                                            if node.when_nodes.is_none() {
                                                node.when_nodes = Some(vec![]);
                                            }
                                            node.when_nodes.as_mut().unwrap().push(x.clone());
                                        }
                                        NodeType::NOtherwise(onode) => {
                                            node.otherwise_node = Some(Box::new(x.clone()));
                                        }
                                        _ => {
                                            return Err(rbatis_core::Error::from("[rbatis] parser node fail,choose node' child must be when and otherwise nodes!: ".to_string() + child_str.as_str()));
                                        }
                                    }
                                }
                            }
                            NodeType::NString(node) => {
                                for x in &parserd {
                                    match x {
                                        parserd => {
                                            pys.push(parserd.clone());
                                        }
                                    }
                                }
                            }
                            _ => {
                                return Err(rbatis_core::Error::from("[rbatis] not support node  type in sql!: ".to_string() + child_str.as_str()));
                            }
                        }
                    }
                }
            }
            if skip_line != -1 && line <= skip_line {
                continue;
            }

            let node = Py::parser_node(x, *line_space_map.get(&line).unwrap() as usize)?;


            pys.push(node);
            //当前node
        }
        return Ok(pys);
    }

    fn parser_node(x: &str, space: usize) -> Result<NodeType, rbatis_core::Error> {
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
                    return Err(rbatis_core::Error::from("[rbatis] parser express fail:".to_string() + trim_x));
                }
                trim_x = trim_x["for ".len()..].trim();
                let in_index = trim_x.find(" in ").unwrap();
                let col = trim_x[in_index + " in ".len()..].trim();
                let item = trim_x[..in_index].trim();
                return Ok(NodeType::NForEach(ForEachNode {
                    childs: vec![],
                    collection: col.to_string(),
                    index: "".to_string(),
                    item: item.to_string(),
                    open: "".to_string(),
                    close: "".to_string(),
                    separator: "".to_string(),
                }));
            } else if trim_x.starts_with("trim ") {
                trim_x = trim_x["trim ".len()..].trim();
                if trim_x.starts_with("'") && trim_x.ends_with("'") {
                    trim_x = trim_x[1..trim_x.len() - 1].trim();
                    return Ok(NodeType::NTrim(TrimNode {
                        childs: vec![],
                        prefix: "".to_string(),
                        suffix: "".to_string(),
                        suffix_overrides: trim_x.to_string(),
                        prefix_overrides: trim_x.to_string(),
                    }));
                } else {
                    return Err(rbatis_core::Error::from("[rbatis] parser express fail:".to_string() + trim_x));
                }
            } else if trim_x.starts_with("choose") {
                trim_x = trim_x["choose".len()..].trim();
                return Ok(NodeType::NChoose(ChooseNode {
                    when_nodes: None,
                    otherwise_node: None,
                }));
            } else if trim_x.starts_with("otherwise") {
                trim_x = trim_x["otherwise".len()..].trim();
                return Ok(NodeType::NOtherwise(OtherwiseNode {
                    childs: vec![],
                }));
            } else if trim_x.starts_with("when ") {
                trim_x = trim_x["when ".len()..].trim();
                return Ok(NodeType::NWhen(WhenNode {
                    childs: vec![],
                    test: trim_x.to_string(),
                }));
            } else if trim_x.starts_with("bind ") {
                trim_x = trim_x["bind ".len()..].trim();
                let name_value: Vec<&str> = trim_x.split(",").collect();
                if name_value.len() != 2 {
                    return Err(rbatis_core::Error::from("[rbatis] parser express fail:".to_string() + trim_x));
                }
                return Ok(NodeType::NBind(BindNode {
                    name: name_value[0].to_owned(),
                    value: name_value[1].to_owned(),
                }));
            } else if trim_x.starts_with("set") {
                trim_x = trim_x["set".len()..].trim();
                return Ok(NodeType::NSet(SetNode {
                    childs: vec![]
                }));
            } else if trim_x.starts_with("where") {
                trim_x = trim_x["where".len()..].trim();
                return Ok(NodeType::NWhere(WhereNode {
                    childs: vec![]
                }));
            } else {
                // unkonw tag
                return Err(rbatis_core::Error::from("[rbatis] unknow tag: ".to_string() + trim_x));
            }
        } else {
            //string,replace space to only one
            let s_node;
            if space <= 1 {
                s_node = StringNode::new(x);
            } else {
                s_node = StringNode::new(x[(space - 1)..].as_ref());
            }
            return Ok(NodeType::NString(s_node));
        }
    }

    fn count_space(arg: &str) -> i32 {
        let cs = arg.chars();
        let mut index = 0;
        for x in cs {
            match x {
                ' ' => {
                    index += 1;
                }
                _ => {
                    break;
                }
            }
        }
        return index;
    }

    ///find_child_str
    fn find_child_str(line_index: i32, space_index: i32, arg: &str, m: &HashMap<i32, i32>) -> (String, i32) {
        let mut result = String::new();
        let mut skip_line = -1;
        let mut line = -1;
        let lines = arg.lines();
        for x in lines {
            line += 1;
            if line >= line_index {
                if *m.get(&line).unwrap() >= space_index {
                    result = result + x + "\n";
                    skip_line = line;
                } else {
                    break;
                }
            }
        }
        let ss = result.as_str();
        return (result, skip_line);
    }

    ///Map<line,space>
    fn create_line_space_map(arg: &str) -> HashMap<i32, i32> {
        let mut m = HashMap::new();
        let lines = arg.lines();
        let mut line = -1;
        for x in lines {
            line += 1;
            let space = Py::count_space(x);
            //dothing
            m.insert(line, space);
        }
        return m;
    }
}


#[test]
pub fn test_py_interpreter_parse() {
    let s = "
    SELECT * FROM biz_activity
    if  name!=null:
      AND delete_flag = #{del}
      AND version = 1
      if  age!=1:
        AND version = 1
      AND version = 1
    AND a = 0
      yes
    for item in ids:
      #{item}
    trim 'AND':
      AND delete_flag = #{del2}
    WHERE id  = '2';";
    //println!("{}", s);
    let pys = Py::parse(s);
    println!("{:?}", pys);
}

#[test]
pub fn test_exec() {
    let s = "SELECT * FROM biz_activity where
    if  name!=null:
      name = #{name}
    AND delete_flag1 = #{del}
    if  age != 1:
       AND age = 1
       if  age != 1:
         AND age = 2
    trim 'AND ':
      AND delete_flag2 = #{del}
    AND ids in (
    trim ',':
      for item in ids:
        #{item},
    )
    choose:
        when age==27:
          AND age = 27
        otherwise:
          AND age = 0
    WHERE id  = 'end';";
    let pys = Py::parse(s).unwrap();
    println!("{:#?}", pys);
    //for x in &pys {
    // println!("{:?}", x.clone());
    //}
    //println!("pys:len:{}", pys.len());


    let mut arg_array = vec![];
    let mut engine = RbatisEngine::new();
    let mut env = json!({
        "name": "1",
        "age": 27,
        "del":1,
        "ids":[1,2,3]
    });
    let r = crate::ast::node::node::do_child_nodes(&DriverType::Mysql, &pys, &mut env, &mut engine, &mut arg_array).unwrap();
    println!("result sql:{}", r.clone());
    println!("arg array:{:?}", arg_array.clone());
}

//cargo.exe test --release --color=always --package rbatis --lib ast::lang::py::bench_exec  --nocapture --exact
#[test]
pub fn bench_exec() {
    let mut b = Bencher::new(1000000);
    let mut sql = "asdfsdaflakagjsda".to_string();
    b.iter_mut(&mut sql, |s| {
        let s = s.ends_with("WHERE")
            || s.ends_with("AND")
            || s.ends_with("OR")
            || s.ends_with("(")
            || s.ends_with(",")
            || s.ends_with("=")
            || s.ends_with("+")
            || s.ends_with("-")
            || s.ends_with("*")
            || s.ends_with("/");
    });
}