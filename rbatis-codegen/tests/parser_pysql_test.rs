use rbatis_codegen::codegen::syntax_tree_pysql::NodeType;
use rbatis_codegen::codegen::parser_pysql::ParsePySql;

// 测试基本的 SQL 语句解析
#[test]
fn test_parse_basic_sql() {
    let sql = "select * from user";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    assert_eq!(nodes.len(), 1);
    
    match &nodes[0] {
        NodeType::NString(node) => {
            assert_eq!(node.value, "select * from user");
        }
        _ => panic!("Expected StringNode, got {:?}", nodes[0]),
    }
}

// 测试 if 语句的解析
#[test]
fn test_parse_if_node() {
    let sql = "select * from user\nif id != null:\n  where id = #{id}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    assert_eq!(nodes.len(), 2);
    
    match &nodes[0] {
        NodeType::NString(node) => {
            assert_eq!(node.value, "select * from user");
        }
        _ => panic!("Expected StringNode, got {:?}", nodes[0]),
    }
    
    match &nodes[1] {
        NodeType::NIf(node) => {
            assert_eq!(node.test, "id != null");
            assert_eq!(node.childs.len(), 1);
            
            match &node.childs[0] {
                NodeType::NString(string_node) => {
                    assert_eq!(string_node.value, "where id = #{id}");
                }
                _ => panic!("Expected StringNode, got {:?}", node.childs[0]),
            }
        }
        _ => panic!("Expected IfNode, got {:?}", nodes[1]),
    }
}

// 测试 for 循环解析
#[test]
fn test_parse_foreach_node() {
    let sql = "select * from user\nfor item in items:\n  #{item}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    assert_eq!(nodes.len(), 2);
    
    match &nodes[1] {
        NodeType::NForEach(node) => {
            assert_eq!(node.collection, "items");
            assert_eq!(node.item, "item");
            assert_eq!(node.index, "");
        }
        _ => panic!("Expected ForEachNode, got {:?}", nodes[1]),
    }
}

// 测试带索引的 for 循环解析
#[test]
fn test_parse_foreach_with_index() {
    let sql = "select * from user\nfor key,item in items:\n  (#{key}, #{item})";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[1] {
        NodeType::NForEach(node) => {
            assert_eq!(node.collection, "items");
            assert_eq!(node.item, "item");
            assert_eq!(node.index, "key");
        }
        _ => panic!("Expected ForEachNode, got {:?}", nodes[1]),
    }
}

// 测试 where 节点解析
#[test]
fn test_parse_where_node() {
    let sql = "select * from user\nwhere:\n  if id != null:\n    and id = #{id}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[1] {
        NodeType::NWhere(node) => {
            assert_eq!(node.childs.len(), 1);
            
            match &node.childs[0] {
                NodeType::NIf(_) => {}
                _ => panic!("Expected IfNode, got {:?}", node.childs[0]),
            }
        }
        _ => panic!("Expected WhereNode, got {:?}", nodes[1]),
    }
}

// 测试 trim 节点解析 - 简单模式
#[test]
fn test_parse_trim_node_simple() {
    let sql = "select * from user\ntrim ',':\n  id = #{id},\n  name = #{name},";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[1] {
        NodeType::NTrim(node) => {
            assert_eq!(node.start, ",");
            assert_eq!(node.end, ",");
        }
        _ => panic!("Expected TrimNode, got {:?}", nodes[1]),
    }
}

// 测试 choose-when-otherwise 节点解析
#[test]
fn test_parse_choose_when_otherwise() {
    let sql = "select * from user\nchoose:\n  when id != null:\n    where id = #{id}\n  otherwise:\n    where 1=1";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[1] {
        NodeType::NChoose(node) => {
            assert_eq!(node.when_nodes.len(), 1);
            assert!(node.otherwise_node.is_some());
            
            match &node.when_nodes[0] {
                NodeType::NWhen(when_node) => {
                    assert_eq!(when_node.test, "id != null");
                }
                _ => panic!("Expected WhenNode, got {:?}", node.when_nodes[0]),
            }
            
            let otherwise = node.otherwise_node.as_ref().unwrap();
            match &**otherwise {
                NodeType::NOtherwise(_) => {}
                _ => panic!("Expected OtherwiseNode"),
            }
        }
        _ => panic!("Expected ChooseNode, got {:?}", nodes[1]),
    }
}

