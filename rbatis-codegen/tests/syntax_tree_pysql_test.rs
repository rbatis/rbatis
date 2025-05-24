use rbatis_codegen::codegen::syntax_tree_pysql::{
    AsHtml, NodeType, to_html, DefaultName, Name,
};
use rbatis_codegen::codegen::syntax_tree_pysql::bind_node::BindNode;
use rbatis_codegen::codegen::syntax_tree_pysql::break_node::BreakNode;
use rbatis_codegen::codegen::syntax_tree_pysql::choose_node::ChooseNode;
use rbatis_codegen::codegen::syntax_tree_pysql::continue_node::ContinueNode;
use rbatis_codegen::codegen::syntax_tree_pysql::error::Error;
use rbatis_codegen::codegen::syntax_tree_pysql::foreach_node::ForEachNode;
use rbatis_codegen::codegen::syntax_tree_pysql::if_node::IfNode;
use rbatis_codegen::codegen::syntax_tree_pysql::otherwise_node::OtherwiseNode;
use rbatis_codegen::codegen::syntax_tree_pysql::set_node::SetNode;
use rbatis_codegen::codegen::syntax_tree_pysql::sql_node::SqlNode;
use rbatis_codegen::codegen::syntax_tree_pysql::string_node::StringNode;
use rbatis_codegen::codegen::syntax_tree_pysql::trim_node::TrimNode;
use rbatis_codegen::codegen::syntax_tree_pysql::when_node::WhenNode;
use rbatis_codegen::codegen::syntax_tree_pysql::where_node::WhereNode;
use std::error::Error as StdError;

#[test]
fn test_string_node_as_html() {
    let node = StringNode {
        value: "select * from user".to_string(),
    };
    let html = node.as_html();
    assert_eq!(html, "`select * from user`");
}

#[test]
fn test_string_node_as_html_with_backticks() {
    let node = StringNode {
        value: "`select * from user`".to_string(),
    };
    let html = node.as_html();
    assert_eq!(html, "`select * from user`");
}

#[test]
fn test_if_node_as_html() {
    let node = IfNode {
        childs: vec![NodeType::NString(StringNode {
            value: "where id = #{id}".to_string(),
        })],
        test: "id != null".to_string(),
    };
    let html = node.as_html();
    assert_eq!(html, "<if test=\"id != null\">`where id = #{id}`</if>");
}

#[test]
fn test_foreach_node_as_html() {
    let node = ForEachNode {
        childs: vec![NodeType::NString(StringNode {
            value: "#{item}".to_string(),
        })],
        collection: "ids".to_string(),
        index: "index".to_string(),
        item: "item".to_string(),
    };
    let html = node.as_html();
    assert_eq!(html, "<foreach collection=\"ids\" index=\"index\" item=\"item\" >`#{item}`</foreach>");
}

#[test]
fn test_choose_node_as_html() {
    let when_node = WhenNode {
        childs: vec![NodeType::NString(StringNode {
            value: "where id = #{id}".to_string(),
        })],
        test: "id != null".to_string(),
    };
    let otherwise_node = OtherwiseNode {
        childs: vec![NodeType::NString(StringNode {
            value: "where id = 0".to_string(),
        })],
    };
    let node = ChooseNode {
        when_nodes: vec![NodeType::NWhen(when_node)],
        otherwise_node: Some(Box::new(NodeType::NOtherwise(otherwise_node))),
    };
    let html = node.as_html();
    assert_eq!(html, "<choose><when test=\"id != null\">`where id = #{id}`</when><otherwise>`where id = 0`</otherwise></choose>");
}

#[test]
fn test_choose_node_without_otherwise_as_html() {
    let when_node = WhenNode {
        childs: vec![NodeType::NString(StringNode {
            value: "where id = #{id}".to_string(),
        })],
        test: "id != null".to_string(),
    };
    let node = ChooseNode {
        when_nodes: vec![NodeType::NWhen(when_node)],
        otherwise_node: None,
    };
    let html = node.as_html();
    assert_eq!(html, "<choose><when test=\"id != null\">`where id = #{id}`</when></choose>");
}

