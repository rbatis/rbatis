//! Second round of targeted coverage tests for the few remaining reachable gaps:
//! - `src/plugin/intercept/intercept_log.rs`: `RbsValueDisplay` formatting
//! - `rbatis-codegen/src/ops_eq.rs` / `ops_cmp.rs`: `&&Value` / reference impls
//!   invoked through fully qualified UFCS
//! - `src/plugin/id_generator/snowflake.rs`: realtime same-millisecond path
//! - `src/plugin/table_sync/mod.rs`: non-"already" CREATE error
//! - `src/plugin/intercept/mod.rs`: default `before`/`after` bodies
//! - `src/executor.rs`: `RBatisConnExecutor::{commit,rollback}` and the
//!   before-interceptor early return of `RBatis::exec_decode`

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use log::LevelFilter;
    use rbatis::executor::Executor;
    use rbatis::intercept::{Intercept, ResultType};
    use rbatis::plugin::intercept_log::LogInterceptor;
    use rbatis::snowflake::Snowflake;
    use rbatis::table_sync::sync;
    use rbatis::{Action, Error, RBatis};
    use rbatis_codegen::ops::{PartialEq as OpEq, PartialOrd as OpOrd};
    use rbdc::db::ExecResult;
    use rbdc_sqlite::SqliteDriver;
    use rbs::{value, Value};
    use std::cmp::Ordering;
    use std::sync::{Arc, Once};

    fn exec_ok(rows: u64) -> Result<ExecResult, Error> {
        Ok(ExecResult {
            rows_affected: rows,
            last_insert_id: Value::Null,
        })
    }

    // ------------------------------------------------------------------
    // LogInterceptor: RbsValueDisplay::new + Display::fmt
    // ------------------------------------------------------------------

    /// A real logger is required for the `log!` macro arguments to be
    /// formatted: the default no-op logger never calls `Display::fmt`.
    /// `set_max_level(Trace)` opens the macro's level gate; the logger below
    /// forces the formatting so `RbsValueDisplay`'s `Display` impl runs.
    struct FormattingLogger;

    static FORMATTING_LOGGER: FormattingLogger = FormattingLogger;

    impl log::Log for FormattingLogger {
        fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
            true
        }

        fn log(&self, record: &log::Record<'_>) {
            // Drive `Display` for every logged argument (RbsValueDisplay).
            let _ = format!("{}", record.args());
        }

        fn flush(&self) {}
    }

    static LOGGER_INIT: Once = Once::new();

    fn init_test_logger() {
        LOGGER_INIT.call_once(|| {
            // `set_boxed_logger` needs log's `std` feature, which is not
            // enabled in this workspace; `set_logger` is always available.
            let _ = log::set_logger(&FORMATTING_LOGGER);
            log::set_max_level(LevelFilter::Trace);
        });
    }

    #[tokio::test]
    async fn test_log_interceptor_display_empty_and_multiple_args() {
        init_test_logger();
        let rb = RBatis::new();
        let interceptor = LogInterceptor::new(LevelFilter::Trace);
        let mut sql = "select * from t where id = ?".to_string();

        // Empty args: '[' and ']' are written, the loop body is skipped.
        let mut empty: Vec<Value> = Vec::new();
        let mut exec_result = exec_ok(0);
        assert_eq!(
            interceptor
                .before(
                    1,
                    &rb,
                    &mut sql,
                    &mut empty,
                    ResultType::Exec(&mut exec_result)
                )
                .await
                .unwrap(),
            Action::Next
        );

        // Two args: the loop runs once and the comma branch runs once.
        let mut two = vec![Value::I64(1), Value::String("x".to_string())];
        let mut exec_result = exec_ok(1);
        assert_eq!(
            interceptor
                .before(
                    2,
                    &rb,
                    &mut sql,
                    &mut two,
                    ResultType::Exec(&mut exec_result)
                )
                .await
                .unwrap(),
            Action::Next
        );

        // Three args: comma after the first and the second argument.
        let mut three = vec![
            Value::I64(1),
            Value::String("x".to_string()),
            Value::Null,
        ];
        let mut query_result: Result<Value, Error> = Ok(Value::Array(vec![Value::I64(9)]));
        assert_eq!(
            interceptor
                .before(
                    3,
                    &rb,
                    &mut sql,
                    &mut three,
                    ResultType::Query(&mut query_result)
                )
                .await
                .unwrap(),
            Action::Next
        );

        // after() paths stay exercised with a real logger installed.
        let mut query_result: Result<Value, Error> = Ok(Value::Array(vec![Value::I64(9)]));
        assert_eq!(
            interceptor
                .after(
                    4,
                    &rb,
                    &mut sql,
                    &mut three,
                    ResultType::Query(&mut query_result)
                )
                .await
                .unwrap(),
            Action::Next
        );
    }

    // ------------------------------------------------------------------
    // ops_eq.rs: exact impls through fully qualified UFCS
    // ------------------------------------------------------------------

    #[test]
    fn test_eq_amp_value_receiver_ufcs() {
        let a = Value::I64(5);
        let b = Value::I64(5);
        let aa = &&a;

        // PartialEq<Value> for &&Value (ops_eq.rs:60-61)
        assert!(<&&Value as OpEq<Value>>::op_eq(&aa, &b));
        // PartialEq<&Value> for &&Value (ops_eq.rs:66-67)
        assert!(<&&Value as OpEq<&Value>>::op_eq(&aa, &&b));
        // PartialEq<&&Value> for &&Value (ops_eq.rs:72-73)
        assert!(<&&Value as OpEq<&&Value>>::op_eq(&aa, &&&b));

        // negative checks so the comparisons are real behaviour checks
        let c = Value::I64(6);
        assert!(!<&&Value as OpEq<Value>>::op_eq(&aa, &c));
        assert!(!<&&Value as OpEq<&Value>>::op_eq(&aa, &&c));
        assert!(!<&&Value as OpEq<&&Value>>::op_eq(&aa, &&&c));
    }

    #[test]
    fn test_eq_reference_string_ufcs() {
        let v = Value::String("x".to_string());
        let vr: &Value = &v;
        let s = String::from("x");
        let sr: &String = &s;

        // PartialEq<String> for &Value (ops_eq.rs:120-121)
        assert!(<&Value as OpEq<String>>::op_eq(&vr, &s));
        // PartialEq<&String> for &Value (ops_eq.rs:126-127)
        assert!(<&Value as OpEq<&String>>::op_eq(&vr, &&s));
        // PartialEq<Value> for &String (ops_eq.rs:144-145)
        assert!(<&String as OpEq<Value>>::op_eq(&sr, &v));

        let other = String::from("y");
        assert!(!<&Value as OpEq<String>>::op_eq(&vr, &other));
        assert!(!<&Value as OpEq<&String>>::op_eq(&vr, &&other));
        assert!(!<&String as OpEq<Value>>::op_eq(
            &&other,
            &Value::String("x".to_string())
        ));
    }

    // ------------------------------------------------------------------
    // ops_cmp.rs: exact impls through fully qualified UFCS
    // ------------------------------------------------------------------

    #[test]
    fn test_cmp_amp_value_receiver_ufcs() {
        let a = Value::I64(5);
        let b = Value::I64(5);
        let aa = &&a;

        // PartialOrd<Value> for &&Value (ops_cmp.rs:67-68)
        assert_eq!(
            <&&Value as OpOrd<Value>>::op_partial_cmp(&aa, &b),
            Some(Ordering::Equal)
        );
        // PartialOrd<&Value> for &&Value (ops_cmp.rs:73-74)
        assert_eq!(
            <&&Value as OpOrd<&Value>>::op_partial_cmp(&aa, &&b),
            Some(Ordering::Equal)
        );
        // PartialOrd<&&Value> for &&Value (ops_cmp.rs:79-80)
        assert_eq!(
            <&&Value as OpOrd<&&Value>>::op_partial_cmp(&aa, &&&b),
            Some(Ordering::Equal)
        );

        let c = Value::I64(7);
        assert_eq!(
            <&&Value as OpOrd<Value>>::op_partial_cmp(&aa, &c),
            Some(Ordering::Less)
        );
        assert_eq!(
            <&&Value as OpOrd<&&Value>>::op_partial_cmp(&aa, &&&c),
            Some(Ordering::Less)
        );
    }

    #[test]
    fn test_cmp_amp_value_str_ufcs() {
        let v = Value::String("x".to_string());
        let vv = &&v;

        // PartialOrd<&str> for &&Value (ops_cmp.rs:260-261)
        assert_eq!(
            <&&Value as OpOrd<&str>>::op_partial_cmp(&vv, &"x"),
            Some(Ordering::Equal)
        );
        // PartialOrd<&&str> for &&Value (ops_cmp.rs:266-267)
        assert_eq!(
            <&&Value as OpOrd<&&str>>::op_partial_cmp(&vv, &&"x"),
            Some(Ordering::Equal)
        );
        assert_eq!(
            <&&Value as OpOrd<&str>>::op_partial_cmp(&vv, &"y"),
            Some(Ordering::Less)
        );
    }

    // ------------------------------------------------------------------
    // Snowflake realtime same-millisecond path
    // ------------------------------------------------------------------

    #[test]
    fn test_snowflake_realtime_same_millisecond_biding() {
        // Construct with `last_timestamp` = the current millisecond, then call
        // `generate_id` immediately: when the clock has not ticked in between,
        // `now_millis == last_time` and `biding_time_conditions` runs
        // (snowflake.rs:139-141, 166-174). Bounded and timing-agnostic.
        for _ in 0..100 {
            let now = Snowflake::get_time_millis(std::time::UNIX_EPOCH);
            let sf = Snowflake::with_last_timestamp(1, 1, 1, now);
            assert_ne!(sf.generate_id(), 0);
        }
    }

    // ------------------------------------------------------------------
    // table_sync: CREATE TABLE error that is not an "already exists" error
    // ------------------------------------------------------------------

    #[tokio::test]
    async fn test_sync_create_table_non_already_error() {
        let rb = RBatis::default();
        rb.init(SqliteDriver {}, "sqlite://:memory:").unwrap();
        let conn = rb.acquire().await.unwrap();

        // An invalid table name makes CREATE TABLE fail with a syntax error,
        // so the `contains("already")` branch is false and the error is
        // returned directly (table_sync/mod.rs:147, 176).
        let err = sync(&conn, &rb, value! {"id": "INT"}, "sync bad name")
            .await
            .unwrap_err();
        let msg = err.to_string();
        assert!(!msg.is_empty());
        assert!(!msg.to_lowercase().contains("already"), "msg={}", msg);
    }

    // ------------------------------------------------------------------
    // Intercept trait default before/after bodies via dynamic dispatch
    // ------------------------------------------------------------------

    #[derive(Debug)]
    struct DefaultMethodsInterceptor;

    impl Intercept for DefaultMethodsInterceptor {}

    #[tokio::test]
    async fn test_intercept_default_methods_dyn_dispatch() {
        init_test_logger();
        let rb = RBatis::new();
        let interceptor: Box<dyn Intercept> = Box::new(DefaultMethodsInterceptor);
        let mut sql = "select 1".to_string();
        let mut args = vec![Value::I64(1)];
        let mut exec_result = exec_ok(1);

        assert_eq!(
            interceptor
                .before(
                    0,
                    &rb,
                    &mut sql,
                    &mut args,
                    ResultType::Exec(&mut exec_result)
                )
                .await
                .unwrap(),
            Action::Next
        );
        assert_eq!(
            interceptor
                .after(
                    0,
                    &rb,
                    &mut sql,
                    &mut args,
                    ResultType::Exec(&mut exec_result)
                )
                .await
                .unwrap(),
            Action::Next
        );
        assert!(interceptor.name().contains("DefaultMethodsInterceptor"));
    }

    // ------------------------------------------------------------------
    // executor.rs: RBatisConnExecutor::{commit, rollback}
    // ------------------------------------------------------------------

    #[tokio::test]
    async fn test_conn_executor_commit_and_rollback() {
        let rb = RBatis::default();
        rb.init(SqliteDriver {}, "sqlite://:memory:").unwrap();
        let conn = rb.acquire().await.unwrap();

        conn.exec("begin", vec![]).await.unwrap();
        conn.rollback().await.unwrap();

        conn.exec("begin", vec![]).await.unwrap();
        conn.commit().await.unwrap();
    }

    // ------------------------------------------------------------------
    // executor.rs: RBatis::exec_decode early return from a before interceptor
    // ------------------------------------------------------------------

    #[derive(Debug)]
    struct BeforeQueryReturn;

    #[async_trait]
    impl Intercept for BeforeQueryReturn {
        async fn before(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            _sql: &mut String,
            _args: &mut Vec<Value>,
            result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
        ) -> Result<Action, Error> {
            if let ResultType::Query(r) = result {
                *r = Ok(Value::Array(vec![Value::I64(7)]));
            }
            Ok(Action::Return)
        }
    }

    #[tokio::test]
    async fn test_rbatis_exec_decode_before_return() {
        let mut rb = RBatis::new();
        rb.set_intercepts(vec![Arc::new(BeforeQueryReturn)]);
        rb.init(SqliteDriver {}, "sqlite://:memory:").unwrap();

        // apply_before reports Return, so exec_decode returns the interceptor's
        // value through the early-return path (executor.rs:748).
        let v: Vec<i64> = rb.exec_decode("select 1", vec![]).await.unwrap();
        assert_eq!(v, vec![7]);
    }
}