// 测试 bind 节点解析，必须包含在另一条语句内
#[test]
fn test_parse_bind_node() {
    let sql = "for item in items:\n  bind name = 'test':\n    #{item}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NForEach(node) => {
            match &node.childs[0] {
                NodeType::NBind(bind_node) => {
                    assert_eq!(bind_node.name, "name");
                    assert_eq!(bind_node.value, "'test'");
                }
                _ => panic!("Expected BindNode, got {:?}", node.childs[0]),
            }
        }
        _ => panic!("Expected ForEachNode, got {:?}", nodes[0]),
    }
}

// 测试 break 节点解析，必须包含在 for 循环内
#[test]
fn test_parse_break_node() {
    let sql = "for item in items:\n  if item == null:\n    break:";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NForEach(node) => {
            match &node.childs[0] {
                NodeType::NIf(if_node) => {
                    match &if_node.childs[0] {
                        NodeType::NBreak(_) => {}
                        _ => panic!("Expected BreakNode, got {:?}", if_node.childs[0]),
                    }
                }
                _ => panic!("Expected IfNode, got {:?}", node.childs[0]),
            }
        }
        _ => panic!("Expected ForEachNode, got {:?}", nodes[0]),
    }
}

// 测试 continue 节点解析，必须包含在 for 循环内
#[test]
fn test_parse_continue_node() {
    let sql = "for item in items:\n  if item == 0:\n    continue:";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NForEach(node) => {
            match &node.childs[0] {
                NodeType::NIf(if_node) => {
                    match &if_node.childs[0] {
                        NodeType::NContinue(_) => {}
                        _ => panic!("Expected ContinueNode, got {:?}", if_node.childs[0]),
                    }
                }
                _ => panic!("Expected IfNode, got {:?}", node.childs[0]),
            }
        }
        _ => panic!("Expected ForEachNode, got {:?}", nodes[0]),
    }
}

// 测试 SQL 节点解析
#[test]
fn test_parse_sql_node() {
    let sql = "sql id='userColumns':\n  id, name, age";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NSql(node) => {
            assert_eq!(node.id, "userColumns");
            assert_eq!(node.childs.len(), 1);
            
            match &node.childs[0] {
                NodeType::NString(string_node) => {
                    assert_eq!(string_node.value, "id, name, age");
                }
                _ => panic!("Expected StringNode, got {:?}", node.childs[0]),
            }
        }
        _ => panic!("Expected SqlNode, got {:?}", nodes[0]),
    }
}

// 测试 SQL 节点引号处理
#[test]
fn test_parse_sql_node_quotes() {
    // 单引号
    let sql = "sql id='userColumns':\n  id, name, age";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NSql(node) => {
            assert_eq!(node.id, "userColumns");
        }
        _ => panic!("Expected SqlNode, got {:?}", nodes[0]),
    }
    
    // 双引号
    let sql = "sql id=\"userColumns\":\n  id, name, age";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NSql(node) => {
            assert_eq!(node.id, "userColumns");
        }
        _ => panic!("Expected SqlNode, got {:?}", nodes[0]),
    }
}

// 测试 SQL 节点语法错误
#[test]
fn test_parse_sql_node_errors() {
    // 缺少 id 参数
    let sql = "sql :\n  id, name, age";
    let result = NodeType::parse_pysql(sql);
    assert!(result.is_err());
    
    // 缺少引号
    let sql = "sql id=userColumns:\n  id, name, age";
    let result = NodeType::parse_pysql(sql);
    assert!(result.is_err());
}

// 测试复杂嵌套结构，包含多种节点类型
#[test]
fn test_parse_complex_structure() {
    let sql = "select\n\
              sql id='columns':\n\
                id, name, age\n\
              from user\n\
              where:\n\
                if id != null:\n\
                  and id = #{id}\n\
                if name != null:\n\
                  and name like #{name}\n\
              for item in items:\n\
                #{item}";
    
    let nodes = NodeType::parse_pysql(sql).unwrap();
    assert!(nodes.len() > 2);
    
    // 验证第一个是 StringNode
    match &nodes[0] {
        NodeType::NString(_) => {}
        _ => panic!("Expected StringNode, got {:?}", nodes[0]),
    }
    
    // 验证有 SqlNode
    let has_sql_node = nodes.iter().any(|node| {
        matches!(node, NodeType::NSql(_))
    });
    assert!(has_sql_node, "Expected to find a SqlNode in the parsed result");
    
    // 验证有 WhereNode
    let has_where_node = nodes.iter().any(|node| {
        matches!(node, NodeType::NWhere(_))
    });
    assert!(has_where_node, "Expected to find a WhereNode in the parsed result");
    
    // 验证有 ForEachNode
    let has_foreach_node = nodes.iter().any(|node| {
        matches!(node, NodeType::NForEach(_))
    });
    assert!(has_foreach_node, "Expected to find a ForEachNode in the parsed result");
} 