#[test]
fn test_trim_node_as_html() {
    let node = TrimNode {
        childs: vec![NodeType::NString(StringNode {
            value: "and id = #{id}".to_string(),
        })],
        start: "and".to_string(),
        end: ",".to_string(),
    };
    let html = node.as_html();
    assert_eq!(html, "<trim prefixOverrides=\"and\" suffixOverrides=\",\">`and id = #{id}`</trim>");
}

#[test]
fn test_where_node_as_html() {
    let node = WhereNode {
        childs: vec![NodeType::NString(StringNode {
            value: "id = #{id}".to_string(),
        })],
    };
    let html = node.as_html();
    assert_eq!(html, "<where>`id = #{id}`</where>");
}

#[test]
fn test_set_node_as_html() {
    let node = SetNode {
        childs: vec![NodeType::NString(StringNode {
            value: "name = #{name}".to_string(),
        })],
        collection: "".to_string(),
        skip_null: false,
        skips: "".to_string(),
    };
    let html = node.as_html();
    assert_eq!(html, "<set>`name = #{name}`</set>");
}

#[test]
fn test_set_node_as_html_with_collection_and_skips() {
    let node = SetNode {
        childs: vec![NodeType::NString(StringNode {
            value: "field = #{field_value}".to_string(),
        })],
        collection: "table".to_string(),
        skips: "id".to_string(),
        skip_null: false, // Test with skip_null as false
    };
    let html = node.as_html();
    assert_eq!(html, "<set collection=\"table\" skips=\"id\">`field = #{field_value}`</set>");

    let node_skip_null_true = SetNode {
        childs: vec![NodeType::NString(StringNode {
            value: "field2 = #{field_value2}".to_string(),
        })],
        collection: "another_table".to_string(),
        skips: "uid,timestamp".to_string(),
        skip_null: true, // Test with skip_null as true
    };
    let html_skip_null_true = node_skip_null_true.as_html();
    assert_eq!(html_skip_null_true, "<set collection=\"another_table\" skips=\"uid,timestamp\" skip_null=\"true\">`field2 = #{field_value2}`</set>");
}

#[test]
fn test_bind_node_as_html() {
    let node = BindNode {
        name: "pattern".to_string(),
        value: "'%' + name + '%'".to_string(),
    };
    let html = node.as_html();
    assert_eq!(html, "<bind name=\"pattern\" value=\"'%' + name + '%'\"/>");
}

#[test]
fn test_break_node_as_html() {
    let node = BreakNode {};
    let html = node.as_html();
    assert_eq!(html, "<break/>");
}

#[test]
fn test_continue_node_as_html() {
    let node = ContinueNode {};
    let html = node.as_html();
    assert_eq!(html, "<continue />");
}

#[test]
fn test_sql_node_as_html() {
    let node = SqlNode {
        childs: vec![NodeType::NString(StringNode {
            value: "select * from user".to_string(),
        })],
        id: "a".to_string(),
    };
    let html = node.as_html();
    assert_eq!(html, "<sql id=\"a\">`select * from user`</sql>");
}

#[test]
fn test_to_html_select() {
    let nodes = vec![
        NodeType::NString(StringNode {
            value: "select * from user".to_string(),
        }),
        NodeType::NIf(IfNode {
            childs: vec![NodeType::NString(StringNode {
                value: "where id = #{id}".to_string(),
            })],
            test: "id != null".to_string(),
        }),
    ];
    let html = to_html(&nodes, true, "findUser");
    assert!(html.contains("<mapper>"));
    assert!(html.contains("<select id=\"findUser\">"));
    assert!(html.contains("</select>"));
    assert!(html.contains("</mapper>"));
}

