use crate::DriverType;

pub trait SqlRule {
    fn make_where(&self, where_sql: &str) -> String {
        let sql = where_sql.trim_start();
        if sql.is_empty() {
            return String::new();
        }
        if sql.starts_with(crate::sql::TEMPLATE.order_by.trim_start())
            || sql.starts_with(crate::sql::TEMPLATE.group_by.trim_start())
            || sql.starts_with(crate::sql::TEMPLATE.limit.trim_start())
        {
            sql.to_string()
        } else {
            format!(
                "{}{} ",
                crate::sql::TEMPLATE.r#where,
                sql.trim_start_matches(crate::sql::TEMPLATE.r#where.trim_start())
                    .trim_start_matches(crate::sql::TEMPLATE.and.trim_start())
                    .trim_start_matches(crate::sql::TEMPLATE.or.trim_start())
            )
        }
    }

    fn make_left_insert_where(&self, insert_sql: &str, where_sql: &str) -> String {
        let sql = where_sql
            .trim()
            .trim_start_matches(crate::sql::TEMPLATE.r#where.trim_start())
            .trim_start_matches(crate::sql::TEMPLATE.and.trim_start());
        if sql.is_empty() {
            return insert_sql.to_string();
        }
        if sql.starts_with(crate::sql::TEMPLATE.order_by.trim_start())
            || sql.starts_with(crate::sql::TEMPLATE.group_by.trim_start())
            || sql.starts_with(crate::sql::TEMPLATE.limit.trim_start()) {
            format!(
                "{}{} {}",
                crate::sql::TEMPLATE.r#where,
                insert_sql.trim().trim_end_matches(crate::sql::TEMPLATE.and.trim_end()),
                sql
            )
        } else {
            format!(
                " {} {} {} {}",
                crate::sql::TEMPLATE.r#where,
                insert_sql.trim().trim_end_matches(crate::sql::TEMPLATE.and.trim_end()),
                crate::sql::TEMPLATE.and,
                sql
            )
        }
    }
}

impl SqlRule for DriverType {}
