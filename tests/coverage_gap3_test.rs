//! Third coverage pass for the last reachable lines in the operator macros,
//! the `IncludeTagNode` tag name, `ops::PartialOrd::op_le` and the empty
//! `<when>`/`<otherwise>` arms of the HTML element dispatcher.
//!
//! Everything else reported as uncovered by tarpaulin is structurally
//! unreachable from a runtime test: `quote!` token literals, the
//! `proc_macro::TokenStream` entry points, `if false` macro bodies, empty
//! match-arm labels and defensive error closures whose precondition can never
//! fail.

#[cfg(test)]
mod test {
    use rbatis_codegen::codegen::parser_html::parse_html;
    use rbatis_codegen::codegen::syntax_tree_html::{HtmlAstNode, IncludeTagNode};
    use rbatis_codegen::ops::{PartialEq as OpEq, PartialOrd as OpOrd};
    use rbs::Value;
    use std::cmp::Ordering;

    // ops_macros.rs: the scalar compare macro expansions for `&&Value` receivers,
    // `&primitive` reverse impls and the `&rhs` cells of `impl_compare_matrix`.
    #[test]
    fn scalar_compare_macro_cells() {
        let v = Value::I64(5);
        let vv = &&v;

        // impl PartialEq<i64> for &&Value / PartialEq<&i64> for &&Value
        assert!(<&&Value as OpEq<i64>>::op_eq(&vv, &5i64));
        assert!(<&&Value as OpEq<&i64>>::op_eq(&vv, &&5i64));

        // reverse: impl PartialEq<Value> / <&Value> / <&&Value> for &i64
        let n = 5i64;
        assert!(<&i64 as OpEq<Value>>::op_eq(&&n, &v));
        assert!(<&i64 as OpEq<&Value>>::op_eq(&&n, &&v));
        assert!(<&i64 as OpEq<&&Value>>::op_eq(&&n, &&&v));

        // impl_compare_matrix cells whose receiver is `&$rhs`
        let x: u16 = 5;
        let y: u8 = 5;
        assert!(<&u16 as OpEq<u8>>::op_eq(&&x, &y));
        assert!(<&u16 as OpEq<&u8>>::op_eq(&&x, &&y));

        // the same cells for PartialOrd
        assert_eq!(
            <&&Value as OpOrd<i64>>::op_partial_cmp(&vv, &5i64),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&&Value as OpOrd<&i64>>::op_partial_cmp(&vv, &&5i64),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&i64 as OpOrd<Value>>::op_partial_cmp(&&n, &v),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&i64 as OpOrd<&Value>>::op_partial_cmp(&&n, &&v),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&i64 as OpOrd<&&Value>>::op_partial_cmp(&&n, &&&v),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&u16 as OpOrd<u8>>::op_partial_cmp(&&x, &y),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&u16 as OpOrd<&u8>>::op_partial_cmp(&&x, &&y),
            Some(Ordering::Equal)
        );
    }

    // ops.rs: the default `op_le` body (`Some(Less) | Some(Equal)`).
    #[test]
    fn op_le_default_body() {
        assert!(<i64 as OpOrd<i64>>::op_le(&5i64, &5i64));
        assert!(<i64 as OpOrd<i64>>::op_le(&5i64, &6i64));
        assert!(!<i64 as OpOrd<i64>>::op_le(&5i64, &4i64));
    }

    // include_tag_node.rs: `node_tag_name` is not used by `parse_html` (the
    // dispatcher matches string constants), so call it directly.
    #[test]
    fn include_node_tag_name() {
        assert_eq!(IncludeTagNode::node_tag_name(), "include");
    }

    // parser_html.rs: a `<when>` / `<otherwise>` element that reaches the element
    // dispatcher directly (they are normally consumed by `<choose>`). The
    // dispatcher ignores them; this exercises those match arms.
    #[test]
    fn top_level_when_and_otherwise_are_ignored() {
        let mut ignore = vec![];
        let code = parse_html(
            r#"<select id="s">`select 1`<when test="1==1">`+ 1`</when><otherwise>`+ 2`</otherwise></select>"#,
            "s",
            &mut ignore,
        )
        .to_string();
        assert!(!code.is_empty());
    }
}
