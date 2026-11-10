//! Runtime coverage for the HTML AST nodes:
//! if / trim / where / choose / when / otherwise / foreach / set (+ bind, break, continue).
//!
//! Most tests drive the nodes through the runtime-safe `parse_html` entry point and
//! assert on the generated `proc_macro2::TokenStream` text. A few tests call the
//! `HtmlAstNode` trait directly (`from_element` / `generate_tokens`) to reach
//! attribute-default and empty-children branches without depending on the loader.
#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use proc_macro2::TokenStream;
    use quote::quote;
    use rbatis_codegen::codegen::loader_html::Element;
    use rbatis_codegen::codegen::parser_html::parse_html;
    use rbatis_codegen::codegen::syntax_tree_html::{
        BindTagNode, BreakTagNode, ChooseTagNode, ContinueTagNode, ForeachTagNode, HtmlAstNode,
        IfTagNode, NodeContext, OtherwiseTagNode, SetTagNode, TrimTagNode, WhenTagNode,
        WhereTagNode,
    };

    /// Parses a template whose statement id is always `s`.
    fn parse(html: &str) -> String {
        let mut ignore = vec![];
        parse_html(html, "s", &mut ignore).to_string()
    }

    fn assert_contains(haystack: &str, needle: &str) {
        assert!(
            haystack.contains(needle),
            "expected generated code to contain {:?}, got:\n{}",
            needle,
            haystack
        );
    }

    fn element(tag: &str, attrs: &[(&str, &str)], childs: Vec<Element>) -> Element {
        Element {
            tag: tag.to_string(),
            data: String::new(),
            attrs: attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<HashMap<_, _>>(),
            childs,
        }
    }

    /// Runs `generate_tokens` on a node with a child parser that records that it was
    /// called. This lets a node's "has children" / "no children" branch be covered
    /// without a full template.
    fn direct<N: HtmlAstNode>(node: &N) -> (String, Vec<String>) {
        let mut methods = TokenStream::new();
        let mut ignore = Vec::new();
        let mut context = NodeContext {
            methods: &mut methods,
            fn_name: "f",
            child_parser: |children: &[Element],
                           _methods: &mut TokenStream,
                           ignore: &mut Vec<String>,
                           _fn_name: &str| {
                let mut tokens = quote! {};
                for child in children {
                    let tag = &child.tag;
                    tokens.extend(quote! { /* child */ });
                    let _ = tag;
                }
                ignore.push("child-parsed".to_string());
                tokens
            },
        };
        let tokens = node.generate_tokens(&mut context, &mut ignore);
        let text = tokens.to_string();
        (text, ignore)
    }

    // Several node_tag_name() bodies are never called by the parser (it dispatches on
    // string constants), so call them directly to cover those lines.
    #[test]
    fn test_node_tag_names() {
        assert_eq!(IfTagNode::node_tag_name(), "if");
        assert_eq!(TrimTagNode::node_tag_name(), "trim");
        assert_eq!(WhereTagNode::node_tag_name(), "where");
        assert_eq!(ChooseTagNode::node_tag_name(), "choose");
        assert_eq!(WhenTagNode::node_tag_name(), "when");
        assert_eq!(OtherwiseTagNode::node_tag_name(), "otherwise");
        assert_eq!(ForeachTagNode::node_tag_name(), "foreach");
        assert_eq!(SetTagNode::node_tag_name(), "set");
        assert_eq!(BindTagNode::node_tag_name(), "bind");
        assert_eq!(BreakTagNode::node_tag_name(), "break");
        assert_eq!(ContinueTagNode::node_tag_name(), "continue");
    }

    // ------------------------------------------------------------------
    // <if>
    // ------------------------------------------------------------------

    // if with children: parse_children branch is taken.
    #[test]
    fn test_if_with_children() {
        let code = parse(r#"<select id="s">`a ` <if test="a == 1">`b`</if></select>"#);
        assert_contains(&code, "if");
        assert_contains(&code, "& arg [\"a\"]");
        assert_contains(&code, "op_eq");
        // body text of the if is emitted inside the conditional
        assert_contains(&code, "b");
    }

    // if without children: empty quote!{} branch is taken.
    #[test]
    fn test_if_without_children() {
        let code = parse(r#"<select id="s">`a ` <if test="a == 1"></if></select>"#);
        assert_contains(&code, "if");
        assert_contains(&code, "& arg [\"a\"]");
        assert_contains(&code, "op_eq");
    }

    // if test expressions: comparison, arithmetic, string literal, method call,
    // logical and/or and negation all go through impl_fn.
    #[test]
    fn test_if_expression_variants() {
        // comparison and arithmetic
        let code = parse(r#"<select id="s"><if test="a + 1 > 2">`x`</if></select>"#);
        assert_contains(&code, "op_add");
        assert_contains(&code, "op_gt");

        // string literal
        let code = parse(r#"<select id="s"><if test="name == 'abc'">`x`</if></select>"#);
        assert_contains(&code, "\"abc\"");
        assert_contains(&code, "op_eq");

        // method call
        let code = parse(r#"<select id="s"><if test="name.len() > 0">`x`</if></select>"#);
        assert_contains(&code, "len");
        assert_contains(&code, "op_gt");

        // logical and / or
        let code = parse(r#"<select id="s"><if test="a && b">`x`</if></select>"#);
        assert_contains(&code, "bool :: op_from");
        let code = parse(r#"<select id="s"><if test="a || b">`x`</if></select>"#);
        assert_contains(&code, "bool :: op_from");

        // negation
        let code = parse(r#"<select id="s"><if test="!a">`x`</if></select>"#);
        assert_contains(&code, "!");
        assert_contains(&code, "& arg [\"a\"]");
    }

    #[test]
    fn test_if_direct_with_and_without_children() {
        // non-empty childs -> child parser is called
        let node = IfTagNode::from_element(&element(
            "if",
            &[("test", "a")],
            vec![element("", &[], vec![])],
        ));
        assert_eq!(node.test, "a");
        let (code, ignore) = direct(&node);
        assert_contains(&code, "if");
        assert_eq!(ignore, vec!["child-parsed".to_string()]);

        // empty childs -> child parser is not called
        let node = IfTagNode::from_element(&element("if", &[("test", "a")], vec![]));
        let (_code, ignore) = direct(&node);
        assert!(ignore.is_empty());
    }

    #[test]
    #[should_panic(expected = "<if> element must have test field")]
    fn test_if_missing_test_panics() {
        parse(r#"<select id="s"><if>`x`</if></select>"#);
    }

    // ------------------------------------------------------------------
    // <trim>
    // ------------------------------------------------------------------

    // all four attributes plus multiple `|` alternatives in start/end
    #[test]
    fn test_trim_all_attributes() {
        let code = parse(
            r#"<select id="s"><trim prefix="PREX" suffix="SUFX" start="a|b" end="c|d">`body`</trim></select>"#,
        );
        assert_contains(&code, "PREX");
        assert_contains(&code, "SUFX");
        assert_eq!(code.matches("trim_start_matches").count(), 2);
        assert_eq!(code.matches("trim_end_matches").count(), 2);
        assert_contains(&code, "\"a\"");
        assert_contains(&code, "\"b\"");
        assert_contains(&code, "\"c\"");
        assert_contains(&code, "\"d\"");
    }

    #[test]
    fn test_trim_only_prefix() {
        let code = parse(r#"<select id="s"><trim prefix="PREX">`body`</trim></select>"#);
        assert_contains(&code, "PREX");
        assert!(!code.contains("trim_start_matches"));
        assert!(!code.contains("trim_end_matches"));
    }

    #[test]
    fn test_trim_only_suffix() {
        let code = parse(r#"<select id="s"><trim suffix="SUFX">`body`</trim></select>"#);
        assert_contains(&code, "SUFX");
        assert!(!code.contains("trim_start_matches"));
        assert!(!code.contains("trim_end_matches"));
    }

    #[test]
    fn test_trim_prefix_overrides_camel_and_lowercase() {
        let code = parse(
            r#"<select id="s"><trim prefixOverrides="CAMPRE">`body`</trim></select>"#,
        );
        assert_contains(&code, "CAMPRE");
        assert_contains(&code, "trim_start_matches");

        let code = parse(
            r#"<select id="s"><trim prefixoverrides="LOWPRE">`body`</trim></select>"#,
        );
        assert_contains(&code, "LOWPRE");
        assert_contains(&code, "trim_start_matches");
    }

    #[test]
    fn test_trim_suffix_overrides_camel_and_lowercase() {
        let code = parse(
            r#"<select id="s"><trim suffixOverrides="CAMSUF">`body`</trim></select>"#,
        );
        assert_contains(&code, "CAMSUF");
        assert_contains(&code, "trim_end_matches");

        let code = parse(
            r#"<select id="s"><trim suffixoverrides="LOWSUF">`body`</trim></select>"#,
        );
        assert_contains(&code, "LOWSUF");
        assert_contains(&code, "trim_end_matches");
    }

    // empty overrides -> no trimming path (has_trim == false)
    #[test]
    fn test_trim_empty_overrides_no_trimming() {
        let code = parse(
            r#"<select id="s"><trim prefix="PREX" suffix="SUFX" start="" end="">`body`</trim></select>"#,
        );
        assert_contains(&code, "PREX");
        assert_contains(&code, "SUFX");
        assert!(!code.contains("trim_start_matches"));
        assert!(!code.contains("trim_end_matches"));
        // the body is emitted directly, not wrapped in the trimming expression
        assert_contains(&code, "body");
    }

    // no attributes at all: every from_element default branch
    #[test]
    fn test_trim_no_attributes_defaults() {
        let node = TrimTagNode::from_element(&element("trim", &[], vec![]));
        assert_eq!(node.prefix, "");
        assert_eq!(node.suffix, "");
        assert_eq!(node.prefix_overrides, "");
        assert_eq!(node.suffix_overrides, "");

        let (code, _) = direct(&node);
        assert!(!code.contains("trim_start_matches"));
        assert!(!code.contains("trim_end_matches"));
    }

    // start/end take priority over prefixOverrides/suffixOverrides
    #[test]
    fn test_trim_start_end_priority() {
        let node = TrimTagNode::from_element(&element(
            "trim",
            &[
                ("start", "START_WINS"),
                ("prefixOverrides", "PRE_LOSES"),
                ("end", "END_WINS"),
                ("suffixOverrides", "SUF_LOSES"),
            ],
            vec![],
        ));
        assert_eq!(node.prefix_overrides, "START_WINS");
        assert_eq!(node.suffix_overrides, "END_WINS");
    }

    // ------------------------------------------------------------------
    // <where>
    // ------------------------------------------------------------------

    #[test]
    fn test_where_leading_and() {
        let code = parse(
            r#"<select id="s">`select * from t`<where>`and a = 1 and b = 2`</where></select>"#,
        );
        assert_contains(&code, " where ");
        // where builds a trim with the and/or overrides
        assert_contains(&code, "trim_start_matches");
        assert_contains(&code, "trim_end_matches");
        assert_contains(&code, "and ");
    }

    #[test]
    fn test_where_leading_or() {
        let code = parse(
            r#"<select id="s">`select * from t`<where>`or a = 1`</where></select>"#,
        );
        assert_contains(&code, " where ");
        assert_contains(&code, "or ");
    }

    #[test]
    fn test_where_without_leading_conjunction() {
        let code = parse(
            r#"<select id="s">`select * from t`<where>`a = 1`</where></select>"#,
        );
        assert_contains(&code, " where ");
        assert_contains(&code, "trim_end_matches");
    }

    #[test]
    fn test_where_empty() {
        let code = parse(r#"<select id="s">`select * from t`<where></where></select>"#);
        assert_contains(&code, " where ");
        // the trailing " where" cleanup is always generated
        assert_contains(&code, "\" where\"");
    }

    #[test]
    fn test_where_direct_generate() {
        let node = WhereTagNode::from_element(&element(
            "where",
            &[],
            vec![Element {
                tag: String::new(),
                data: "and a = 1".to_string(),
                attrs: HashMap::new(),
                childs: vec![],
            }],
        ));
        let (code, ignore) = direct(&node);
        assert_contains(&code, " where ");
        // child parser ran
        assert_eq!(ignore, vec!["child-parsed".to_string()]);
    }

    // ------------------------------------------------------------------
    // <choose> / <when> / <otherwise>
    // ------------------------------------------------------------------

    #[test]
    fn test_choose_when_with_children_and_otherwise() {
        let code = parse(
            r#"<select id="s"><choose>
                <when test="a == 1">`WHEN_A`</when>
                <when test="a == 2">`WHEN_B`</when>
                <when test="a == 3"></when>
                <otherwise>`OTHERWISE_C`</otherwise>
            </choose></select>"#,
        );
        // choose wraps the branches in its own closure
        assert_contains(&code, "String :: with_capacity");
        assert_contains(&code, "return sql");
        assert_contains(&code, "WHEN_A");
        assert_contains(&code, "WHEN_B");
        assert_contains(&code, "OTHERWISE_C");
        assert_contains(&code, "& arg [\"a\"]");
    }

    #[test]
    fn test_choose_when_without_children() {
        let code = parse(
            r#"<select id="s"><choose>
                <when test="a == 1"></when>
                <otherwise>`OTHERWISE_C`</otherwise>
            </choose></select>"#,
        );
        assert_contains(&code, "if");
        assert_contains(&code, "OTHERWISE_C");
    }

    #[test]
    fn test_choose_only_whens() {
        let code = parse(
            r#"<select id="s"><choose>
                <when test="a">`ONLY_A`</when>
                <when test="b">`ONLY_B`</when>
            </choose></select>"#,
        );
        assert_contains(&code, "ONLY_A");
        assert_contains(&code, "ONLY_B");
    }

    #[test]
    fn test_choose_empty() {
        let code = parse(r#"<select id="s"><choose></choose></select>"#);
        // the branch body is a closure over an empty string
        assert_contains(&code, "String :: with_capacity");
        assert!(!code.contains("if "));
    }

    // <otherwise> does not need a `test` attribute
    #[test]
    fn test_otherwise_without_test_is_allowed() {
        let code = parse(
            r#"<select id="s"><choose><otherwise>`NO_TEST_OTHERWISE`</otherwise></choose></select>"#,
        );
        assert_contains(&code, "NO_TEST_OTHERWISE");
    }

    #[test]
    fn test_when_direct_empty_and_non_empty() {
        let node = WhenTagNode::from_element(&element("when", &[("test", "a")], vec![]));
        assert_eq!(node.test, "a");
        let (code, ignore) = direct(&node);
        assert_contains(&code, "if");
        assert!(ignore.is_empty());

        let node = WhenTagNode::from_element(&element(
            "when",
            &[("test", "a")],
            vec![element("", &[], vec![])],
        ));
        let (_code, ignore) = direct(&node);
        assert_eq!(ignore, vec!["child-parsed".to_string()]);
    }

    #[test]
    fn test_otherwise_direct() {
        let node = OtherwiseTagNode::from_element(&element(
            "otherwise",
            &[],
            vec![element("", &[], vec![])],
        ));
        let (_code, ignore) = direct(&node);
        assert_eq!(ignore, vec!["child-parsed".to_string()]);
    }

    #[test]
    #[should_panic(expected = "<choose> node's children must be <when> or <otherwise>")]
    fn test_choose_invalid_child_panics() {
        parse(r#"<select id="s"><choose><if test="a">`x`</if></choose></select>"#);
    }

    #[test]
    #[should_panic(expected = "<when> element must have test field")]
    fn test_when_missing_test_panics() {
        parse(r#"<select id="s"><choose><when>`x`</when></choose></select>"#);
    }

    // ------------------------------------------------------------------
    // <foreach>
    // ------------------------------------------------------------------

    #[test]
    fn test_foreach_open_close_separator() {
        let code = parse(
            r#"<select id="s"><foreach collection="list" open="(" close=")" separator=",">`#{item}`</foreach></select>"#,
        );
        assert_contains(&code, "for (ref index , item)");
        assert_contains(&code, "& arg [\"list\"]");
        assert_contains(&code, "push_str (\"(\")");
        assert_contains(&code, "push_str (\")\")");
        assert_contains(&code, "push_str (\",\")");
        // separator trimming of the last element
        assert_contains(&code, "__foreach_sql_start");
        assert_contains(&code, "truncate");
    }

    #[test]
    fn test_foreach_without_open_close_separator() {
        let code = parse(
            r#"<select id="s"><foreach collection="list">`#{item}`</foreach></select>"#,
        );
        assert_contains(&code, "for (ref index , item)");
        assert!(!code.contains("__foreach_sql_start"));
        assert!(!code.contains("truncate"));
    }

    #[test]
    fn test_foreach_custom_item_index() {
        let code = parse(
            r#"<select id="s"><foreach collection="list" item="x" index="i">`#{x}`</foreach></select>"#,
        );
        assert_contains(&code, "for (ref i , x)");
    }

    // `item="_"` and `index="_"` fall back to the defaults
    #[test]
    fn test_foreach_underscore_defaults() {
        let code = parse(
            r#"<select id="s"><foreach collection="list" item="_" index="_">`#{item}`</foreach></select>"#,
        );
        assert_contains(&code, "for (ref index , item)");
    }

    // empty item/index values also fall back to the defaults
    #[test]
    fn test_foreach_empty_values_default() {
        let code = parse(
            r#"<select id="s"><foreach collection="list" item="" index="">`#{item}`</foreach></select>"#,
        );
        assert_contains(&code, "for (ref index , item)");
    }

    #[test]
    fn test_foreach_defaults_from_element() {
        let node = ForeachTagNode::from_element(&element("foreach", &[("collection", "l")], vec![]));
        assert_eq!(node.item, "item");
        assert_eq!(node.index, "index");
        assert_eq!(node.open, "");
        assert_eq!(node.close, "");
        assert_eq!(node.separator, "");

        let node = ForeachTagNode::from_element(&element(
            "foreach",
            &[("collection", "l"), ("item", "_"), ("index", "")],
            vec![],
        ));
        assert_eq!(node.item, "item");
        assert_eq!(node.index, "index");
    }

    #[test]
    fn test_foreach_nested_if_bind_continue_break() {
        let code = parse(
            r#"<select id="s"><foreach collection="list" separator=",">
                <if test="item > 0">
                    <bind name="bound" value="item + 1"></bind>
                    `#{bound}`
                    <continue></continue>
                </if>
                <break></break>
            </foreach></select>"#,
        );
        assert_contains(&code, "for (ref index , item)");
        assert_contains(&code, "if");
        assert_contains(&code, "continue ;");
        assert_contains(&code, "break ;");
        // <bind> writes the value into the argument map
        assert_contains(&code, "arg [\"bound\"] =");
    }

    #[test]
    #[should_panic(expected = "<foreach> element must have a 'collection' attribute")]
    fn test_foreach_missing_collection_panics() {
        parse(r#"<select id="s"><foreach item="x">`#{x}`</foreach></select>"#);
    }

    // ------------------------------------------------------------------
    // <set>
    // ------------------------------------------------------------------

    // plain form: no collection, children are trimmed of leading/trailing " |,"
    #[test]
    fn test_set_plain_form() {
        let code = parse(
            r#"<select id="s"><set>`name = #{name},`</set></select>"#,
        );
        assert_contains(&code, " set ");
        assert_contains(&code, "trim_start_matches");
        assert_contains(&code, "trim_end_matches");
        assert_contains(&code, "name");
    }

    #[test]
    fn test_set_collection_skips_id_name() {
        let code = parse(
            r#"<select id="s"><set collection="m" skips="id,name">`x`</set></select>"#,
        );
        assert_contains(&code, "& arg [\"m\"]");
        assert_contains(&code, "for (ref k , v)");
        // skips become `k == 'id'` / `k == 'name'` guards
        assert_contains(&code, "op_eq");
        assert_contains(&code, "\"id\"");
        assert_contains(&code, "\"name\"");
        // skip_null defaults to true -> the null guard is generated
        assert_contains(&code, "Value :: Null");
        assert_contains(&code, "continue ;");
    }

    #[test]
    fn test_set_collection_empty_skips() {
        let code = parse(
            r#"<select id="s"><set collection="m" skips="" skip_null="false">`x`</set></select>"#,
        );
        // no skip guards at all
        assert!(!code.contains("op_eq"));
        assert!(!code.contains("Value :: Null"));
        assert_contains(&code, "for (ref k , v)");
    }

    // omitted skips defaults to "id"
    #[test]
    fn test_set_collection_omitted_skips_defaults_to_id() {
        let node = SetTagNode::from_element(&element("set", &[("collection", "m")], vec![]));
        assert_eq!(node.skips, "id");
        assert_eq!(node.skip_null, None);

        let code = parse(r#"<select id="s"><set collection="m">`x`</set></select>"#);
        assert_contains(&code, "\"id\"");
        assert_contains(&code, "op_eq");
    }

    #[test]
    fn test_set_collection_skip_null_true_and_false() {
        let code = parse(
            r#"<select id="s"><set collection="m" skip_null="true">`x`</set></select>"#,
        );
        assert_contains(&code, "Value :: Null");

        let code = parse(
            r#"<select id="s"><set collection="m" skip_null="false">`x`</set></select>"#,
        );
        assert!(!code.contains("Value :: Null"));
        // skips defaults to "id" even when skip_null is false
        assert_contains(&code, "\"id\"");
    }

    // explicit skip_null absent: is_skip_null is true
    #[test]
    fn test_set_collection_skip_null_omitted() {
        let code = parse(
            r#"<select id="s"><set collection="m" skips="a">`x`</set></select>"#,
        );
        assert_contains(&code, "Value :: Null");
    }

    #[test]
    fn test_set_plain_direct_from_element() {
        let node = SetTagNode::from_element(&element(
            "set",
            &[],
            vec![element("", &[], vec![])],
        ));
        assert!(node.collection.is_none());
        assert!(node.skip_null.is_none());
        assert_eq!(node.skips, "id");
        let (code, ignore) = direct(&node);
        assert_contains(&code, " set ");
        assert_eq!(ignore, vec!["child-parsed".to_string()]);
    }

    #[test]
    fn test_set_collection_direct_from_element() {
        let node = SetTagNode::from_element(&element(
            "set",
            &[("collection", "m"), ("skips", "a,b"), ("skip_null", "false")],
            vec![],
        ));
        assert_eq!(node.collection.as_deref(), Some("m"));
        assert_eq!(node.skips, "a,b");
        assert_eq!(node.skip_null.as_deref(), Some("false"));
        // token generation for the collection form. The direct helper does not recurse
        // into the constructed trim/foreach tree, but it does run the whole element
        // construction before generating the outer trim.
        let (code, _) = direct(&node);
        assert_contains(&code, " set ");
        assert_contains(&code, "trim_start_matches");
    }

    // ------------------------------------------------------------------
    // <bind> / <break> / <continue> standalone
    // ------------------------------------------------------------------

    #[test]
    fn test_bind_standalone() {
        let code = parse(
            r#"<select id="s"><bind name="b" value="a + 1"></bind>`#{b}`</select>"#,
        );
        assert_contains(&code, "arg [\"b\"] =");
    }

    #[test]
    #[should_panic(expected = "<bind> element must have name")]
    fn test_bind_missing_name_panics() {
        parse(r#"<select id="s"><bind value="a"/></select>"#);
    }

    #[test]
    #[should_panic(expected = "<bind> element must have value")]
    fn test_bind_missing_value_panics() {
        parse(r#"<select id="s"><bind name="b"/></select>"#);
    }

    #[test]
    fn test_break_and_continue_standalone() {
        // break/continue are only meaningful in a loop but the node itself just emits
        // the keyword; drive them through foreach so the generated code is realistic.
        let code = parse(
            r#"<select id="s"><foreach collection="l"><continue></continue><break></break></foreach></select>"#,
        );
        assert_contains(&code, "continue ;");
        assert_contains(&code, "break ;");
    }
}
