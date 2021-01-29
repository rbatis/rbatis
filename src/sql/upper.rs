use crate::core::db::DriverType;

/// sql to up case
pub trait SqlUpperCase {
    fn upper_case_sql(&self, sql: &str) -> String {
        let sql = format!(" {} ", sql);
        sql.replace("  ", " ")
            .replace(" select ", " SELECT ")
            .replace(" delete ", " DELETE ")
            .replace(" update ", " UPDATE ")
            .replace(" insert ", " INSERT ")
            .replace(" set ", " SET ")
            .replace(" from ", " FROM ")
            .replace(" where ", " WHERE ")
            .replace(" group by ", " GROUP BY ")
            .replace(" order by ", " ORDER BY ")
            .replace(" limit ", " LIMIT ")
    }
}


pub trait SqlReplaceCase {
    fn try_add_where_sql(&self, where_sql: &str) -> String {
        let sql = where_sql.trim();
        if sql.is_empty() {
            return String::new();
        }
        if sql.starts_with("WHERE ") ||
            sql.starts_with("ORDER BY ") ||
            sql.starts_with("GROUP BY ") ||
            sql.starts_with("LIMIT ") {
            format!(" {} ", sql)
        } else {
            format!(" WHERE {} ", sql.trim_start_matches("AND ").trim_start_matches("OR "))
        }
    }
}

impl SqlUpperCase for DriverType {}

impl SqlReplaceCase for DriverType {}
