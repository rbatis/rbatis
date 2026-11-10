//! Coverage tests for table_sync (sync + deprecated mappers + RBatis mapper),
//! executor interceptor / error paths, crud! macro branches, crud_traits,
//! decode.rs and rbatis.rs gaps.

#[cfg(test)]
mod test {
    #![allow(deprecated)]

    use async_trait::async_trait;
    use rbatis::crud_traits::{FilterByColumns, ValueOperatorSql};
    use rbatis::executor::Executor;
    use rbatis::intercept::{Intercept, ResultType};
    use rbatis::table_sync::deprecated::{
        MssqlTableMapper, MysqlTableMapper, PGTableMapper, SqliteTableMapper,
    };
    use rbatis::table_sync::{sync, ColumnMapper};
    use rbatis::{Action, DefaultPool, Error, PageRequest, RBatis};
    use rbdc::db::ExecResult;
    use rbdc_sqlite::{SqliteConnectOptions, SqliteDriver};
    use rbs::value::map::ValueMap;
    use rbs::{value, Value};
    use std::sync::Arc;

    // ==================== helpers ====================

    /// linked sqlite engine with the default interceptors (PageIntercept + Log)
    async fn linked_rb() -> RBatis {
        let rb = RBatis::new();
        rb.link(SqliteDriver {}, "sqlite://:memory:")
            .await
            .unwrap();
        rb
    }

    /// linked sqlite engine WITHOUT interceptors (fast paths)
    fn bare_rb() -> RBatis {
        let rb = RBatis::default();
        rb.init(SqliteDriver {}, "sqlite://:memory:").unwrap();
        rb
    }

    // ==================== table_sync::sync ====================