#[test]
fn test_to_html_update() {
    let nodes = vec![
        NodeType::NString(StringNode {
            value: "update user set".to_string(),
        }),
        NodeType::NSet(SetNode {
            childs: vec![NodeType::NString(StringNode {
                value: "name = #{name}".to_string(),
            })],
            collection: "default_collection".to_string(),
            skip_null: false,
            skips: "".to_string(),
        }),
    ];
    let html = to_html(&nodes, false, "updateUser");
    assert!(html.contains("<mapper>"));
    assert!(html.contains("<update id=\"updateUser\">"));
    assert!(html.contains("</update>"));
    assert!(html.contains("</mapper>"));
}

#[test]
fn test_error() {
    let error = Error::from("Test error");
    assert_eq!(error.to_string(), "Test error");
}

#[test]
fn test_node_names() {
    assert_eq!(String::name(), "string");
    assert_eq!(IfNode::name(), "if");
    assert_eq!(ForEachNode::name(), "for");
    assert_eq!(ChooseNode::name(), "choose");
    assert_eq!(WhenNode::name(), "when");
    assert_eq!(OtherwiseNode::name(), "otherwise");
    assert_eq!(WhereNode::name(), "where");
    assert_eq!(SetNode::name(), "set");
    assert_eq!(TrimNode::name(), "trim");
    assert_eq!(BindNode::name(), "bind");
    assert_eq!(BreakNode::name(), "break");
    assert_eq!(ContinueNode::name(), "continue");
    assert_eq!(SqlNode::name(), "sql");
}

#[test]
fn test_node_default_names() {
    assert_eq!(OtherwiseNode::default_name(), "_");
    assert_eq!(BindNode::default_name(), "let");
    
    // 注意：并非所有节点都实现了DefaultName特性
    // WhenNode没有实现DefaultName特性，所以不能调用WhenNode::default_name()
}

#[test]
fn test_complex_nested_nodes() {
    let inner_if = IfNode {
        childs: vec![NodeType::NString(StringNode {
            value: "name = #{name}".to_string(),
        })],
        test: "name != null".to_string(),
    };
    
    let foreach_node = ForEachNode {
        childs: vec![
            NodeType::NString(StringNode {
                value: "id = #{item}".to_string(),
            }),
            NodeType::NIf(inner_if),
        ],
        collection: "ids".to_string(),
        index: "index".to_string(),
        item: "item".to_string(),
    };
    
    let html = foreach_node.as_html();
    assert!(html.contains("<foreach"));
    assert!(html.contains("collection=\"ids\""));
    assert!(html.contains("<if test=\"name != null\""));
}

#[test]
fn test_vector_as_html() {
    let nodes = vec![
        NodeType::NString(StringNode {
            value: "select * from user".to_string(),
        }),
        NodeType::NString(StringNode {
            value: "where 1=1".to_string(),
        }),
    ];
    
    let html = nodes.as_html();
    assert_eq!(html, "`select * from user``where 1=1`");
}

#[test]
fn test_nested_node_types() {
    let string_node = NodeType::NString(StringNode {
        value: "id = #{id}".to_string(),
    });
    
    let html = string_node.as_html();
    assert_eq!(html, "`id = #{id}`");
    
    let if_node = NodeType::NIf(IfNode {
        childs: vec![string_node.clone()],
        test: "id != null".to_string(),
    });
    
    let html = if_node.as_html();
    assert_eq!(html, "<if test=\"id != null\">`id = #{id}`</if>");
}

