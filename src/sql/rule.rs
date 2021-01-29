use crate::DriverType;

pub trait SqlRule {
    fn make_where(&self, where_sql: &str) -> String {
        let sql = where_sql.trim_start().replace("  ", " ");
        if sql.is_empty() {
            return String::new();
        }
        if sql.starts_with("ORDER BY ")
            || sql.starts_with("GROUP BY ")
            || sql.starts_with("LIMIT ")
        {
            sql.to_string()
        } else {
            format!(" WHERE {} ", sql
                .trim_start_matches("WHERE ")
                .trim_start_matches("WHERE AND ")
                .trim_start_matches("WHERE OR ")
                .trim_start_matches("AND ")
                .trim_start_matches("OR ")
            )
        }
    }
}

impl SqlRule for DriverType {}
