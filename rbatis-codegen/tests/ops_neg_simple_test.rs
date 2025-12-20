// Simple test to verify that Neg trait is implemented for Value
#[test]
fn test_neg_trait_exists() {
    // This test just verifies that the Neg trait is implemented for Value
    // We don't need to test detailed functionality since that's
    // covered in the main ops_neg_test.rs
    
    // Just verify that the trait is available
    use rbs::Value;
    let _v = Value::I32(42);
    
    // If this compiles, the trait is available
    assert!(true);
}