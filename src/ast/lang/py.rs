use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;
use std::sync::{Mutex, RwLock};

use serde_json::json;
use serde_json::Value;

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
use crate::core::Error;
use crate::engine::parser::parse;

/// Py lang,make sure Send+Sync
pub struct Py {
    pub cache: RwLock<HashMap<String, Vec<NodeType>>>
}

impl Py {
    /// parser and cache py data sql,return an vec node type
    pub fn parse_and_cache(&self, arg: &str) -> Result<Vec<NodeType>, crate::core::Error> {
        let rd = self.cache.try_read();
        if rd.is_err() {
            let nods = Py::parse(arg)?;
            self.try_cache_into(arg, nods.clone());
            return Ok(nods);
        } else {
            let rd = rd.unwrap();
            let nodes = rd.get(&arg.to_string());
            if nodes.is_some() {
                return Ok(nodes.unwrap().clone());
            } else {
                let nods = Py::parse(arg)?;
                drop(rd);
                self.try_cache_into(arg, nods.clone());
                return Ok(nods);
            }
        }
    }

    fn try_cache_into(&self, py: &str, arg: Vec<NodeType>) {
        let rd = self.cache.try_write();
        if rd.is_ok() {
            rd.unwrap().insert(py.to_string(), arg);
        }
    }

    /// parser py string data
    pub fn parse(arg: &str) -> Result<Vec<NodeType>, crate::core::Error> {
        let line_space_map = Py::create_line_space_map(arg);
        let mut main_node = vec![];
        let ls = arg.lines();
        let mut space = -1;
        let mut line = -1;
        let mut skip = -1;
        for x in ls {
            line += 1;
            if x.is_empty() || (skip != -1 && line <= skip) {
                continue;
            }
            let count_index = *line_space_map.get(&line).unwrap();
            if space == -1 {
                space = count_index;
            }
            let (child_str, do_skip) = Py::find_child_str(line, count_index, arg, &line_space_map);
            if do_skip != -1 && do_skip >= skip {
                skip = do_skip;
            }
            let parserd;
            if !child_str.is_empty() {
                parserd = Py::parse(child_str.as_str())?;
            } else {
                parserd = vec![];
            }
            Py::parse_node(&mut main_node, x, *line_space_map.get(&line).unwrap() as usize, parserd)?;
        }
        return Ok(main_node);
    }