#[test]
fn test_all_node_types_as_html() {
    // 创建并测试所有节点类型
    let string_node = NodeType::NString(StringNode {
        value: "test".to_string(),
    });
    assert_eq!(string_node.as_html(), "`test`");
    
    let if_node = NodeType::NIf(IfNode {
        childs: vec![string_node.clone()],
        test: "test".to_string(),
    });
    assert_eq!(if_node.as_html(), "<if test=\"test\">`test`</if>");
    
    let trim_node = NodeType::NTrim(TrimNode {
        childs: vec![string_node.clone()],
        start: "start".to_string(),
        end: "end".to_string(),
    });
    assert_eq!(trim_node.as_html(), "<trim prefixOverrides=\"start\" suffixOverrides=\"end\">`test`</trim>");
    
    let foreach_node = NodeType::NForEach(ForEachNode {
        childs: vec![string_node.clone()],
        collection: "collection".to_string(),
        index: "index".to_string(),
        item: "item".to_string(),
    });
    assert_eq!(foreach_node.as_html(), "<foreach collection=\"collection\" index=\"index\" item=\"item\" >`test`</foreach>");
    
    let when_node = WhenNode {
        childs: vec![string_node.clone()],
        test: "test".to_string(),
    };
    let otherwise_node = OtherwiseNode {
        childs: vec![string_node.clone()],
    };
    let choose_node = NodeType::NChoose(ChooseNode {
        when_nodes: vec![NodeType::NWhen(when_node.clone())],
        otherwise_node: Some(Box::new(NodeType::NOtherwise(otherwise_node.clone()))),
    });
    assert_eq!(choose_node.as_html(), "<choose><when test=\"test\">`test`</when><otherwise>`test`</otherwise></choose>");
    
    let otherwise_node_type = NodeType::NOtherwise(otherwise_node);
    assert_eq!(otherwise_node_type.as_html(), "<otherwise>`test`</otherwise>");
    
    let when_node_type = NodeType::NWhen(when_node);
    assert_eq!(when_node_type.as_html(), "<when test=\"test\">`test`</when>");
    
    let bind_node = NodeType::NBind(BindNode {
        name: "name".to_string(),
        value: "value".to_string(),
    });
    assert_eq!(bind_node.as_html(), "<bind name=\"name\" value=\"value\"/>");
    
    let set_node = NodeType::NSet(SetNode {
        childs: vec![string_node.clone()],
        collection: "".to_string(),
        skip_null: false,
        skips: "".to_string(),
    });
    assert_eq!(set_node.as_html(), "<set>`test`</set>");
    
    let where_node = NodeType::NWhere(WhereNode {
        childs: vec![string_node.clone()],
    });
    assert_eq!(where_node.as_html(), "<where>`test`</where>");
    
    let continue_node = NodeType::NContinue(ContinueNode {});
    assert_eq!(continue_node.as_html(), "<continue />");
    
    let break_node = NodeType::NBreak(BreakNode {});
    assert_eq!(break_node.as_html(), "<break/>");
    
    let sql_node = NodeType::NSql(SqlNode {
        childs: vec![string_node],
        id: "a".to_string(),
    });
    assert_eq!(sql_node.as_html(), "<sql id=\"a\">`test`</sql>");
}

#[test]
fn test_empty_nodes() {
    // 测试空节点
    let empty_if = IfNode {
        childs: vec![],
        test: "test".to_string(),
    };
    assert_eq!(empty_if.as_html(), "<if test=\"test\"></if>");
    
    let empty_foreach = ForEachNode {
        childs: vec![],
        collection: "collection".to_string(),
        index: "index".to_string(),
        item: "item".to_string(),
    };
    assert_eq!(empty_foreach.as_html(), "<foreach collection=\"collection\" index=\"index\" item=\"item\" ></foreach>");
    
    let empty_trim = TrimNode {
        childs: vec![],
        start: "start".to_string(),
        end: "end".to_string(),
    };
    assert_eq!(empty_trim.as_html(), "<trim prefixOverrides=\"start\" suffixOverrides=\"end\"></trim>");
    
    let empty_where = WhereNode {
        childs: vec![],
    };
    assert_eq!(empty_where.as_html(), "<where></where>");
    
    let empty_set = SetNode {
        childs: vec![],
        collection: "".to_string(),
        skip_null: false,
        skips: "".to_string(),
    };
    assert_eq!(empty_set.as_html(), "<set></set>");
    
    let empty_otherwise = OtherwiseNode {
        childs: vec![],
    };
    assert_eq!(empty_otherwise.as_html(), "<otherwise></otherwise>");
    
    let empty_sql = SqlNode {
        childs: vec![],
        id: "a".to_string(),
    };
    assert_eq!(empty_sql.as_html(), "<sql id=\"a\"></sql>");
}

