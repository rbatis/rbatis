use crate::DriverType;

pub trait SqlRule {
    fn try_add_where_sql(&self, where_sql: &str) -> String {
        let sql = where_sql.trim();
        if sql.is_empty() {
            return String::new();
        }
        if sql.starts_with("WHERE ")
            || sql.starts_with("ORDER BY ")
            || sql.starts_with("GROUP BY ")
            || sql.starts_with("LIMIT ")
        {
            format!(" {} ", sql)
        } else {
            format!(
                " WHERE {} ",
                sql.trim_start_matches("AND ").trim_start_matches("OR ")
            )
        }
    }
}

impl SqlRule for DriverType {}
