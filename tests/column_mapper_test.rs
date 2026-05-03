/// Tests for table_sync ColumnMapper implementations:
/// - MysqlTableMapper (all Value types -> SQL types)
/// - PGTableMapper (all Value types -> SQL types)
/// - SqliteTableMapper (all Value types -> SQL types)
/// - MssqlTableMapper (all Value types -> SQL types)
/// - driver_type() for each mapper
/// - Edge cases (empty strings, special column naming)

#[cfg(test)]
mod test {
    use rbatis::table_sync::{
        ColumnMapper, MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper,
    };
    use rbs::Value;

    // ==================== MysqlTableMapper Tests ====================

    #[test]
    fn test_mysql_driver_type() {
        let mapper = MysqlTableMapper {};
        assert_eq!(mapper.driver_type(), "mysql");
    }

    #[test]
    fn test_mysql_null_type() {
        let mapper = MysqlTableMapper {};
        assert_eq!(mapper.get_column_type("col", &Value::Null), "NULL");
    }

    #[test]
    fn test_mysql_bool_type() {
        let mapper = MysqlTableMapper {};
        assert_eq!(mapper.get_column_type("col", &Value::Bool(true)), "TINYINT");
    }

    #[test]
    fn test_mysql_int_types() {
        let mapper = MysqlTableMapper {};
        assert_eq!(mapper.get_column_type("col", &Value::I32(0)), "INT");
        assert_eq!(mapper.get_column_type("col", &Value::I64(0)), "BIGINT");
        assert_eq!(mapper.get_column_type("col", &Value::U32(0)), "INT");
        assert_eq!(mapper.get_column_type("col", &Value::U64(0)), "BIGINT");
    }

    #[test]
    fn test_mysql_float_types() {
        let mapper = MysqlTableMapper {};
        assert_eq!(mapper.get_column_type("col", &Value::F32(0.0)), "FLOAT");
        assert_eq!(mapper.get_column_type("col", &Value::F64(0.0)), "DOUBLE");
    }

    #[test]
    fn test_mysql_string_empty_becomes_varchar() {
        let mapper = MysqlTableMapper {};
        assert_eq!(
            mapper.get_column_type("name", &Value::String("".to_string())),
            "VARCHAR(100)"
        );
        assert_eq!(
            mapper.get_column_type("id", &Value::String("".to_string())),
            "VARCHAR(50)"
        );
        assert_eq!(
            mapper.get_column_type("user_id", &Value::String("".to_string())),
            "VARCHAR(50)"
        );
        assert_eq!(
            mapper.get_column_type("id_code", &Value::String("".to_string())),
            "VARCHAR(50)"
        );
    }

