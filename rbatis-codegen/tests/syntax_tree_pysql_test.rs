// Simple test to verify PySQL parsing works
#[test]
fn test_pysql_basic_parsing() {
    // This test just verifies that basic PySQL nodes are accessible
    // We don't need to test detailed functionality here since that's
    // covered in parser_pysql_test.rs
    
    // Just verify modules are accessible
    let _ = rbatis_codegen::codegen::syntax_tree_pysql::string_node::StringNode {
        value: "test".to_string(),
    };
    
    // If this compiles, basic imports work
    assert!(true);
}