use crate::core::db::DriverType;

/// sql to up case
pub trait SqlUpperCase {
    fn to_upper_case(&self, sql: &str) -> String {
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

impl SqlUpperCase for DriverType {}