#[test]
fn test_error_display_and_from() {
    let error1 = Error::from("Custom error message");
    assert_eq!(error1.to_string(), "Custom error message");
    
    let error2 = Error::from(std::io::Error::new(std::io::ErrorKind::Other, "IO error"));
    assert!(error2.to_string().contains("IO error"));
    
    let error3: Error = "String literal error".into();
    assert_eq!(error3.to_string(), "String literal error");
    
    // 测试 From<&dyn std::error::Error>
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "IO error");
    let std_error: &dyn StdError = &io_error;
    let error4 = Error::from(std_error);
    assert!(error4.to_string().contains("IO error"));
    
    // 测试 Clone
    let error5 = error1.clone();
    assert_eq!(error5.to_string(), "Custom error message");
    
    // 测试 Clone_from
    let mut error6 = Error::from("Original");
    error6.clone_from(&error1);
    assert_eq!(error6.to_string(), "Custom error message");
}

#[test]
fn test_vec_node_type_as_html() {
    let nodes: Vec<NodeType> = vec![];
    assert_eq!(nodes.as_html(), "");
    
    let nodes = vec![
        NodeType::NString(StringNode {
            value: "test1".to_string(),
        }),
        NodeType::NString(StringNode {
            value: "test2".to_string(),
        }),
        NodeType::NString(StringNode {
            value: "test3".to_string(),
        }),
    ];
    assert_eq!(nodes.as_html(), "`test1``test2``test3`");
}

#[test]
fn test_to_html_with_empty_nodes() {
    let nodes: Vec<NodeType> = vec![];
    let html = to_html(&nodes, true, "emptySelect");
    assert_eq!(html, "<mapper><select id=\"emptySelect\"></select></mapper>");
    
    let html = to_html(&nodes, false, "emptyUpdate");
    assert_eq!(html, "<mapper><update id=\"emptyUpdate\"></update></mapper>");
}

#[test]
fn test_all_name_methods() {
    // 测试每个节点类型的 name() 方法
    assert_eq!(String::name(), "string");
    assert_eq!(IfNode::name(), "if");
    assert_eq!(TrimNode::name(), "trim");
    assert_eq!(ForEachNode::name(), "for");
    assert_eq!(ChooseNode::name(), "choose");
    assert_eq!(OtherwiseNode::name(), "otherwise");
    assert_eq!(WhenNode::name(), "when");
    assert_eq!(BindNode::name(), "bind");
    assert_eq!(SetNode::name(), "set");
    assert_eq!(WhereNode::name(), "where");
    assert_eq!(ContinueNode::name(), "continue");
    assert_eq!(BreakNode::name(), "break");
    assert_eq!(SqlNode::name(), "sql");
}

#[test]
fn test_all_default_name_methods() {
    // 测试每个支持 DefaultName trait 的节点类型的 default_name() 方法
    assert_eq!(BindNode::default_name(), "let");
    assert_eq!(OtherwiseNode::default_name(), "_");
    
    // 注意：并非所有节点都实现了DefaultName特性
}

#[test]
fn test_string_name_trait() {
    // StringNode是特殊的，Name trait是在String上实现的，而不是StringNode上
    assert_eq!(String::name(), "string");
    
    // 注意：String类型实现了Name trait，但Name::name()是静态方法，不能通过实例调用
    // 下面这行代码会编译失败：
    // let string_value = "test".to_string();
    // string_value.name(); // 错误！
} 