    #[tokio::test]
    async fn test_sync_create_table_with_literal_id_primary_key() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        // the literal string "id" as a value takes the `v == "id"` primary-key branch
        sync(
            &conn,
            &rb,
            value! {"id": "id", "name": "TEXT"},
            "sync_literal_id",
        )
        .await
        .unwrap();
        // the table and its columns exist
        let rows = conn
            .query("select id, name from sync_literal_id", vec![])
            .await
            .unwrap();
        assert_eq!(rows, Value::Array(vec![]));
    }

    #[tokio::test]
    async fn test_sync_create_table_with_column_type_string() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        sync(
            &conn,
            &rb,
            value! {"id": "INT", "name": "TEXT", "age": "INT"},
            "sync_types",
        )
        .await
        .unwrap();
        let rows = conn
            .query("select id, name, age from sync_types", vec![])
            .await
            .unwrap();
        assert_eq!(rows, Value::Array(vec![]));
    }

    /// A mapper whose column type is empty for `id` (and only for `id`) takes the
    /// `column_type_value.is_empty() && k == "id"` primary-key branch.
    struct EmptyIdTypeMapper;

    impl ColumnMapper for EmptyIdTypeMapper {
        fn driver_type(&self) -> String {
            "sqlite".to_string()
        }
        fn get_column_type(&self, column: &str, _v: &Value) -> String {
            if column == "id" {
                String::new()
            } else {
                "TEXT".to_string()
            }
        }
    }

    #[tokio::test]
    async fn test_sync_primary_key_from_empty_column_type() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        sync(
            &conn,
            &EmptyIdTypeMapper,
            value! {"id": 1i64, "name": "x"},
            "sync_empty_type",
        )
        .await
        .unwrap();
        // the column named id exists, and the table is empty
        let rows = conn
            .query("select id from sync_empty_type", vec![])
            .await
            .unwrap();
        assert_eq!(rows, Value::Array(vec![]));
    }

    #[tokio::test]
    async fn test_sync_existing_table_runs_alter_loop() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        // first call creates the table
        sync(
            &conn,
            &rb,
            value! {"id": "INT", "name": "TEXT"},
            "sync_alter",
        )
        .await
        .unwrap();
        // second call: CREATE TABLE fails with "already exists" -> ALTER loop.
        // `id` and `name` fail (duplicate column -> debug! + continue), `age` succeeds.
        sync(
            &conn,
            &rb,
            value! {"id": "INT", "name": "TEXT", "age": "INT"},
            "sync_alter",
        )
        .await
        .unwrap();
        let rows = conn
            .query("select age from sync_alter", vec![])
            .await
            .unwrap();
        assert_eq!(rows, Value::Array(vec![]));
    }

    #[tokio::test]
    async fn test_sync_non_map_value_is_error() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        let err = sync(&conn, &rb, Value::I64(1), "not_a_map")
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("table not is an struct or map"),
            "{}",
            err
        );
    }

    struct WrongDriverMapper;

    impl ColumnMapper for WrongDriverMapper {
        fn driver_type(&self) -> String {
            "mysql".to_string()
        }
        fn get_column_type(&self, _column: &str, _v: &Value) -> String {
            "TEXT".to_string()
        }
    }

    #[tokio::test]
    async fn test_sync_mapper_driver_mismatch_is_error() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        let err = sync(
            &conn,
            &WrongDriverMapper,
            value! {"id": "INT"},
            "driver_mismatch",
        )
        .await
        .unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("table sync mapper driver='mysql'"), "{}", msg);
        assert!(msg.contains("db driver='sqlite'"), "{}", msg);
    }

    // ==================== deprecated table mappers ====================

    fn assert_deprecated_mapper(m: &dyn ColumnMapper, default_text: &str) {
        assert_eq!(
            m.get_column_type("id", &Value::String(String::new())),
            "VARCHAR(50)"
        );
        assert_eq!(
            m.get_column_type("user_id", &Value::String(String::new())),
            "VARCHAR(50)"
        );
        assert_eq!(
            m.get_column_type("id_x", &Value::String(String::new())),
            "VARCHAR(50)"
        );
        assert_eq!(
            m.get_column_type("name", &Value::String(String::new())),
            default_text
        );
        assert_eq!(
            m.get_column_type("name", &Value::String("VARCHAR(9)".to_string())),
            "VARCHAR(9)"
        );
        // non-string values take the `_` arm
        assert_eq!(m.get_column_type("id", &Value::I64(1)), "TEXT");
        assert_eq!(m.get_column_type("name", &Value::Bool(true)), "TEXT");
        assert_eq!(m.get_column_type("name", &Value::Null), "TEXT");
    }

    #[test]
    fn test_sqlite_table_mapper() {
        let mapper = SqliteTableMapper;
        assert_eq!(mapper.driver_type(), "sqlite");
        assert_deprecated_mapper(&mapper, "TEXT");
    }

    #[test]
    fn test_mysql_table_mapper() {
        let mapper = MysqlTableMapper;
        assert_eq!(mapper.driver_type(), "mysql");
        assert_deprecated_mapper(&mapper, "TEXT");
    }

    #[test]
    fn test_mssql_table_mapper() {
        let mapper = MssqlTableMapper;
        assert_eq!(mapper.driver_type(), "mssql");
        assert_deprecated_mapper(&mapper, "NVARCHAR(MAX)");
    }

    #[test]
    fn test_pg_table_mapper() {
        let mapper = PGTableMapper;
        assert_eq!(mapper.driver_type(), "postgres");
        assert_deprecated_mapper(&mapper, "TEXT");
    }

    // ==================== ColumnMapper for RBatis (rbdc_mapper.rs) ====================

    #[test]
    fn test_rbatis_column_mapper() {
        let rb = bare_rb();
        assert_eq!(ColumnMapper::driver_type(&rb), "sqlite");
        // empty string + special column names -> VARCHAR(50)
        assert_eq!(
            ColumnMapper::get_column_type(&rb, "id", &Value::String(String::new())),
            "VARCHAR(50)"
        );
        assert_eq!(
            ColumnMapper::get_column_type(&rb, "user_id", &Value::String(String::new())),
            "VARCHAR(50)"
        );
        assert_eq!(
            ColumnMapper::get_column_type(&rb, "id_x", &Value::String(String::new())),
            "VARCHAR(50)"
        );
        // ordinary column with empty string -> the driver's column type
        assert_eq!(
            ColumnMapper::get_column_type(&rb, "name", &Value::String(String::new())),
            "TEXT"
        );
        // non-empty string is used verbatim
        assert_eq!(
            ColumnMapper::get_column_type(
                &rb,
                "name",
                &Value::String("VARCHAR(9)".to_string())
            ),
            "VARCHAR(9)"
        );
        // non-string values -> driver column type
        assert_eq!(
            ColumnMapper::get_column_type(&rb, "n", &Value::I64(1)),
            "INTEGER"
        );
        assert_eq!(
            ColumnMapper::get_column_type(&rb, "b", &Value::Bool(true)),
            "BOOLEAN"
        );
        assert_eq!(
            ColumnMapper::get_column_type(&rb, "x", &Value::Null),
            "NULL"
        );
    }

    // ==================== executor interceptor paths ====================

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Phase {
        Before,
        After,
    }

    /// Interceptor that writes an answer into the result slot and returns `Action::Return`
    /// either from `before` or from `after`.
    #[derive(Debug)]
    struct ReturnResult {
        phase: Phase,
        exec: bool,
    }

    #[async_trait]
    impl Intercept for ReturnResult {
        async fn before(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            _sql: &mut String,
            _args: &mut Vec<Value>,
            result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
        ) -> Result<Action, Error> {
            if self.phase == Phase::Before {
                match result {
                    ResultType::Exec(r) if self.exec => {
                        *r = Ok(ExecResult {
                            rows_affected: 123,
                            last_insert_id: Value::U64(9),
                        });
                    }
                    ResultType::Query(r) if !self.exec => {
                        *r = Ok(Value::Array(vec![Value::I32(7)]));
                    }
                    _ => {}
                }
                Ok(Action::Return)
            } else {
                Ok(Action::Next)
            }
        }

        async fn after(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            _sql: &mut String,
            _args: &mut Vec<Value>,
            result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
        ) -> Result<Action, Error> {
            if self.phase == Phase::After {
                match result {
                    ResultType::Exec(r) if self.exec => {
                        *r = Ok(ExecResult {
                            rows_affected: 55,
                            last_insert_id: Value::U64(5),
                        });
                    }
                    ResultType::Query(r) if !self.exec => {
                        *r = Ok(Value::Array(vec![Value::I32(42)]));
                    }
                    _ => {}
                }
                Ok(Action::Return)
            } else {
                Ok(Action::Next)
            }
        }
    }

    fn rb_with_interceptor(phase: Phase, exec: bool) -> RBatis {
        let mut rb = RBatis::new();
        rb.set_intercepts(vec![Arc::new(ReturnResult { phase, exec })]);
        rb.init(SqliteDriver {}, "sqlite://:memory:").unwrap();
        rb
    }

    // ---- RBatisConnExecutor ----

    #[tokio::test]
    async fn test_conn_exec_before_return() {
        let rb = rb_with_interceptor(Phase::Before, true);
        let conn = rb.acquire().await.unwrap();
        let r = conn.exec("insert into t values (1)", vec![]).await.unwrap();
        assert_eq!(r.rows_affected, 123);
        assert_eq!(r.last_insert_id, Value::U64(9));
    }

    #[tokio::test]
    async fn test_conn_query_before_return() {
        let rb = rb_with_interceptor(Phase::Before, false);
        let conn = rb.acquire().await.unwrap();
        let v = conn.query("select 1", vec![]).await.unwrap();
        assert_eq!(v, Value::Array(vec![Value::I32(7)]));
    }

    #[tokio::test]
    async fn test_conn_exec_decode_before_return() {
        let rb = rb_with_interceptor(Phase::Before, false);
        let conn = rb.acquire().await.unwrap();
        let v: Vec<i32> = conn.exec_decode("select 1", vec![]).await.unwrap();
        assert_eq!(v, vec![7]);
    }

    #[tokio::test]
    async fn test_conn_exec_after_return() {
        let rb = rb_with_interceptor(Phase::After, true);
        let conn = rb.acquire().await.unwrap();
        // the statement runs, then the after interceptor replaces the result
        let r = conn
            .exec(
                "create table if not exists after_conn_exec (id integer)",
                vec![],
            )
            .await
            .unwrap();
        assert_eq!(r.rows_affected, 55);
    }

    #[tokio::test]
    async fn test_conn_query_after_return() {
        let rb = rb_with_interceptor(Phase::After, false);
        let conn = rb.acquire().await.unwrap();
        let v = conn.query("select 1", vec![]).await.unwrap();
        assert_eq!(v, Value::Array(vec![Value::I32(42)]));
    }

    #[tokio::test]
    async fn test_conn_exec_decode_after_return() {
        let rb = rb_with_interceptor(Phase::After, false);
        let conn = rb.acquire().await.unwrap();
        let v: Vec<i32> = conn.exec_decode("select 1", vec![]).await.unwrap();
        assert_eq!(v, vec![42]);
    }

    #[tokio::test]
    async fn test_conn_exec_decode_fast_path_without_interceptors() {
        let rb = bare_rb();
        let conn = rb.acquire().await.unwrap();
        let n: i64 = conn.exec_decode("select 1 as n", vec![]).await.unwrap();
        assert_eq!(n, 1);
    }

    // ---- RBatisTxExecutor ----

    #[tokio::test]
    async fn test_tx_exec_before_return() {
        let rb = rb_with_interceptor(Phase::Before, true);
        let tx = rb.acquire_begin().await.unwrap();
        let r = tx.exec("insert into t values (1)", vec![]).await.unwrap();
        assert_eq!(r.rows_affected, 123);
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_tx_query_before_return() {
        let rb = rb_with_interceptor(Phase::Before, false);
        let tx = rb.acquire_begin().await.unwrap();
        let v = tx.query("select 1", vec![]).await.unwrap();
        assert_eq!(v, Value::Array(vec![Value::I32(7)]));
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_tx_exec_decode_before_return() {
        let rb = rb_with_interceptor(Phase::Before, false);
        let tx = rb.acquire_begin().await.unwrap();
        let v: Vec<i32> = tx.exec_decode("select 1", vec![]).await.unwrap();
        assert_eq!(v, vec![7]);
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_tx_exec_after_return() {
        let rb = rb_with_interceptor(Phase::After, true);
        let tx = rb.acquire_begin().await.unwrap();
        let r = tx
            .exec(
                "create table if not exists after_tx_exec (id integer)",
                vec![],
            )
            .await
            .unwrap();
        assert_eq!(r.rows_affected, 55);
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_tx_query_after_return() {
        let rb = rb_with_interceptor(Phase::After, false);
        let tx = rb.acquire_begin().await.unwrap();
        let v = tx.query("select 1", vec![]).await.unwrap();
        assert_eq!(v, Value::Array(vec![Value::I32(42)]));
        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_tx_exec_decode_after_return() {
        let rb = rb_with_interceptor(Phase::After, false);
        let tx = rb.acquire_begin().await.unwrap();
        let v: Vec<i32> = tx.exec_decode("select 1", vec![]).await.unwrap();
        assert_eq!(v, vec![42]);
        tx.rollback().await.unwrap();
    }

    // ---- RBatis::exec_decode fast / interceptor paths ----

    #[tokio::test]
    async fn test_rbatis_exec_decode_fast_path_without_interceptors() {
        let rb = bare_rb();
        let n: i64 = rb.exec_decode("select 1 as n", vec![]).await.unwrap();
        assert_eq!(n, 1);
    }

    #[tokio::test]
    async fn test_rbatis_exec_decode_with_after_interceptor() {
        let rb = rb_with_interceptor(Phase::After, false);
        let v: Vec<i32> = rb.exec_decode("select 1", vec![]).await.unwrap();
        assert_eq!(v, vec![42]);
    }

    #[tokio::test]
    async fn test_rbatis_exec_decode_unlinked_no_interceptors() {
        let rb = RBatis::default();
        let err = rb
            .exec_decode::<Vec<i32>>("select 1", vec![])
            .await
            .unwrap_err();
        assert!(err.to_string().contains("pool not inited"), "{}", err);
    }

    #[tokio::test]
    async fn test_rbatis_exec_decode_unlinked_with_interceptor() {
        let mut rb = RBatis::default();
        rb.set_intercepts(vec![Arc::new(ReturnResult {
            phase: Phase::After,
            exec: false,
        })]);
        let err = rb
            .exec_decode::<Vec<i32>>("select 1", vec![])
            .await
            .unwrap_err();
        assert!(err.to_string().contains("pool not inited"), "{}", err);
    }

    // ---- Debug impls, take_connection / begin error paths ----

    #[tokio::test]
    async fn test_conn_executor_debug() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        let id = conn.id;
        let s = format!("{:?}", conn);
        assert!(s.contains("RBatisConnExecutor"), "{}", s);
        assert!(s.contains(&id.to_string()), "{}", s);
    }

    #[tokio::test]
    async fn test_tx_guard_debug() {
        let rb = linked_rb().await;
        let tx = rb.acquire_begin().await.unwrap();
        let guard = tx.defer_async(|_tx| async {});
        let s = format!("{:?}", guard);
        assert!(s.contains("RBatisTxExecutorGuard"), "{}", s);
        assert!(s.contains("RBatisTxExecutor"), "{}", s);
    }

    #[tokio::test]
    async fn test_conn_take_connection_shared_returns_none() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        let clone = conn.clone();
        // the Arc is still shared by the clone, so it cannot be extracted
        assert!(conn.take_connection().is_none());
        drop(clone);
    }

    #[tokio::test]
    async fn test_conn_begin_fails_when_connection_is_shared() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        let clone = conn.clone();
        let err = conn.begin().await.unwrap_err();
        assert!(
            err.to_string().contains("failed to take connection"),
            "{}",
            err
        );
        drop(clone);
    }

    #[tokio::test]
    async fn test_conn_begin_commit_and_take_connection() {
        let rb = linked_rb().await;
        let conn = rb.acquire().await.unwrap();
        let tx = conn.begin().await.unwrap();
        tx.commit().await.unwrap();
        // after commit, a fresh unique connection can be taken out of its executor
        let conn = rb.acquire().await.unwrap();
        assert!(conn.take_connection().is_some());
    }

    #[tokio::test]
    async fn test_tx_take_connection_some() {
        let rb = linked_rb().await;
        let tx = rb.acquire_begin().await.unwrap();
        assert!(tx.take_connection().is_some());
    }

    // ---- deprecated query_decode ----

    #[tokio::test]
    async fn test_deprecated_query_decode() {
        let rb = bare_rb();
        #[allow(deprecated)]
        let n: i64 = rb.query_decode("select 1 as n", vec![]).await.unwrap();
        assert_eq!(n, 1);
    }

    // ---- RBatisTxExecutorGuard helpers ----

    #[tokio::test]
    async fn test_tx_guard_helpers() {
        let rb = linked_rb().await;
        let tx = rb.acquire_begin().await.unwrap();
        let expected_id = tx.tx_id;
        let guard = tx.defer_async(|_tx| async {});
        assert_eq!(guard.tx_id(), expected_id);
        assert_eq!(Executor::id(&guard), expected_id);

        // guard::exec_decode delegates to the transaction
        tx.exec("create table if not exists guard_t (id integer)", vec![])
            .await
            .unwrap();
        let n: i64 = guard.exec_decode("select 1 as n", vec![]).await.unwrap();
        assert_eq!(n, 1);

        guard.commit().await.unwrap();
        assert!(tx.done());
    }

    #[tokio::test]
    async fn test_tx_guard_rollback() {
        let rb = linked_rb().await;
        let tx = rb.acquire_begin().await.unwrap();
        let guard = tx.defer_async(|_tx| async {});
        guard.rollback().await.unwrap();
        assert!(tx.done());
    }

    /// The guard keeps a clone of the transaction alive, so `take_connection` can
    /// never extract the connection (the Arc is always shared). This documents
    /// the actual behaviour of the implementation.
    #[tokio::test]
    async fn test_tx_guard_take_connection_is_none() {
        let rb = linked_rb().await;
        let tx = rb.acquire_begin().await.unwrap();
        let guard = tx.defer_async(|_tx| async {});
        assert!(guard.take_connection().is_none());
        drop(tx);
    }

    // ==================== crud! macro branches ====================

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct CrudUser {
        id: Option<i64>,
        name: Option<String>,
        age: Option<i64>,
    }

    rbatis::crud!(CrudUser {});

    #[tokio::test]
    async fn test_crud_insert_and_empty_batch_error() {
        let rb = bare_rb();
        rb.exec(
            "create table crud_user (id integer, name text, age integer)",
            vec![],
        )
        .await
        .unwrap();
        let user = CrudUser {
            id: Some(1),
            name: Some("alice".to_string()),
            age: Some(20),
        };
        CrudUser::insert(&rb, &user).await.unwrap();
        // empty slice -> explicit error branch (lines 153-154)
        let err = CrudUser::insert_batch(&rb, &[], 10).await.unwrap_err();
        assert!(
            err.to_string().contains("insert can not insert empty array"),
            "{}",
            err
        );
        // two rows in one batch with batch_size 0 (all in one statement)
        let user2 = CrudUser {
            id: Some(2),
            name: Some("bob".to_string()),
            age: Some(30),
        };
        let r = CrudUser::insert_batch(&rb, &[user.clone(), user2], 0)
            .await
            .unwrap();
        assert_eq!(r.rows_affected, 2);
    }

    #[tokio::test]
    async fn test_crud_select_by_map_column_variants() {
        let rb = bare_rb();
        rb.exec(
            "create table crud_user (id integer, name text, age integer)",
            vec![],
        )
        .await
        .unwrap();
        CrudUser::insert(
            &rb,
            &CrudUser {
                id: Some(1),
                name: Some("alice".to_string()),
                age: Some(20),
            },
        )
        .await
        .unwrap();

        // "column" as a String
        let rows = CrudUser::select_by_map(&rb, value! {"id": 1i64, "column": "name"})
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name.as_deref(), Some("alice"));

        // "column" as a non-empty Array
        let rows = CrudUser::select_by_map(&rb, value! {"id": 1i64, "column": ["id", "name"]})
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, Some(1));

        // "column" as an empty Array -> "*"
        let empty: Vec<String> = vec![];
        let rows = CrudUser::select_by_map(&rb, value! {"id": 1i64, "column": empty})
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].age, Some(20));

        // "column" as another kind -> "*" (line 228)
        let rows = CrudUser::select_by_map(&rb, value! {"id": 1i64, "column": 7i64})
            .await
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name.as_deref(), Some("alice"));
    }

    #[tokio::test]
    async fn test_crud_update_by_map_column_variants() {
        let rb = bare_rb();
        rb.exec(
            "create table crud_user (id integer, name text, age integer)",
            vec![],
        )
        .await
        .unwrap();
        CrudUser::insert(
            &rb,
            &CrudUser {
                id: Some(1),
                name: Some("alice".to_string()),
                age: Some(20),
            },
        )
        .await
        .unwrap();

        // "column" as a String -> only that column is updated (line 311)
        let user = CrudUser {
            id: Some(1),
            name: Some("alice2".to_string()),
            age: Some(99),
        };
        CrudUser::update_by_map(&rb, &user, value! {"id": 1i64, "column": "name"})
            .await
            .unwrap();
        let rows = CrudUser::select_by_map(&rb, value! {"id": 1i64})
            .await
            .unwrap();
        assert_eq!(rows[0].name.as_deref(), Some("alice2"));
        assert_eq!(rows[0].age, Some(20)); // age untouched

        // "column" as another kind -> Null -> full update (line 325)
        let user = CrudUser {
            id: Some(1),
            name: Some("alice3".to_string()),
            age: Some(33),
        };
        CrudUser::update_by_map(&rb, &user, value! {"id": 1i64, "column": 7i64})
            .await
            .unwrap();
        let rows = CrudUser::select_by_map(&rb, value! {"id": 1i64})
            .await
            .unwrap();
        assert_eq!(rows[0].name.as_deref(), Some("alice3"));
        assert_eq!(rows[0].age, Some(33));

        // "column" as an empty Array -> Null -> full update
        let empty: Vec<String> = vec![];
        let user = CrudUser {
            id: Some(1),
            name: Some("alice4".to_string()),
            age: Some(44),
        };
        CrudUser::update_by_map(&rb, &user, value! {"id": 1i64, "column": empty})
            .await
            .unwrap();
        let rows = CrudUser::select_by_map(&rb, value! {"id": 1i64})
            .await
            .unwrap();
        assert_eq!(rows[0].name.as_deref(), Some("alice4"));
        assert_eq!(rows[0].age, Some(44));
    }

    #[tokio::test]
    async fn test_crud_condition_key_as_ext_uses_operator_sql() {
        let rb = bare_rb();
        rb.exec(
            "create table crud_user (id integer, name text, age integer)",
            vec![],
        )
        .await
        .unwrap();
        CrudUser::insert(
            &rb,
            &CrudUser {
                id: Some(1),
                name: Some("alice".to_string()),
                age: Some(20),
            },
        )
        .await
        .unwrap();
        CrudUser::insert(
            &rb,
            &CrudUser {
                id: Some(2),
                name: Some("bob".to_string()),
                age: Some(30),
            },
        )
        .await
        .unwrap();

        // An Ext key makes ValueOperatorSql recurse into the inner string (line 90)
        let mut map = ValueMap::new();
        map.insert(
            Value::Ext("Reflect", Box::new(Value::String("name".to_string()))),
            value!("bob"),
        );
        let r = CrudUser::delete_by_map(&rb, Value::Map(map))
            .await
            .unwrap();
        assert_eq!(r.rows_affected, 1);

        let rows = CrudUser::select_by_map(&rb, value! {}).await.unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name.as_deref(), Some("alice"));
    }

    #[test]
    fn test_value_operator_sql_branches() {
        assert_eq!(value!("id").operator_sql(), "id = ");
        assert_eq!(value!("name like ").operator_sql(), "name like ");
        // Ext recurses
        assert_eq!(
            Value::Ext("Reflect", Box::new(value!("age"))).operator_sql(),
            "age = "
        );
        // non-string key -> empty
        assert_eq!(value!(1i64).operator_sql(), "");
    }

    #[test]
    fn test_filter_by_columns_branches() {
        let v = value!({"a": 1, "b": 2});
        // empty array -> empty map
        let out = v.filter_by_columns(&Value::Array(vec![]));
        assert_eq!(out, Value::Map(ValueMap::new()));
        // non-empty array without string entries -> empty map (line 126)
        let out = v.filter_by_columns(&Value::Array(vec![Value::I64(9)]));
        assert_eq!(out, Value::Map(ValueMap::new()));
        // non-array columns -> clone
        assert_eq!(v.filter_by_columns(&Value::I64(9)), v);
        // selects only the named column
        let out = v.filter_by_columns(&Value::Array(vec![value!("a")]));
        assert_eq!(out, value!({"a": 1}));
    }

    // ==================== crud page methods ====================

    rbatis::pysql_select_page!(select_crud_user_page(id: i64) -> CrudUser =>
        r#"`select `
            if do_count == true:
              ` count(1) as count `
            if do_count == false:
              ` * `
            `from crud_user where id = #{id}"#);

    async fn seed_crud_user(rb: &RBatis) {
        rb.exec(
            "create table crud_user (id integer, name text, age integer)",
            vec![],
        )
        .await
        .unwrap();
        CrudUser::insert(
            rb,
            &CrudUser {
                id: Some(1),
                name: Some("alice".to_string()),
                age: Some(20),
            },
        )
        .await
        .unwrap();
    }

    /// plain `RBatis` executor: name matches `rb_ref().name()`, so the generated
    /// function acquires a connection and runs the page statements on it.
    #[tokio::test]
    async fn test_page_plain_rbatis_executor() {
        let rb = linked_rb().await;
        seed_crud_user(&rb).await;
        let page = select_crud_user_page(&rb, &PageRequest::new(1, 10), 1)
            .await
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.records.len(), 1);
        assert_eq!(page.records[0].name.as_deref(), Some("alice"));
    }

    /// an acquired `RBatisConnExecutor`: its name differs from the engine name, so
    /// the generated function runs directly on the connection.
    #[tokio::test]
    async fn test_page_acquired_connection_executor() {
        let rb = linked_rb().await;
        seed_crud_user(&rb).await;
        let conn = rb.acquire().await.unwrap();
        let page = select_crud_user_page(&conn, &PageRequest::new(1, 10), 1)
            .await
            .unwrap();
        assert_eq!(page.total, 1);
        assert_eq!(page.records.len(), 1);
        assert_eq!(page.records[0].id, Some(1));
    }

    // ==================== decode.rs gaps ====================

    #[test]
    fn test_try_decode_elements_option_from_single_map() {
        let mut map = ValueMap::new();
        map.insert(Value::String("k".to_string()), Value::I32(42));
        let value = Value::Array(vec![Value::Map(map)]);
        let v: Option<i64> = rbatis::decode::try_decode_elements(&value).unwrap();
        assert_eq!(v, Some(42));
    }

    #[test]
    fn test_try_decode_elements_scalar_from_single_element_array() {
        let value = Value::Array(vec![Value::Array(vec![Value::I32(7)])]);
        let v: i32 = rbatis::decode::try_decode_elements(&value).unwrap();
        assert_eq!(v, 7);
    }

    #[test]
    fn test_try_decode_elements_plain_value() {
        // non-array -> direct decode (line 92)
        let v: i64 = rbatis::decode::try_decode_elements(&Value::I64(5)).unwrap();
        assert_eq!(v, 5);
    }

    #[test]
    fn test_try_decode_elements_collects_row_decode_error() {
        let mut map = ValueMap::new();
        map.insert(
            Value::String("k".to_string()),
            Value::String("not a number".to_string()),
        );
        let value = Value::Array(vec![Value::Map(map)]);
        let err = rbatis::decode::try_decode_elements::<i64>(&value).unwrap_err();
        assert!(err.to_string().contains("invalid type: string"), "{}", err);
    }

    #[test]
    fn test_try_decode_elements_no_single_column_fallback() {
        // array with no single-column rows: last_err stays None -> line 85
        let value = Value::Array(vec![Value::Array(vec![Value::I32(1), Value::I32(2)])]);
        let err = rbatis::decode::try_decode_elements::<i64>(&value).unwrap_err();
        assert!(
            err.to_string()
                .contains("unsupported single row struct decode"),
            "{}",
            err
        );
    }

    // ==================== rbatis.rs gaps ====================

    #[tokio::test]
    async fn test_link_twice_returns_already_initialized_error() {
        let rb = linked_rb().await;
        let err = rb
            .link(SqliteDriver {}, "sqlite://:memory:")
            .await
            .unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("already initialized"), "{}", msg);
        assert!(msg.contains("init can only be called once"), "{}", msg);
    }

    #[test]
    fn test_init_option_twice_returns_already_initialized_error() {
        let rb = RBatis::default();
        rb.init_option::<SqliteDriver, SqliteConnectOptions, DefaultPool>(
            SqliteDriver {},
            SqliteConnectOptions::new(),
        )
        .unwrap();
        let err = rb
            .init_option::<SqliteDriver, SqliteConnectOptions, DefaultPool>(
                SqliteDriver {},
                SqliteConnectOptions::new(),
            )
            .unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("already initialized"), "{}", msg);
        assert!(
            msg.contains("init_option can only be called once"),
            "{}",
            msg
        );
    }

    #[derive(Debug)]
    struct RealIntercept;

    #[async_trait]
    impl Intercept for RealIntercept {}

    /// Reports the type name of `RealIntercept`, but is not one.
    #[derive(Debug)]
    struct FakeNamedIntercept;

    #[async_trait]
    impl Intercept for FakeNamedIntercept {
        fn name(&self) -> &str {
            std::any::type_name::<RealIntercept>()
        }
    }

    #[test]
    fn test_get_intercept_found_and_downcast_miss() {
        let mut rb = RBatis::new();
        rb.set_intercepts(vec![Arc::new(RealIntercept)]);
        assert!(rb.get_intercept::<RealIntercept>().is_some());

        // the name matches the real type but the downcast fails -> None
        let mut rb = RBatis::new();
        rb.set_intercepts(vec![Arc::new(FakeNamedIntercept)]);
        assert!(rb.get_intercept::<RealIntercept>().is_none());
        // no interceptor with this type name at all -> fall-through None
        assert!(rb.get_intercept::<FakeNamedIntercept>().is_none());
    }
}
