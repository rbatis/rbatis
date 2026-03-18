use rbs::Value;

const DTD: &str = r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.1//EN"
        "https://raw.githubusercontent.com/rbatis/rbatis/master/rbatis-codegen/mybatis-3-mapper.dtd">"#;

/// Generate an rbatis HTML SQL mapper template from a table name and a struct
/// serialized as an [`rbs::Value`].
///
/// The generated template contains four common CRUD operations:
/// - `select_by_column` — SELECT with a dynamic `<where>` block; string
///   fields receive an additional empty-string guard (`!= ''`).
/// - `insert`           — INSERT using `<foreach>` over the struct map
/// - `update_by_id`     — UPDATE using rbatis `<set collection="arg">` (which
///   expands to a SET clause for every non-null field at runtime), with a
///   `WHERE {id_column} = #{id_column}` predicate.
/// - `delete_by_id`     — DELETE filtering on `id_column`
///
/// # Arguments
/// * `table_name` — Name of the database table (e.g. `"activity"`)
/// * `value`      — An `rbs::Value` produced from a struct instance via
///                  [`rbs::value!`].  The value must be a `Value::Map`.
/// * `id_column`  — Name of the primary-key column used in UPDATE / DELETE
///                  predicates (typically `"id"`).
///
/// Returns an empty [`String`] when `value` is not a `Value::Map`.
///
/// # Example
/// ```rust
/// use rbatis::rbdc::datetime::DateTime;
/// use rbatis::plugin::gen_html_sql::gen_html_sql;
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// pub struct Activity {
///     pub id: Option<String>,
///     pub name: Option<String>,
///     pub status: Option<i32>,
///     pub create_time: Option<DateTime>,
/// }
///
/// let activity = Activity {
///     id: Some(String::new()),
///     name: Some(String::new()),
///     status: Some(0),
///     create_time: Some(DateTime::now()),
/// };
/// let html = gen_html_sql("activity", &rbs::value!(activity), "id");
/// assert!(html.contains("<select id=\"select_by_column\">"));
/// assert!(html.contains("<insert id=\"insert\">"));
/// assert!(html.contains("<update id=\"update_by_id\">"));
/// assert!(html.contains("<delete id=\"delete_by_id\">"));
/// ```
pub fn gen_html_sql(table_name: &str, value: &Value, id_column: &str) -> String {
    match value {
        Value::Map(m) => {
            let mut result = format!("{}\n<mapper>\n", DTD);
            result.push_str(&gen_select(table_name, m));
            result.push('\n');
            result.push_str(&gen_insert(table_name));
            result.push('\n');
            result.push_str(&gen_update(table_name, id_column));
            result.push('\n');
            result.push_str(&gen_delete(table_name, id_column));
            result.push_str("\n</mapper>");
            result
        }
        _ => String::new(),
    }
}

/// Returns `true` when the value represents a string-like SQL column whose
/// HTML SQL condition should include an empty-string guard (`!= ''`).
fn is_string_value(v: &Value) -> bool {
    matches!(v, Value::String(_))
}

fn gen_select(table_name: &str, m: &rbs::value::map::ValueMap) -> String {
    let mut sql = format!("    <select id=\"select_by_column\">\n");
    sql.push_str(&format!("        `select * from {}`\n", table_name));
    sql.push_str("        <where>\n");

    for (k, v) in m.0.iter() {
        let col = match k.as_str() {
            Some(s) => s,
            None => continue,
        };
        let condition = if is_string_value(v) {
            format!("{} != null and {} != ''", col, col)
        } else {
            format!("{} != null", col)
        };
        sql.push_str(&format!(
            "            <if test=\"{}\">\n                ` and {} = #{{{}}}`\n            </if>\n",
            condition, col, col
        ));
    }

    sql.push_str("        </where>\n");
    sql.push_str("    </select>");
    sql
}

fn gen_insert(table_name: &str) -> String {
    format!(
        r#"    <insert id="insert">
        `insert into {table_name}`
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            ${{key}}
        </foreach>
        ` values `
        <foreach collection="arg" index="key" item="item" open="(" close=")" separator=",">
            ${{item.sql()}}
        </foreach>
    </insert>"#,
        table_name = table_name
    )
}

fn gen_update(table_name: &str, id_column: &str) -> String {
    format!(
        r#"    <update id="update_by_id">
        ` update {table_name} `
        <set collection="arg"></set>
        ` where {id_column} = #{{{id_column}}} `
    </update>"#,
        table_name = table_name,
        id_column = id_column,
    )
}

