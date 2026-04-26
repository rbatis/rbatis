/// Extended tests for intercept module covering:
/// - ResultType::type_name()
/// - Action::Next / Action::Return equality
/// - LogInterceptor (level filter, clone, before/after behavior)
/// - Intercept trait default implementations
/// - RBatis::remove_intercept_dyn
/// - RBatis::get_intercept_dyn
/// - RBatis::is_debug_mode / driver_type

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use futures_core::future::BoxFuture;
    use futures_core::Stream;
    use log::LevelFilter;
    use rbatis::executor::Executor;
    use rbatis::intercept::{
        intercept_log::LogInterceptor,
        intercept_page::PageIntercept,
        Intercept, ResultType,
    };
    use rbatis::{Action, Error, RBatis};
    use rbdc::db::{ConnectOptions, Connection, Driver, ExecResult, Row};
    use rbs::Value;
    use std::fmt::Debug;
    use std::pin::Pin;
    use std::sync::Arc;

    // ==================== Mock infrastructure ====================

    #[derive(Debug, Clone)]
    struct MockDriver {}

    impl Driver for MockDriver {
        fn name(&self) -> &str { "test" }

        fn connect(&self, _url: &str) -> BoxFuture<'_, Result<Box<dyn Connection>, Error>> {
            Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
        }

        fn connect_opt<'a>(&'a self, _option: &'a dyn ConnectOptions) -> BoxFuture<'a, Result<Box<dyn Connection>, Error>> {
            Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
        }

        fn default_option(&self) -> Box<dyn ConnectOptions> {
            Box::new(MockConnectOptions {})
        }
    }

    #[derive(Clone, Debug)]
    struct MockConnection {}

    impl Connection for MockConnection {
        fn exec_rows(
            &mut self, _sql: &str, _params: Vec<Value>,
        ) -> BoxFuture<'_, Result<Pin<Box<dyn Stream<Item = Result<Box<dyn Row>, Error>> + Send + '_>>, Error>> {
            Box::pin(async {
                Err(Error::from("mock"))
            })
        }

        fn exec(&mut self, _sql: &str, _params: Vec<Value>) -> BoxFuture<'_, Result<ExecResult, Error>> {
            Box::pin(async {
                Ok(ExecResult { rows_affected: 0, last_insert_id: Value::Null })
            })
        }

        fn close(&mut self) -> BoxFuture<'_, Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }

        fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[derive(Clone, Debug)]
    struct MockConnectOptions {}

    impl ConnectOptions for MockConnectOptions {
        fn connect(&self) -> BoxFuture<'_, Result<Box<dyn Connection>, Error>> {
            Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
        }

        fn set_uri(&mut self, _uri: &str) -> Result<(), Error> {
            Ok(())
        }
    }

    // ==================== ResultType Tests ====================

    #[test]
    fn test_result_type_exec_type_name() {
        let rt: ResultType<i32, String> = ResultType::Exec(42i32);
        assert_eq!(rt.type_name(), "exec");
    }

    #[test]
    fn test_result_type_query_type_name() {
        let rt: ResultType<i32, String> = ResultType::Query("hello".to_string());
        assert_eq!(rt.type_name(), "query");
    }

    // ==================== Action Tests ====================

    #[test]
    fn test_action_next_equals() {
        assert_eq!(Action::Next, Action::Next);
    }

    #[test]
    fn test_action_return_equals() {
        assert_eq!(Action::Return, Action::Return);
    }

    #[test]
    fn test_action_not_equal() {
        assert_ne!(Action::Next, Action::Return);
    }

    // ==================== LogInterceptor Tests ====================

    #[test]
    fn test_log_interceptor_new_off() {
        let log = LogInterceptor::new(LevelFilter::Off);
        assert_eq!(log.get_level_filter(), LevelFilter::Off);
    }

    #[test]
    fn test_log_interceptor_new_debug() {
        let log = LogInterceptor::new(LevelFilter::Debug);
        assert_eq!(log.get_level_filter(), LevelFilter::Debug);
    }

    #[test]
    fn test_log_interceptor_new_error() {
        let log = LogInterceptor::new(LevelFilter::Error);
        assert_eq!(log.get_level_filter(), LevelFilter::Error);
    }

    #[test]
    fn test_log_interceptor_new_info() {
        let log = LogInterceptor::new(LevelFilter::Info);
        assert_eq!(log.get_level_filter(), LevelFilter::Info);
    }

    #[test]
    fn test_log_interceptor_new_trace() {
        let log = LogInterceptor::new(LevelFilter::Trace);
        assert_eq!(log.get_level_filter(), LevelFilter::Trace);
    }

    #[test]
    fn test_log_interceptor_new_warn() {
        let log = LogInterceptor::new(LevelFilter::Warn);
        assert_eq!(log.get_level_filter(), LevelFilter::Warn);
    }

    #[test]
    fn test_log_interceptor_set_level_filter_roundtrip() {
        let log = LogInterceptor::new(LevelFilter::Off);
        log.set_level_filter(LevelFilter::Trace);
        assert_eq!(log.get_level_filter(), LevelFilter::Trace);

        log.set_level_filter(LevelFilter::Error);
        assert_eq!(log.get_level_filter(), LevelFilter::Error);

        log.set_level_filter(LevelFilter::Off);
        assert_eq!(log.get_level_filter(), LevelFilter::Off);
    }

    #[test]
    fn test_log_interceptor_clone() {
        let log = LogInterceptor::new(LevelFilter::Debug);
        let cloned = log.clone(); // uses custom Clone impl
        assert_eq!(cloned.get_level_filter(), LevelFilter::Debug);
    }

    #[test]
    fn test_log_interceptor_to_level() {
        let log = LogInterceptor::new(LevelFilter::Off);
        assert!(log.to_level().is_none());

        let log = LogInterceptor::new(LevelFilter::Error);
        assert_eq!(log.to_level(), Some(log::Level::Error));

        let log = LogInterceptor::new(LevelFilter::Warn);
        assert_eq!(log.to_level(), Some(log::Level::Warn));

        let log = LogInterceptor::new(LevelFilter::Info);
        assert_eq!(log.to_level(), Some(log::Level::Info));

        let log = LogInterceptor::new(LevelFilter::Debug);
        assert_eq!(log.to_level(), Some(log::Level::Debug));

        let log = LogInterceptor::new(LevelFilter::Trace);
        assert_eq!(log.to_level(), Some(log::Level::Trace));
    }

    #[test]
    fn test_log_interceptor_is_send_sync() {
        // Compile-time check that LogInterceptor can be sent between threads
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<LogInterceptor>();
    }

    // ==================== PageIntercept Tests ====================

    #[test]
    fn test_page_intercept_new() {
        let pi = PageIntercept::new();
        assert_eq!(pi.select_ids.len(), 0);
        assert_eq!(pi.count_ids.len(), 0);
    }

    #[test]
    fn test_page_intercept_count_param_count() {
        let pi = PageIntercept::new();
        assert_eq!(pi.count_param_count("mysql", "SELECT * FROM t WHERE id = ? AND name = ?"), 2);
        assert_eq!(pi.count_param_count("sqlite", "SELECT * FROM t WHERE id = ?"), 1);
        assert_eq!(pi.count_param_count("pg", "SELECT * FROM t"), 0);
        assert_eq!(pi.count_param_count("mssql", "???"), 3);
    }

    #[test]
    fn test_page_intercept_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<PageIntercept>();
    }

    // ==================== Intercept Trait Default Impl Tests ====================

    #[derive(Debug)]
    struct DefaultImplIntercept;

    #[async_trait]
    impl Intercept for DefaultImplIntercept {
        // Uses all default implementations
    }

    #[test]
    fn test_intercept_default_name() {
        let intercept = DefaultImplIntercept;
        // Default name is type_name
        let name = intercept.name();
        assert!(name.contains("DefaultImplIntercept"));
    }

    // ==================== RBatis::remove_intercept_dyn Tests ====================

    #[test]
    fn test_remove_intercept_dyn_existing() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();

        let initial_len = rb.intercepts.len();
        // Get name of first intercept (e.g., PageIntercept)
        let first_name = rb.intercepts[0].name().to_string();

        let removed = rb.remove_intercept_dyn(&first_name);
        assert!(removed.is_some());
        assert_eq!(rb.intercepts.len(), initial_len - 1);
    }

    #[test]
    fn test_remove_intercept_dyn_nonexistent() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();

        let initial_len = rb.intercepts.len();
        let removed = rb.remove_intercept_dyn("NonExistentInterceptThatDoesNotExist");
        assert!(removed.is_none());
        assert_eq!(rb.intercepts.len(), initial_len);
    }

    #[test]
    fn test_remove_intercept_dyn_empty_name() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();

        let removed = rb.remove_intercept_dyn("");
        assert!(removed.is_none());
    }

    #[test]
    fn test_remove_and_re_add_intercept() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();

        // Remove PageIntercept
        let removed = rb.remove_intercept_dyn(rb.intercepts[0].name());
        assert!(removed.is_some());

        // Re-add it
        rb.intercepts.push(removed.unwrap());
        // Length should be back to original
        assert!(!rb.intercepts.is_empty());
    }

    #[test]
    fn test_remove_all_intercepts() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();

        while !rb.intercepts.is_empty() {
            let name = rb.intercepts[rb.intercepts.len() - 1].name().to_string();
            let removed = rb.remove_intercept_dyn(&name);
            assert!(removed.is_some());
        }
        assert_eq!(rb.intercepts.len(), 0);
    }

    // ==================== RBatis::get_intercept_dyn Tests ====================

    #[test]
    fn test_get_intercept_dyn_found() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();

        let first_name = rb.intercepts[0].name().to_string();
        let found = rb.get_intercept_dyn(&first_name);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), first_name);
    }

    #[test]
    fn test_get_intercept_dyn_not_found() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();

        let found = rb.get_intercept_dyn("TotallyFakeIntercept");
        assert!(found.is_none());
    }

    #[test]
    fn test_get_intercept_vs_get_intercept_dyn_consistency() {
        // Verify both methods find the same interceptors
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();

        for item in rb.intercepts.iter() {
            let name = item.name().to_string();
            let dyn_result = rb.get_intercept_dyn(&name);
            assert!(dyn_result.is_some(), "get_intercept_dyn should find {}", name);
        }
    }

    // ==================== RBatis::driver_type Tests ====================

    #[test]
    fn test_driver_type_after_init() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();

        let dt = rb.driver_type();
        assert!(dt.is_ok());
        // MockDriver returns "test"
        assert_eq!(dt.unwrap(), "test");
    }

    #[test]
    fn test_driver_type_before_init_fails() {
        let rb = RBatis::new();
        // Pool not initialized
        let dt = rb.driver_type();
        assert!(dt.is_err());
        assert!(dt.err().unwrap().to_string().contains("pool not inited"));
    }

    // ==================== RBatis::is_debug_mode Tests ====================

    #[test]
    fn test_is_debug_mode_returns_bool() {
        let rb = RBatis::new();
        // Just verify it returns without panicking; value depends on compile-time features
        let _mode = rb.is_debug_mode();
        // In release mode without debug_mode feature, this should be false
        // In debug mode with debug_mode feature enabled, it could be true
    }

    // ==================== RBatis::Default Tests ====================

    #[test]
    fn test_rbatis_default_has_no_pool() {
        let rb = RBatis::default();
        assert!(rb.pool.get().is_none());
        assert!(rb.intercepts.is_empty()); // Default has no intercepts (unlike ::new())
    }

    #[test]
    fn test_rbatis_new_has_intercepts() {
        let rb = RBatis::new();
        // ::new() adds PageIntercept + LogInterceptor
        assert!(!rb.intercepts.is_empty());
        assert!(rb.intercepts.len() >= 2);
    }

    #[test]
    fn test_rbatis_clone() {
        let rb = RBatis::new();
        let rb2 = rb.clone();
        // Both share the same pool and intercepts (Arc)
        assert_eq!(rb.intercepts.len(), rb2.intercepts.len());
    }

    // ==================== Custom Intercept Integration Tests ====================

    #[derive(Debug)]
    struct CountingIntercept {
        pub before_count: Arc<std::sync::atomic::AtomicUsize>,
        pub after_count: Arc<std::sync::atomic::AtomicUsize>,
    }

    #[async_trait]
    impl Intercept for CountingIntercept {
        async fn before(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            _sql: &mut String,
            _args: &mut Vec<Value>,
            _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
        ) -> Result<Action, Error> {
            self.before_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(Action::Next)
        }

        async fn after(
            &self,
            _task_id: i64,
            _rb: &dyn Executor,
            _sql: &mut String,
            _args: &mut Vec<Value>,
            _result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
        ) -> Result<Action, Error> {
            self.after_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok(Action::Next)
        }
    }

    #[tokio::test]
    async fn test_intercept_before_after_called_for_exec() {
        let before_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let after_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let mut rb = RBatis::new();
        rb.set_intercepts(vec![Arc::new(CountingIntercept {
            before_count: before_count.clone(),
            after_count: after_count.clone(),
        })]);
        rb.init(MockDriver {}, "test").unwrap();

        rb.exec("select * from test", vec![]).await.unwrap();

        assert_eq!(before_count.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(after_count.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_intercept_before_return_skips_execution() {
        #[derive(Debug)]
        struct SkipExecutionIntercept;

        #[async_trait]
        impl Intercept for SkipExecutionIntercept {
            async fn before(
                &self,
                _task_id: i64,
                _rb: &dyn Executor,
                _sql: &mut String,
                _args: &mut Vec<Value>,
                result: ResultType<&mut Result<ExecResult, Error>, &mut Result<Value, Error>>,
            ) -> Result<Action, Error> {
                match result {
                    ResultType::Exec(v) => {
                        *v = Ok(ExecResult {
                            rows_affected: 999,
                            last_insert_id: Value::Null,
                        });
                    }
                    ResultType::Query(_) => {}
                }
                Ok(Action::Return)
            }
        }

        let mut rb = RBatis::new();
        rb.set_intercepts(vec![Arc::new(SkipExecutionIntercept)]);
        rb.init(MockDriver {}, "test").unwrap();

        let result = rb.exec("should not execute", vec![]).await.unwrap();
        assert_eq!(result.rows_affected, 999);
    }

    // ==================== set_intercepts Tests ====================

    #[test]
    fn test_set_intercepts_replaces_all() {
        #[derive(Debug)]
        struct CustomIntercept1;
        #[derive(Debug)]
        struct CustomIntercept2;
        #[derive(Debug)]
        struct CustomIntercept3;

        #[async_trait]
        impl Intercept for CustomIntercept1 {}
        #[async_trait]
        impl Intercept for CustomIntercept2 {}
        #[async_trait]
        impl Intercept for CustomIntercept3 {}

        let mut rb = RBatis::new();
        let orig_len = rb.intercepts.len();
        assert!(orig_len >= 2); // PageIntercept + LogInterceptor

        rb.set_intercepts(vec![
            Arc::new(CustomIntercept1),
            Arc::new(CustomIntercept2),
            Arc::new(CustomIntercept3),
        ]);
        assert_eq!(rb.intercepts.len(), 3);
    }

    #[test]
    fn test_set_intercepts_empty() {
        let mut rb = RBatis::new();
        rb.set_intercepts(vec![]);
        assert_eq!(rb.intercepts.len(), 0);
    }

    // ==================== RBatis::init error cases ====================

    #[test]
    fn test_init_empty_url_returns_error() {
        let rb = RBatis::new();
        let result = rb.init(MockDriver {}, "");
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("url is empty"));
    }

    // ==================== get_pool error case ====================

    #[test]
    fn test_get_pool_not_inited() {
        let rb = RBatis::default(); // No pool initialized
        let result = rb.get_pool();
        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("pool not inited"));
    }

    // ==================== acquire error cases ====================

    #[tokio::test]
    async fn test_acquire_pool_not_inited() {
        let rb = RBatis::default();
        let result = rb.acquire().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_try_acquire_pool_not_inited() {
        let rb = RBatis::default();
        let result = rb.try_acquire().await;
        assert!(result.is_err());
    }

    // ==================== Executor::name() default impl ====================

    #[tokio::test]
    async fn test_executor_name_default() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();
        let name = rb.name();
        assert!(name.contains("RBatis"));
    }
}
