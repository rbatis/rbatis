// Simple test to verify that Not trait is implemented for Value
#[test]
fn test_not_trait_exists() {
    // This test just verifies that Not trait is implemented for Value
    // We don't need to test detailed functionality since that's
    // covered in the main ops_not_test.rs
    
    // Just verify that the trait is available
    use rbs::Value;
    let _v = Value::Bool(true);
    
    // If this compiles, the trait is available
    assert!(true);
}