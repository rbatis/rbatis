use rbatis_codegen::codegen::syntax_tree_pysql::NodeType;
use rbatis_codegen::codegen::parser_pysql::ParsePySql;

/// 测试所有 pysql 节点的基本解析功能

// StringNode 测试
#[test]
fn test_string_node() {
    // 基本字符串
    let sql = "SELECT * FROM users";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    assert_eq!(nodes.len(), 1);
    
    match &nodes[0] {
        NodeType::NString(node) => {
            assert_eq!(node.value, "SELECT * FROM users");
        }
        _ => panic!("Expected StringNode, got {:?}", nodes[0]),
    }
    
    // 使用反引号保留空格
    let sql = "`   SELECT   column1,    column2   FROM   table   `";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NString(node) => {
            assert_eq!(node.value, "`   SELECT   column1,    column2   FROM   table   `");
        }
        _ => panic!("Expected StringNode, got {:?}", nodes[0]),
    }
}

// IfNode 测试
#[test]
fn test_if_node() {
    // 基本 if 语句
    let sql = "if id != null:\n  WHERE id = #{id}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NIf(node) => {
            assert_eq!(node.test, "id != null");
            assert_eq!(node.childs.len(), 1);
            
            match &node.childs[0] {
                NodeType::NString(string_node) => {
                    assert_eq!(string_node.value, "WHERE id = #{id}");
                }
                _ => panic!("Expected StringNode in IfNode.childs"),
            }
        }
        _ => panic!("Expected IfNode, got {:?}", nodes[0]),
    }
    
    // 嵌套 if 语句
    let sql = "if id != null:\n  WHERE id = #{id}\n  if name != null:\n    AND name = #{name}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NIf(node) => {
            assert_eq!(node.test, "id != null");
            assert_eq!(node.childs.len(), 2);
            
            match &node.childs[1] {
                NodeType::NIf(inner_if) => {
                    assert_eq!(inner_if.test, "name != null");
                }
                _ => panic!("Expected nested IfNode in IfNode.childs"),
            }
        }
        _ => panic!("Expected IfNode, got {:?}", nodes[0]),
    }
}

// TrimNode 测试
#[test]
fn test_trim_node() {
    // 简单的 trim 测试，使用单个值
    // 注意：单引号内的内容是前缀和后缀
    let sql = "trim ' ':\n  id = #{id}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NTrim(node) => {
            assert_eq!(node.start, " ");
            assert_eq!(node.end, " ");
            assert_eq!(node.childs.len(), 1);
        }
        _ => panic!("Expected TrimNode, got {:?}", nodes[0]),
    }
}

// ForEachNode 测试
#[test]
fn test_foreach_node() {
    // 基本 for 循环
    let sql = "for item in items:\n  #{item}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NForEach(node) => {
            assert_eq!(node.collection, "items");
            assert_eq!(node.item, "item");
            assert_eq!(node.index, "");
            assert_eq!(node.childs.len(), 1);
        }
        _ => panic!("Expected ForEachNode, got {:?}", nodes[0]),
    }
    
    // 带索引的 for 循环
    let sql = "for idx,item in items:\n  #{idx}:#{item}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NForEach(node) => {
            assert_eq!(node.collection, "items");
            assert_eq!(node.item, "item");
            assert_eq!(node.index, "idx");
        }
        _ => panic!("Expected ForEachNode, got {:?}", nodes[0]),
    }
}

