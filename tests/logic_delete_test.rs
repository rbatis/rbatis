#[cfg(test)]
mod tests {
    use rbatis::plugin::logic_delete::{RbatisLogicDeletePlugin, LogicDelete};
    use rbatis::core::db::DriverType;

    #[test]
    fn test_logic_delete_plugin_create_sql() {
        let r = RbatisLogicDeletePlugin::new("del");
        let table_fields = "name,age,del".to_string();
        let sql_where = "";
        let result = r
            .create_remove_sql(&DriverType::Mysql, "test", &table_fields, sql_where)
            .unwrap();
        assert_eq!("UPDATE test SET del = 1", &result);

        let sql_where = " WHERE name = 'zhangsan'";
        let result = r
            .create_remove_sql(&DriverType::Mysql, "test", &table_fields, sql_where)
            .unwrap();
        assert_eq!("UPDATE test SET del = 1 WHERE name = 'zhangsan'", &result);

        let table_fields = "name,age";
        let sql_where = " WHERE name = 'zhangsan'";
        let result = r
            .create_remove_sql(&DriverType::Mysql, "test", &table_fields, sql_where)
            .unwrap();
        assert_eq!("DELETE FROM test WHERE name = 'zhangsan'", &result);

        let table_fields = "name,age";
        let sql_where = "";
        let result = r.create_remove_sql(&DriverType::Mysql, "test", &table_fields, sql_where);
        assert!(result.is_err());
    }
}