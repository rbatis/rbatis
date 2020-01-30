use std::ops::Index;

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

#[derive(Clone, Debug)]
pub struct Py {
    pub tag: &'static str,
    pub props: String,
    pub childs: Option<Vec<Py>>,
}

pub trait ToNodeType {
    fn to_node_type(&self) -> Vec<NodeType>;
}


impl ToNodeType for Vec<Py> {
    fn to_node_type(&self) -> Vec<NodeType> {
        let mut result = vec![];
        for x in self {
            let nt = x.to_node_type();
            result.push(nt);
        }
        return result;
    }
}


impl Py {
    pub fn to_node_type(&self) -> NodeType {
        match self.tag {
            "string" => {
                return NodeType::NString(StringNode::new(self.props.as_str()));
            }
            "if" => {
                let mut childs = vec![];
                if self.childs.is_some() {
                    childs = self.childs.as_ref().unwrap().to_node_type();
                }
                return NodeType::NIf(IfNode {
                    childs,
                    test: self.props.clone(),
                });
            }
            "trim" => {
                let mut childs = vec![];
                if self.childs.is_some() {
                    childs = self.childs.as_ref().unwrap().to_node_type();
                }
                let splits: Vec<&str> = self.props.split(",").collect();
                let mut suffix_overrides = "".to_string();
                let mut prefix_overrides = "".to_string();
                if splits.len() == 1 {
                    prefix_overrides = splits[0].to_string();
                    if prefix_overrides.starts_with("'") && prefix_overrides.starts_with("'") {
                        prefix_overrides = prefix_overrides[1..prefix_overrides.len() - 1].to_string();
                    }
                }
                if splits.len() == 2 {
                    suffix_overrides = splits[1].to_string();
                    if suffix_overrides.starts_with("'") && suffix_overrides.starts_with("'") {
                        suffix_overrides = suffix_overrides[1..suffix_overrides.len() - 1].to_string();
                    }
                }
                return NodeType::NTrim(TrimNode {
                    childs,
                    prefix: "".to_string(),
                    suffix: "".to_string(),
                    suffix_overrides: suffix_overrides,
                    prefix_overrides: prefix_overrides,
                });
            }
            "for" => {
                let mut childs = vec![];
                if self.childs.is_some() {
                    childs = self.childs.as_ref().unwrap().to_node_type();
                }
                //for item in ids:

                return NodeType::NForEach(ForEachNode {
                    childs,
                    collection: "".to_string(),
                    index: "".to_string(),
                    item: "".to_string(),
                    open: "".to_string(),
                    close: "".to_string(),
                    separator: "".to_string(),
                });
            }
            _ => {
                return NodeType::Null;
            }
        }
    }
}

pub struct PyInterpreter {}

impl PyInterpreter {
    pub fn parser(&self, arg: &str) -> Vec<Py> {
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

            let count_index = self.count_space(x);
            if space_index == -1 {
                space_index = count_index;
            }
            if count_index > space_index {
                let (child_str, skip) = self.find_child_str(line_index, space_index, arg);
                println!("child_str:{},skip:{}", child_str.replace("\n", "").trim(), skip);
                if skip != -1 {
                    skip_line = skip;
                }
                if !child_str.is_empty() && pys.last_mut().is_some() {
                    let last: &mut Py = pys.last_mut().unwrap();
                    let parserd = self.parser(child_str.as_str());
                    if !parserd.is_empty() {
                        last.childs = Some(parserd);
                    }
                }
            }
            if skip_line != -1 && line_index <= skip_line {
                continue;
            }
            pys.push(self.parser_node(x));
            //当前node
        }
        return pys;
    }
    pub fn parser_node(&self, x: &str) -> Py {
        let trim_x = x.trim();
        if trim_x.ends_with(":") {
            if trim_x.starts_with("if ") {
                return Py {
                    tag: "if",
                    props: trim_x["if ".len()..trim_x.len() - 1].trim().to_string(),
                    childs: None,
                };
            } else if trim_x.starts_with("for ") {
                return Py {
                    tag: "for",
                    props: trim_x["for ".len()..trim_x.len() - 1].trim().to_string(),
                    childs: None,
                };
            } else if trim_x.starts_with("trim ") {
                return Py {
                    tag: "trim",
                    props: trim_x["trim ".len()..trim_x.len() - 1].trim().to_string(),
                    childs: None,
                };
            } else {
                // unkonw tag
                return Py {
                    tag: "unkonw",
                    props: trim_x.to_string(),
                    childs: None,
                };
            }
        } else {
            //string
            return Py {
                tag: "string",
                props: " ".to_string() + x.trim(),
                childs: None,
            };
        }
    }

    pub fn count_space(&self, arg: &str) -> i32 {
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
    fn find_child_str(&self, line_index: i32, space_index: i32, arg: &str) -> (String, i32) {
        let mut result = String::new();
        let mut skip_line = -1;
        let mut index = -1;
        let lines = arg.lines();
        for x in lines {
            index += 1;
            if index >= line_index {
                if self.count_space(x) >= space_index {
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

    let p = PyInterpreter {};
    let pys = p.parser(s);
    println!("{:?}", pys);

    let nts = pys.to_node_type();
    println!("{:?}", nts);
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
    trim 'and ':
      and delete_flag2 = #{del}
    where id  = '2';";

    let p = PyInterpreter {};
    let pys = p.parser(s);
    println!("{:?}", pys);
    println!("pys:len:{}", pys.len());


    let nts = pys.to_node_type();
    println!("{:?}", nts);
    println!("nts:len:{}", nts.len());

    let mut arg_array = vec![];
    let mut holder = ConfigHolder::new();
    let mut env = json!({
        "name": "1",
        "age": 27,
        "del":1
    });
    let r = crate::ast::node::node::do_child_nodes(&nts, &mut env, &mut holder, &mut arg_array).unwrap();
    println!("{}", r);
}