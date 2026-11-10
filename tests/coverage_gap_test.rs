//! Small, targeted tests for the few operator/impl combinations that the
//! generated impl matrices in `rbatis-codegen` leave uncovered: `&&Value`
//! receivers, primitive-reference reverse comparisons and the reference
//! `IntoSql`/`op_le` paths. These are behavior checks, not just line hits.

#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::syntax_tree_html::{
        DeleteTagNode, HtmlAstNode, InsertTagNode, MapperTagNode, SelectTagNode, SqlTagNode,
        UpdateTagNode,
    };
    use rbatis_codegen::ops::*;
    use rbatis_codegen::IntoSql;
    use rbs::Value;
    use std::borrow::Cow;
    use std::cmp::Ordering;

    // ops_eq.rs: the impls whose receiver is `&&Value` or a `&String`/`&Value`.
    #[test]
    fn eq_amp_value_receiver() {
        let a = Value::I64(5);
        let b = Value::I64(5);
        assert!((&&a).op_eq(&b));
        assert!((&&a).op_eq(&&b));
        assert!((&&a).op_eq(&&&b));
    }

    #[test]
    fn eq_string_reference_combinations() {
        let v = Value::String("x".to_string());
        let s = String::from("x");
        assert!((&v).op_eq(&s)); // PartialEq<String> for &Value
        assert!((&v).op_eq(&&s)); // PartialEq<&String> for &Value
        assert!((&s).op_eq(&v)); // PartialEq<Value> for &String
    }

    // ops_cmp.rs: `&&Value` receivers and `&&Value` vs `&str`/`&&str`.
    #[test]
    fn cmp_amp_value_receiver() {
        let a = Value::I64(5);
        let b = Value::I64(5);
        assert_eq!((&&a).op_partial_cmp(&b), Some(Ordering::Equal));
        assert_eq!((&&a).op_partial_cmp(&&b), Some(Ordering::Equal));
        assert_eq!((&&a).op_partial_cmp(&&&b), Some(Ordering::Equal));

        let s = Value::String("x".to_string());
        assert_eq!((&&s).op_partial_cmp(&"x"), Some(Ordering::Equal));
        assert_eq!((&&s).op_partial_cmp(&&"x"), Some(Ordering::Equal));
    }

    // ops_macros.rs: `&&Value` operator receivers, `&primitive` reverse impls and
    // the `impl_compare_matrix` `&rhs` cells (cross-type primitive references).
    #[test]
    fn scalar_matrix_reference_combinations() {
        let v = Value::I64(5);

        assert!((&&v).op_eq(&5i64));
        assert!((&&v).op_eq(&&5i64));

        assert!((&5i64).op_eq(&v));
        assert!((&5i64).op_eq(&&v));
        assert!((&5i64).op_eq(&&&v));

        assert!((&5u16).op_eq(&5u8));
        assert!((&5u16).op_eq(&&5u8));

        assert_eq!((&5i32).op_partial_cmp(&5u8), Some(Ordering::Equal));
        assert_eq!((&5i32).op_partial_cmp(&&5u8), Some(Ordering::Equal));
        assert_eq!((&6i32).op_partial_cmp(&5u8), Some(Ordering::Greater));
    }

    // ops.rs: the default `op_le` body, including the `Equal` match arm.
    #[test]
    fn le_default_body_equal_and_less() {
        assert!((5i64).op_le(&5i64));
        assert!((5i64).op_le(&6i64));
        assert!(!(5i64).op_le(&4i64));
        assert!((&5i64).op_le(&&5i64));
        assert!(Value::I64(5).op_le(&Value::I64(5)));
    }

    // into_sql.rs: the `&Value` and `Cow` implementations.
    #[test]
    fn into_sql_reference_and_cow() {
        let v = Value::I64(1);
        let r: &Value = &v;
        assert_eq!(<&Value as IntoSql>::sql(&r), "1");
        assert_eq!(<Value as IntoSql>::sql(&v), "1");
        let borrowed = Cow::Borrowed(&v);
        assert_eq!(<Cow<'_, Value> as IntoSql>::sql(&borrowed), "1");
        let owned: Cow<'_, Value> = Cow::Owned(Value::String("x".to_string()));
        assert_eq!(owned.sql(), "'x'");
    }

    // The CRUD html nodes' `node_tag_name` is not used by `parse_html` (it
    // dispatches on string constants), so call it directly.
    #[test]
    fn html_crud_node_tag_names() {
        assert_eq!(SelectTagNode::node_tag_name(), "select");
        assert_eq!(InsertTagNode::node_tag_name(), "insert");
        assert_eq!(UpdateTagNode::node_tag_name(), "update");
        assert_eq!(DeleteTagNode::node_tag_name(), "delete");
        assert_eq!(MapperTagNode::node_tag_name(), "mapper");
        assert_eq!(SqlTagNode::node_tag_name(), "sql");
    }
}
