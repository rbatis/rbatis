use crate::DriverType;

pub trait SqlRule {
    fn make_where(&self, where_sql: &str) -> String {
        let sql = where_sql.trim_start().replace("  ", " ");
        if sql.is_empty() {
            return String::new();
        }
        if sql.starts_with("order by ") || sql.starts_with("group by ") || sql.starts_with("limit ")
        {
            sql.to_string()
        } else {
            format!(
                " where {} ",
                sql.trim_start_matches("where ")
                    .trim_start_matches("and ")
                    .trim_start_matches("or ")
            )
        }
    }

    fn make_left_insert_where(&self, insert_sql: &str, where_sql: &str) -> String {
        let sql = where_sql
            .trim()
            .trim_start_matches("where ")
            .trim_start_matches("and ")
            .replace("  ", " ");
        if sql.is_empty() {
            return insert_sql.to_string();
        }
        if sql.starts_with("order by") || sql.starts_with("group by") || sql.starts_with("limit ") {
            format!(
                " where {} {}",
                insert_sql.trim().trim_end_matches(" and"),
                sql
            )
        } else {
            format!(
                " where {} and {}",
                insert_sql.trim().trim_end_matches(" and"),
                sql
            )
        }
    }
}

impl SqlRule for DriverType {}