    #[test]
    fn test_mysql_string_nonempty_passthrough() {
        let mapper = MysqlTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::String("VARCHAR(255)".to_string())),
            "VARCHAR(255)"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::String("TEXT".to_string())),
            "TEXT"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::String("LONGTEXT".to_string())),
            "LONGTEXT"
        );
    }

    #[test]
    fn test_mysql_binary_blob() {
        let mapper = MysqlTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::Binary(vec![1, 2, 3])),
            "BLOB"
        );
    }

    #[test]
    fn test_mysql_array_map_json() {
        let mapper = MysqlTableMapper {};
        assert_eq!(mapper.get_column_type("col", &Value::Array(vec![])), "JSON");
        assert_eq!(
            mapper.get_column_type("col", &Value::Map(rbs::value::map::ValueMap::new())),
            "JSON"
        );
    }

    #[test]
    fn test_mysql_ext_types() {
        let mapper = MysqlTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Date", Box::new(Value::Null))),
            "DATE"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("DateTime", Box::new(Value::Null))),
            "DATETIME"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Time", Box::new(Value::Null))),
            "TIME"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Timestamp", Box::new(Value::Null))),
            "TIMESTAMP"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Decimal", Box::new(Value::Null))),
            "DECIMAL"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Json", Box::new(Value::Null))),
            "JSON"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Uuid", Box::new(Value::Null))),
            "VARCHAR(50)"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("UnknownType", Box::new(Value::Null))),
            "NULL"
        );
    }

    // ==================== PGTableMapper Tests ====================

    #[test]
    fn test_pg_driver_type() {
        let mapper = PGTableMapper {};
        assert_eq!(mapper.driver_type(), "postgres");
    }

    #[test]
    fn test_pg_basic_types() {
        let mapper = PGTableMapper {};
        assert_eq!(mapper.get_column_type("col", &Value::Null), "NULL");
        assert_eq!(mapper.get_column_type("col", &Value::Bool(true)), "BOOLEAN");
        assert_eq!(mapper.get_column_type("col", &Value::I32(0)), "INTEGER");
        assert_eq!(mapper.get_column_type("col", &Value::I64(0)), "BIGINT");
        assert_eq!(mapper.get_column_type("col", &Value::U32(0)), "INTEGER");
        assert_eq!(mapper.get_column_type("col", &Value::U64(0)), "BIGINT");
        assert_eq!(mapper.get_column_type("col", &Value::F32(0.0)), "REAL");
        assert_eq!(
            mapper.get_column_type("col", &Value::F64(0.0)),
            "DOUBLE PRECISION"
        );
    }

    #[test]
    fn test_pg_string_empty_becomes_text_or_varchar() {
        let mapper = PGTableMapper {};
        assert_eq!(
            mapper.get_column_type("name", &Value::String("".to_string())),
            "TEXT"
        );
        assert_eq!(
            mapper.get_column_type("id", &Value::String("".to_string())),
            "VARCHAR(50)"
        );
    }

    #[test]
    fn test_pg_string_nonempty_passthrough() {
        let mapper = PGTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::String("VARCHAR(100)".to_string())),
            "VARCHAR(100)"
        );
    }

    #[test]
    fn test_pg_binary_bytea() {
        let mapper = PGTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::Binary(vec![])),
            "BYTEA"
        );
    }

    #[test]
    fn test_pg_ext_types() {
        let mapper = PGTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Date", Box::new(Value::Null))),
            "DATE"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("DateTime", Box::new(Value::Null))),
            "TIMESTAMPTZ"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Time", Box::new(Value::Null))),
            "TIME"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Timestamp", Box::new(Value::Null))),
            "TIMESTAMP"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Decimal", Box::new(Value::Null))),
            "NUMERIC"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Json", Box::new(Value::Null))),
            "JSON"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Uuid", Box::new(Value::Null))),
            "UUID"
        );
    }

    // ==================== SqliteTableMapper Tests ====================

    #[test]
    fn test_sqlite_driver_type() {
        let mapper = SqliteTableMapper {};
        assert_eq!(mapper.driver_type(), "sqlite");
    }

    #[test]
    fn test_sqlite_basic_types() {
        let mapper = SqliteTableMapper {};
        assert_eq!(mapper.get_column_type("col", &Value::Null), "NULL");
        assert_eq!(mapper.get_column_type("col", &Value::Bool(true)), "BOOLEAN");
        assert_eq!(mapper.get_column_type("col", &Value::I32(0)), "INTEGER");
        assert_eq!(mapper.get_column_type("col", &Value::I64(0)), "INT8");
        assert_eq!(mapper.get_column_type("col", &Value::U32(0)), "INTEGER");
        assert_eq!(mapper.get_column_type("col", &Value::U64(0)), "INT8");
        assert_eq!(mapper.get_column_type("col", &Value::F32(0.0)), "DOUBLE");
        assert_eq!(mapper.get_column_type("col", &Value::F64(0.0)), "DOUBLE");
    }

    #[test]
    fn test_sqlite_string_always_text_when_empty() {
        let mapper = SqliteTableMapper {};
        assert_eq!(
            mapper.get_column_type("id", &Value::String("".to_string())),
            "TEXT"
        );
        assert_eq!(
            mapper.get_column_type("name", &Value::String("".to_string())),
            "TEXT"
        );
    }

    #[test]
    fn test_sqlite_string_nonempty_passthrough() {
        let mapper = SqliteTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::String("NVARCHAR(50)".to_string())),
            "NVARCHAR(50)"
        );
    }

    #[test]
    fn test_sqlite_binary_array_map_are_blob() {
        let mapper = SqliteTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::Binary(vec![])),
            "BLOB"
        );
        assert_eq!(mapper.get_column_type("col", &Value::Array(vec![])), "BLOB");
        assert_eq!(
            mapper.get_column_type("col", &Value::Map(rbs::value::map::ValueMap::new())),
            "BLOB"
        );
    }

    #[test]
    fn test_sqlite_ext_types() {
        let mapper = SqliteTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Date", Box::new(Value::Null))),
            "TEXT"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("DateTime", Box::new(Value::Null))),
            "TEXT"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Time", Box::new(Value::Null))),
            "TEXT"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Timestamp", Box::new(Value::Null))),
            "INT8"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Decimal", Box::new(Value::Null))),
            "NUMERIC"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Json", Box::new(Value::Null))),
            "BLOB"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Uuid", Box::new(Value::Null))),
            "TEXT"
        );
    }

    // ==================== MssqlTableMapper Tests ====================

    #[test]
    fn test_mssql_driver_type() {
        let mapper = MssqlTableMapper {};
        assert_eq!(mapper.driver_type(), "mssql");
    }

    #[test]
    fn test_mssql_basic_types() {
        let mapper = MssqlTableMapper {};
        assert_eq!(mapper.get_column_type("col", &Value::Null), "NULL");
        assert_eq!(mapper.get_column_type("col", &Value::Bool(true)), "BIT");
        assert_eq!(mapper.get_column_type("col", &Value::I32(0)), "INT");
        assert_eq!(mapper.get_column_type("col", &Value::I64(0)), "BIGINT");
        assert_eq!(mapper.get_column_type("col", &Value::U32(0)), "INT");
        assert_eq!(mapper.get_column_type("col", &Value::U64(0)), "BIGINT");
        assert_eq!(mapper.get_column_type("col", &Value::F32(0.0)), "REAL");
        assert_eq!(mapper.get_column_type("col", &Value::F64(0.0)), "FLOAT");
    }

    #[test]
    fn test_mssql_string_empty_nvarchar_max() {
        let mapper = MssqlTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::String("".to_string())),
            "NVARCHAR(MAX)"
        );
    }

    #[test]
    fn test_mssql_binary_varbinary_max() {
        let mapper = MssqlTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::Binary(vec![])),
            "VARBINARY(MAX)"
        );
    }

    #[test]
    fn test_mssql_ext_types() {
        let mapper = MssqlTableMapper {};
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Date", Box::new(Value::Null))),
            "DATE"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("DateTime", Box::new(Value::Null))),
            "DATETIME2"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Time", Box::new(Value::Null))),
            "TIME"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Timestamp", Box::new(Value::Null))),
            "DATETIME2"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Decimal", Box::new(Value::Null))),
            "DECIMAL"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Json", Box::new(Value::Null))),
            "NVARCHAR(MAX)"
        );
        assert_eq!(
            mapper.get_column_type("col", &Value::Ext("Uuid", Box::new(Value::Null))),
            "NVARCHAR(36)"
        );
    }

    // ==================== Cross-mapper comparison tests ====================

    #[test]
    fn test_same_input_different_mappers_produce_different_sql() {
        let mysql = MysqlTableMapper {};
        let pg = PGTableMapper {};
        let sqlite = SqliteTableMapper {};
        let mssql = MssqlTableMapper {};

        let val = Value::Bool(true);
        assert_ne!(
            mysql.get_column_type("col", &val),
            pg.get_column_type("col", &val)
        );
        assert_eq!(mysql.get_column_type("col", &val), "TINYINT");
        assert_eq!(pg.get_column_type("col", &val), "BOOLEAN");
        assert_eq!(sqlite.get_column_type("col", &val), "BOOLEAN");
        assert_eq!(mssql.get_column_type("col", &val), "BIT");
    }

    #[test]
    fn test_integer_types_differ_across_databases() {
        let mysql = MysqlTableMapper {};
        let pg = PGTableMapper {};
        let sqlite = SqliteTableMapper {};
        let mssql = MssqlTableMapper {};

        let val = Value::I64(0);
        assert_eq!(mysql.get_column_type("col", &val), "BIGINT");
        assert_eq!(pg.get_column_type("col", &val), "BIGINT");
        assert_eq!(sqlite.get_column_type("col", &val), "INT8");
        assert_eq!(mssql.get_column_type("col", &val), "BIGINT");

        let val_i32 = Value::I32(0);
        assert_eq!(mysql.get_column_type("col", &val_i32), "INT");
        assert_eq!(pg.get_column_type("col", &val_i32), "INTEGER");
        assert_eq!(sqlite.get_column_type("col", &val_i32), "INTEGER");
        assert_eq!(mssql.get_column_type("col", &val_i32), "INT");
    }

    #[test]
    fn test_float_types_differ_across_databases() {
        let mysql = MysqlTableMapper {};
        let pg = PGTableMapper {};
        let sqlite = SqliteTableMapper {};
        let mssql = MssqlTableMapper {};

        let val_f64 = Value::F64(0.0);
        assert_eq!(mysql.get_column_type("col", &val_f64), "DOUBLE");
        assert_eq!(pg.get_column_type("col", &val_f64), "DOUBLE PRECISION");
        assert_eq!(sqlite.get_column_type("col", &val_f64), "DOUBLE");
        assert_eq!(mssql.get_column_type("col", &val_f64), "FLOAT");
    }

    #[test]
    fn test_string_empty_differs_per_db_convention() {
        let mysql = MysqlTableMapper {};
        let pg = PGTableMapper {};
        let sqlite = SqliteTableMapper {};
        let mssql = MssqlTableMapper {};

        let val = Value::String("".to_string());

        assert_eq!(mysql.get_column_type("name", &val), "VARCHAR(100)");
        assert_eq!(pg.get_column_type("name", &val), "TEXT");
        assert_eq!(sqlite.get_column_type("name", &val), "TEXT");
        assert_eq!(mssql.get_column_type("name", &val), "NVARCHAR(MAX)");

        assert_eq!(mysql.get_column_type("id", &val), "VARCHAR(50)");
        assert_eq!(pg.get_column_type("id", &val), "VARCHAR(50)");
        assert_eq!(sqlite.get_column_type("id", &val), "TEXT");
        assert_eq!(mssql.get_column_type("id", &val), "NVARCHAR(MAX)");
    }

    #[test]
    fn test_datetime_ext_differs_across_databases() {
        let mysql = MysqlTableMapper {};
        let pg = PGTableMapper {};
        let sqlite = SqliteTableMapper {};
        let mssql = MssqlTableMapper {};

        let val = Value::Ext("DateTime", Box::new(Value::Null));
        assert_eq!(mysql.get_column_type("col", &val), "DATETIME");
        assert_eq!(pg.get_column_type("col", &val), "TIMESTAMPTZ");
        assert_eq!(sqlite.get_column_type("col", &val), "TEXT");
        assert_eq!(mssql.get_column_type("col", &val), "DATETIME2");
    }

    // ==================== Mapper trait object usage ====================

    #[test]
    fn test_column_mapper_as_trait_object() {
        let mappers: Vec<(&str, &dyn ColumnMapper)> = vec![
            ("mysql", &MysqlTableMapper {}),
            ("postgres", &PGTableMapper {}),
            ("sqlite", &SqliteTableMapper {}),
            ("mssql", &MssqlTableMapper {}),
        ];

        for (expected_name, mapper) in &mappers {
            assert_eq!(mapper.driver_type(), *expected_name);
        }
    }

    #[test]
    fn test_mappers_are_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<MysqlTableMapper>();
        assert_send_sync::<PGTableMapper>();
        assert_send_sync::<SqliteTableMapper>();
        assert_send_sync::<MssqlTableMapper>();
    }
}