// ChooseNode, WhenNode, OtherwiseNode 测试
#[test]
fn test_choose_when_otherwise_nodes() {
    let sql = "choose:\n  when id != null:\n    WHERE id = #{id}\n  when name != null:\n    WHERE name = #{name}\n  otherwise:\n    WHERE 1=1";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NChoose(node) => {
            assert_eq!(node.when_nodes.len(), 2);
            assert!(node.otherwise_node.is_some());
            
            // 检查 when 节点
            match &node.when_nodes[0] {
                NodeType::NWhen(when_node) => {
                    assert_eq!(when_node.test, "id != null");
                    assert_eq!(when_node.childs.len(), 1);
                }
                _ => panic!("Expected WhenNode"),
            }
            
            // 检查 otherwise 节点
            let otherwise = node.otherwise_node.as_ref().unwrap();
            match &**otherwise {
                NodeType::NOtherwise(otherwise_node) => {
                    assert_eq!(otherwise_node.childs.len(), 1);
                }
                _ => panic!("Expected OtherwiseNode"),
            }
        }
        _ => panic!("Expected ChooseNode, got {:?}", nodes[0]),
    }
    
    // 使用下划线代替 otherwise
    let sql = "choose:\n  when id != null:\n    WHERE id = #{id}\n  _:\n    WHERE 1=1";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NChoose(node) => {
            assert!(node.otherwise_node.is_some());
        }
        _ => panic!("Expected ChooseNode, got {:?}", nodes[0]),
    }
}

// BindNode 测试
#[test]
fn test_bind_node() {
    let sql = "bind name = 'value':\n  WHERE name = #{name}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NBind(node) => {
            assert_eq!(node.name, "name");
            assert_eq!(node.value, "'value'");
        }
        _ => panic!("Expected BindNode, got {:?}", nodes[0]),
    }
    
    // 测试 let 语法
    let sql = "let name = 'value':\n  WHERE name = #{name}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NBind(node) => {
            assert_eq!(node.name, "name");
            assert_eq!(node.value, "'value'");
        }
        _ => panic!("Expected BindNode, got {:?}", nodes[0]),
    }
}

// SetNode 测试
#[test]
fn test_set_node() {
    let sql = 
"set:
  if name != null:
     name = #{name},
  if age != null:
     age = #{age}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    match &nodes[0] {
        NodeType::NSet(node) => {
            assert_eq!(node.childs.len(), 2);
            
            // 检查子节点是否为 IfNode
            match &node.childs[0] {
                NodeType::NIf(_) => {}
                _ => panic!("Expected IfNode in SetNode.childs"),
            }
        }
        _ => panic!("Expected SetNode, got {:?}", nodes[0]),
    }
}

#[test]
fn test_set_node_collection() {
    let sql =
"set collection = 'collection', skips= 'id':
  if name != null:
     name = #{name},
  if age != null:
     age = #{age}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    match &nodes[0] {
        NodeType::NSet(node) => {
            assert_eq!(node.collection, "collection");
            assert_eq!(node.skips, "id");
            assert_eq!(node.childs.len(), 2);
            // 检查子节点是否为 IfNode
            match &node.childs[0] {
                NodeType::NIf(_) => {}
                _ => panic!("Expected IfNode in SetNode.childs"),
            }
        }
        _ => panic!("Expected SetNode, got {:?}", nodes[0]),
    }
}

// WhereNode 测试
#[test]
fn test_where_node() {
    let sql = "where:\n  if id != null:\n    AND id = #{id}\n  if name != null:\n    AND name = #{name}";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NWhere(node) => {
            assert_eq!(node.childs.len(), 2);
            
            // 检查子节点是否为 IfNode
            match &node.childs[0] {
                NodeType::NIf(_) => {}
                _ => panic!("Expected IfNode in WhereNode.childs"),
            }
        }
        _ => panic!("Expected WhereNode, got {:?}", nodes[0]),
    }
}

// BreakNode 测试
#[test]
fn test_break_node() {
    let sql = "for item in items:\n  if item == null:\n    break:";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NForEach(node) => {
            match &node.childs[0] {
                NodeType::NIf(if_node) => {
                    match &if_node.childs[0] {
                        NodeType::NBreak(_) => {}
                        _ => panic!("Expected BreakNode in IfNode.childs"),
                    }
                }
                _ => panic!("Expected IfNode in ForEachNode.childs"),
            }
        }
        _ => panic!("Expected ForEachNode, got {:?}", nodes[0]),
    }
}