    fn parse_node(main_node: &mut Vec<NodeType>, x: &str, space: usize, childs: Vec<NodeType>) -> Result<(), crate::core::Error> {
        let mut trim_x = x.trim();
        if trim_x.starts_with("//") {
            return Ok(());
        }
        if trim_x.ends_with(":") {
            trim_x = trim_x[0..trim_x.len() - 1].trim();
            if trim_x.starts_with("if ") {
                trim_x = trim_x["if ".len()..].trim();
                main_node.push(NodeType::NIf(IfNode {
                    childs: childs,
                    test: trim_x.to_string(),
                }));
                return Ok(());
            } else if trim_x.starts_with("for ") {
                if !trim_x.contains("in ") {
                    return Err(crate::core::Error::from("[rbatis] parser express fail:".to_string() + x));
                }
                trim_x = trim_x["for ".len()..].trim();
                let in_index = trim_x.find("in ").unwrap();
                let col = trim_x[in_index + "in ".len()..].trim();
                let mut item = trim_x[..in_index].trim();
                let mut index = "";
                if item.contains(",") {
                    let items: Vec<&str> = item.split(",").collect();
                    if items.len() != 2 {
                        return Err(crate::core::Error::from(format!("[rbatis][py] parse fail 'for ,' must be 'for arg1,arg2 in ...',value:'{}'", x)));
                    }
                    index = items[0];
                    item = items[1];
                }
                main_node.push(NodeType::NForEach(ForEachNode {
                    childs: childs,
                    collection: col.to_string(),
                    index: index.to_string(),
                    item: item.to_string(),
                    open: "".to_string(),
                    close: "".to_string(),
                    separator: "".to_string(),
                }));
                return Ok(());
            } else if trim_x.starts_with("trim ") {
                trim_x = trim_x["trim ".len()..].trim();
                if trim_x.starts_with("'") && trim_x.ends_with("'") {
                    trim_x = trim_x[1..trim_x.len() - 1].trim();
                    main_node.push(NodeType::NTrim(TrimNode {
                        childs: childs,
                        prefix: "".to_string(),
                        suffix: "".to_string(),
                        suffix_overrides: trim_x.to_string(),
                        prefix_overrides: trim_x.to_string(),
                    }));
                    return Ok(());
                } else {
                    return Err(crate::core::Error::from(format!("[rbatis] express trim value must be string value, for example:  trim 'value',error express: {}", x)));
                }
            } else if trim_x.starts_with("choose") {
                trim_x = trim_x["choose".len()..].trim();
                let mut node = ChooseNode {
                    when_nodes: None,
                    otherwise_node: None,
                };
                for x in &childs {
                    match x {
                        NodeType::NWhen(_) => {
                            if node.when_nodes.is_none() {
                                node.when_nodes = Some(vec![]);
                            }
                            node.when_nodes.as_mut().unwrap().push(x.clone());
                        }
                        NodeType::NOtherwise(_) => {
                            node.otherwise_node = Some(Box::new(x.clone()));
                        }
                        _ => {
                            return Err(crate::core::Error::from("[rbatis] parser node fail,choose node' child must be when and otherwise nodes!".to_string()));
                        }
                    }
                }
                main_node.push(NodeType::NChoose(node));
                return Ok(());
            } else if trim_x.starts_with("otherwise") {
                trim_x = trim_x["otherwise".len()..].trim();
                main_node.push(NodeType::NOtherwise(OtherwiseNode {
                    childs: childs,
                }));
                return Ok(());
            } else if trim_x.starts_with("when ") {
                trim_x = trim_x["when ".len()..].trim();
                main_node.push(NodeType::NWhen(WhenNode {
                    childs: childs,
                    test: trim_x.to_string(),
                }));
                return Ok(());
            } else if trim_x.starts_with("bind ") {
                trim_x = trim_x["bind ".len()..].trim();
                let name_value: Vec<&str> = trim_x.split("=").collect();
                if name_value.len() != 2 {
                    return Err(crate::core::Error::from("[rbatis] parser bind express fail:".to_string() + x));
                }
                main_node.push(NodeType::NBind(BindNode {
                    name: name_value[0].to_owned(),
                    value: name_value[1].to_owned(),
                }));
                return Ok(());
            } else if trim_x.starts_with("set") {
                trim_x = trim_x["set".len()..].trim();
                main_node.push(NodeType::NSet(SetNode {
                    childs: childs
                }));
                return Ok(());
            } else if trim_x.starts_with("where") {
                trim_x = trim_x["where".len()..].trim();
                main_node.push(NodeType::NWhere(WhereNode {
                    childs: childs
                }));
                return Ok(());
            } else {
                // unkonw tag
                return Err(crate::core::Error::from("[rbatis] unknow tag: ".to_string() + x));
            }
        } else {
            //string,replace space to only one
            let mut data = x.to_owned();
            if space <= 1 {
                data = x.to_string();
            } else {
                data = x[(space - 1)..].to_string();
            }
            main_node.push(NodeType::NString(StringNode::new(&data)));
            for x in childs {
                main_node.push(x);
            }
            return Ok(());
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
            if line > line_index {
                let cached_space = *m.get(&line).unwrap();
                if cached_space > space_index {
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

#[cfg(test)]
mod test {
    use crate::ast::lang::py::Py;
    use crate::core::db::DriverType;
    use crate::engine::runtime::RbatisEngine;

    #[test]
    pub fn test_py_interpreter_parse() {
        let s = "
    SELECT * FROM biz_activity
    //判断名称是否null
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
    for index,item in ids:
      #{item}
    trim 'AND':
      AND delete_flag = #{del2}
    WHERE id  = '2';";
        //println!("{}", s);
        let pys = Py::parse(s);
        match pys {
            Ok(v) => {
                println!("{:?}", v);
            }
            Err(e) => {
                println!("{:?}", e.to_string());
            }
        }
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


    #[test]
    fn bind_test() {
        let mut env = json!({
        "name": "1",
        "age": 27,
        "del":1,
        "ids":[1,2,3]
    });

        let s = "
                       bind name=1+0:
                       if 1==1:
                              select 2
                       select ${name}
                         select 3
                           select 4
                       (
                       trim ',':
                         for item in ids:
                             ${item},
                       )
                       ";

        let pys = Py::parse(s).unwrap();
        println!("{:#?}", pys);

        let mut arg_array = vec![];
        let mut engine = RbatisEngine::new();
        let r = crate::ast::node::node::do_child_nodes(&DriverType::Mysql, &pys, &mut env, &mut engine, &mut arg_array).unwrap();
        println!("result: {}", &r);
        println!("arg: {:?}", arg_array.clone());
    }

    #[test]
    fn test_find() {
        let s = "
                       bind name=1+0:
                       if 1==1:
                              select 2
                       select ${name}
                         select 3
                           select 4
                       ";
        let line_space_map = Py::create_line_space_map(s);
        println!("m:{:#?}", &line_space_map);
        let (child_str, do_skip) = Py::find_child_str(4,
                                                      "                       ".len() as i32,
                                                      s,
                                                      &line_space_map);

        println!("child_str: \n{}", &child_str);
        println!("skip: {}", do_skip);
    }
}