fn gen_delete(table_name: &str, id_column: &str) -> String {
    format!(
        r#"    <delete id="delete_by_id">
        `delete from {table_name} where {id_column} = #{{{id_column}}}`
    </delete>"#,
        table_name = table_name,
        id_column = id_column,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rbdc::datetime::DateTime;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Activity {
        pub id: Option<String>,
        pub name: Option<String>,
        pub pc_link: Option<String>,
        pub h5_link: Option<String>,
        pub pc_banner_img: Option<String>,
        pub h5_banner_img: Option<String>,
        pub sort: Option<String>,
        pub status: Option<i32>,
        pub remark: Option<String>,
        pub create_time: Option<DateTime>,
        pub version: Option<i64>,
        pub delete_flag: Option<i32>,
    }

    fn activity_value() -> Value {
        rbs::value!(Activity {
            id: Some(String::new()),
            name: Some(String::new()),
            pc_link: Some(String::new()),
            h5_link: Some(String::new()),
            pc_banner_img: Some(String::new()),
            h5_banner_img: Some(String::new()),
            sort: Some(String::new()),
            status: Some(0),
            remark: Some(String::new()),
            create_time: Some(DateTime::now()),
            version: Some(0i64),
            delete_flag: Some(0),
        })
    }

    #[test]
    fn test_gen_html_sql_contains_dtd() {
        let html = gen_html_sql("activity", &activity_value(), "id");
        assert!(html.contains(DTD), "generated HTML should contain the DTD declaration");
    }

    #[test]
    fn test_gen_html_sql_contains_mapper_tags() {
        let html = gen_html_sql("activity", &activity_value(), "id");
        assert!(html.contains("<mapper>"));
        assert!(html.contains("</mapper>"));
    }

    #[test]
    fn test_gen_html_sql_contains_all_crud_ids() {
        let html = gen_html_sql("activity", &activity_value(), "id");
        assert!(html.contains("id=\"select_by_column\""), "missing select_by_column");
        assert!(html.contains("id=\"insert\""), "missing insert");
        assert!(html.contains("id=\"update_by_id\""), "missing update_by_id");
        assert!(html.contains("id=\"delete_by_id\""), "missing delete_by_id");
    }

    #[test]
    fn test_gen_html_sql_select_string_condition() {
        let html = gen_html_sql("activity", &activity_value(), "id");
        // String fields get an extra `!= ''` guard
        assert!(
            html.contains("name != null and name != ''"),
            "string field should have empty-string guard"
        );
    }

    #[test]
    fn test_gen_html_sql_select_numeric_condition() {
        let html = gen_html_sql("activity", &activity_value(), "id");
        // Numeric fields only get a null guard
        assert!(
            html.contains("status != null"),
            "numeric field should only have null guard"
        );
        assert!(
            !html.contains("status != null and status != ''"),
            "numeric field should NOT have empty-string guard"
        );
    }

    #[test]
    fn test_gen_html_sql_table_name_used() {
        let html = gen_html_sql("activity", &activity_value(), "id");
        assert!(html.contains("from activity"));
        assert!(html.contains("insert into activity"));
        assert!(html.contains("update activity"));
        assert!(html.contains("delete from activity"));
    }

    #[test]
    fn test_gen_html_sql_non_map_returns_empty() {
        assert_eq!(gen_html_sql("t", &Value::Null, "id"), "");
        assert_eq!(gen_html_sql("t", &Value::String("x".into()), "id"), "");
        assert_eq!(gen_html_sql("t", &Value::I32(1), "id"), "");
    }

    #[test]
    fn test_gen_html_sql_custom_table_name() {
        let v = rbs::value! { "id": "" };
        let html = gen_html_sql("my_custom_table", &v, "id");
        assert!(html.contains("from my_custom_table"));
        assert!(html.contains("insert into my_custom_table"));
    }

    #[test]
    fn test_gen_html_sql_custom_id_column() {
        let v = rbs::value! { "uuid": "", "title": "" };
        let html = gen_html_sql("post", &v, "uuid");
        assert!(html.contains("where uuid = #{uuid}"), "custom id column in DELETE");
        assert!(html.contains("where uuid = #{uuid}"), "custom id column in UPDATE");
    }
}