// ContinueNode 测试
#[test]
fn test_continue_node() {
    let sql = "for item in items:\n  if item == 0:\n    continue:";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NForEach(node) => {
            match &node.childs[0] {
                NodeType::NIf(if_node) => {
                    match &if_node.childs[0] {
                        NodeType::NContinue(_) => {}
                        _ => panic!("Expected ContinueNode in IfNode.childs"),
                    }
                }
                _ => panic!("Expected IfNode in ForEachNode.childs"),
            }
        }
        _ => panic!("Expected ForEachNode, got {:?}", nodes[0]),
    }
}

// SqlNode 测试
#[test]
fn test_sql_node() {
    let sql = "sql id='userColumns':\n  id, name, age";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NSql(node) => {
            assert_eq!(node.id, "userColumns");
            assert_eq!(node.childs.len(), 1);
        }
        _ => panic!("Expected SqlNode, got {:?}", nodes[0]),
    }
    
    // 测试双引号
    let sql = "sql id=\"userColumns\":\n  id, name, age";
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    match &nodes[0] {
        NodeType::NSql(node) => {
            assert_eq!(node.id, "userColumns");
        }
        _ => panic!("Expected SqlNode, got {:?}", nodes[0]),
    }
    
    // 测试 SQL 节点的错误处理
    let sql = "sql id=userColumns:\n  id, name, age"; // 缺少引号
    let result = NodeType::parse_pysql(sql);
    assert!(result.is_err());
}

// 测试复杂嵌套结构
#[test]
fn test_complex_nested_structure() {
    let sql = "SELECT * FROM users
where:
  if id != null:
    AND id = #{id}
choose:
  when status == 'active':
    WHERE status = 'active'
  when status == 'inactive':
    WHERE status = 'inactive'
  otherwise:
    WHERE status IS NOT NULL
set:
  if name != null:
    name = #{name},
  trim ',':
    active = true,
for item in items:
  if item != null:
    #{item.name}
  if item == null:
    continue:";
    
    let nodes = NodeType::parse_pysql(sql).unwrap();
    
    // 验证节点的总数和类型
    assert!(nodes.len() >= 5); // 至少包含 StringNode, WhereNode, ChooseNode, SetNode, ForEachNode
    
    // 验证至少包含以下节点类型
    let has_string = nodes.iter().any(|node| matches!(node, NodeType::NString(_)));
    let has_where = nodes.iter().any(|node| matches!(node, NodeType::NWhere(_)));
    let has_choose = nodes.iter().any(|node| matches!(node, NodeType::NChoose(_)));
    let has_set = nodes.iter().any(|node| matches!(node, NodeType::NSet(_)));
    let has_foreach = nodes.iter().any(|node| matches!(node, NodeType::NForEach(_)));
    
    assert!(has_string, "Expected StringNode in complex structure");
    assert!(has_where, "Expected WhereNode in complex structure");
    assert!(has_choose, "Expected ChooseNode in complex structure");
    assert!(has_set, "Expected SetNode in complex structure");
    assert!(has_foreach, "Expected ForEachNode in complex structure");
}

// 测试语法错误情况
#[test]
fn test_syntax_errors() {
    // 测试：错误的 for 语法 - 没有 in 关键字
    let sql = "for item items:\n  #{item}";
    let result = NodeType::parse_pysql(sql);
    assert!(result.is_err(), "Expected error for missing 'in' keyword in for loop");
    
    // 测试：错误的 trim 语法 - 缺少参数
    let sql = "trim:\n  WHERE id = #{id}";
    let result = NodeType::parse_pysql(sql);
    assert!(result.is_err(), "Expected error for missing parameter in trim statement");
    
    // 测试：错误的 sql 节点 - 缺少 id
    let sql = "sql:\n  id, name, age";
    let result = NodeType::parse_pysql(sql);
    assert!(result.is_err(), "Expected error for missing id in sql node");
} 