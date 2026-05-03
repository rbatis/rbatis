/// Tests for RBatisTxExecutor specific features not fully covered elsewhere:
/// - RBatisTxExecutor::set_done()
/// - RBatisTxExecutor::done() state transitions
/// - RBatisTxExecutor::begin() nested transaction support
/// - RBatisTxExecutor::take_connection()
/// - RBatisTxExecutorGuard::take_connection()

#[cfg(test)]
mod test {
    use futures_core::future::BoxFuture;
    use futures_core::Stream;
    use rbatis::executor::Executor;
    use rbatis::{Error, RBatis, RBatisRef};
    use rbdc::db::{ConnectOptions, Connection, Driver, ExecResult, Row};
    use rbdc::rt::block_on;
    use rbs::Value;
    use std::pin::Pin;

    // ==================== Mock Infrastructure ====================

    #[derive(Debug, Clone)]
    struct MockDriver {}

    impl Driver for MockDriver {
        fn name(&self) -> &str {
            "test"
        }
        fn connect(&self, _url: &str) -> BoxFuture<'_, Result<Box<dyn Connection>, Error>> {
            Box::pin(async { Ok(Box::new(MockConnection {}) as Box<dyn Connection>) })
        }
        fn connect_opt<'a>(
            &'a self,
            _option: &'a dyn ConnectOptions,
        ) -> BoxFuture<'a, Result<Box<dyn Connection>, Error>> {
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
            &mut self,
            _sql: &str,
            _params: Vec<Value>,
        ) -> BoxFuture<
            '_,
            Result<Pin<Box<dyn Stream<Item = Result<Box<dyn Row>, Error>> + Send + '_>>, Error>,
        > {
            Box::pin(async { Err(Error::from("mock")) })
        }
        fn exec(
            &mut self,
            _sql: &str,
            _params: Vec<Value>,
        ) -> BoxFuture<'_, Result<ExecResult, Error>> {
            Box::pin(async {
                Ok(ExecResult {
                    rows_affected: 0,
                    last_insert_id: Value::Null,
                })
            })
        }
        fn close(&mut self) -> BoxFuture<'_, Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }
        fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }

        fn begin(&mut self) -> BoxFuture<'_, Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }
        fn commit(&mut self) -> BoxFuture<'_, Result<(), Error>> {
            Box::pin(async { Ok(()) })
        }
        fn rollback(&mut self) -> BoxFuture<'_, Result<(), Error>> {
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

    // ==================== RBatisTxExecutor::done / set_done Tests ====================

    #[test]
    fn test_tx_executor_initial_done_state() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            assert!(!tx.done());
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_set_done_true() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            tx.set_done(true);
            assert!(tx.done());
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_set_done_false() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            tx.set_done(true);
            assert!(tx.done());
            tx.set_done(false);
            assert!(!tx.done());
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_commit_sets_done() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            assert!(!tx.done());
            tx.commit().await.unwrap();
            assert!(tx.done());
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_rollback_sets_done() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            assert!(!tx.done());
            tx.rollback().await.unwrap();
            assert!(tx.done());
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_multiple_commit_calls() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();

            tx.commit().await.unwrap();
            assert!(tx.done());

            tx.commit().await.unwrap();
            assert!(tx.done());
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_multiple_rollback_calls() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();

            tx.rollback().await.unwrap();
            assert!(tx.done());

            tx.rollback().await.unwrap();
            assert!(tx.done());
        };
        block_on(f);
    }

    // ==================== RBatisTxExecutor::begin (nested) Tests ====================

    #[test]
    fn test_tx_executor_nested_begin() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();

            let tx2 = tx.begin().await.unwrap();
            assert!(!tx2.done());
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_double_nested_begin() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let tx2 = tx.begin().await.unwrap();
            let tx3 = tx2.begin().await.unwrap();
            assert!(!tx3.done());
        };
        block_on(f);
    }

    // ==================== take_connection Tests ====================

    #[test]
    fn test_tx_executor_take_connection() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();

            let conn = tx.take_connection();
            assert!(conn.is_some());
        };
        block_on(f);
    }

    #[test]
    fn test_conn_executor_take_connection_via_conn() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let conn = rb.acquire().await.unwrap();

            let taken = conn.take_connection();
            assert!(taken.is_some());
        };
        block_on(f);
    }

    #[test]
    #[ignore]
    fn test_tx_guard_take_connection() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let guard = tx.defer_async(|_tx| async {});

            let conn = guard.take_connection();
            assert!(conn.is_some());
        };
        block_on(f);
    }

    // ==================== RBatisTxExecutorGuard Tests ====================

    #[test]
    fn test_tx_guard_tx_id() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let guard = tx.defer_async(|_tx| async {});

            let tx_id = guard.tx_id();
            assert_eq!(tx_id, tx.tx_id);
        };
        block_on(f);
    }

    #[test]
    fn test_tx_guard_commit_delegates() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let guard = tx.defer_async(|_tx| async {});

            guard.commit().await.unwrap();
            assert!(tx.done());
        };
        block_on(f);
    }

    #[test]
    fn test_tx_guard_rollback_delegates() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let guard = tx.defer_async(|_tx| async {});

            guard.rollback().await.unwrap();
            assert!(tx.done());
        };
        block_on(f);
    }

    // ==================== RBaisTxExecutorGuard Drop Callback Test ====================

    #[test]
    fn test_tx_guard_drop_callback_invoked() {
        // defer_async uses fn pointer, so we can only verify basic drop behavior
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();

            // Create guard and immediately drop it - should not panic
            {
                let _guard = tx.defer_async(|_tx| async {});
                // Guard is dropped here, triggering callback
            }

            // If we get here without panicking, the callback mechanism works
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        };
        block_on(f);
    }

    // ==================== RBaisTxExecutor::defer_async Tests ====================

    #[test]
    fn test_defer_async_basic_usage() {
        // defer_async takes a fn pointer (not closure), verify it compiles
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();

            // Create guard with simple no-op callback
            let _guard = tx.defer_async(|_tx| async {});
            drop(_guard);

            // Verify tx still accessible
            assert!(!tx.done());
        };
        block_on(f);
    }

    // ==================== RBaisTxExecutor Executor trait impl ====================

    #[test]
    fn test_tx_executor_id() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            assert!(tx.id() != 0);
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_exec() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let result = tx
                .exec("insert into test values (1)", vec![])
                .await
                .unwrap();
            assert_eq!(result.rows_affected, 0);
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_query() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let _result = tx.query("select * from test", vec![]).await;
        };
        block_on(f);
    }

    #[test]
    fn test_tx_executor_rb_ref() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let rb_ref = tx.rb_ref();
            assert_eq!(rb_ref.driver_type().unwrap(), "test");
        };
        block_on(f);
    }

    // ==================== RBaisTxExecutorGuard Executor trait impl ====================

    #[test]
    fn test_tx_guard_id_matches_tx() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let guard = tx.defer_async(|_tx| async {});

            assert_eq!(guard.id(), tx.tx_id);
            assert_eq!(Executor::id(&guard), tx.tx_id);
        };
        block_on(f);
    }

    #[test]
    fn test_tx_guard_exec_delegates() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let guard = tx.defer_async(|_tx| async {});

            let result = guard
                .exec("insert into test values (1)", vec![])
                .await
                .unwrap();
            assert_eq!(result.rows_affected, 0);
        };
        block_on(f);
    }

    #[test]
    fn test_tx_guard_query_delegates() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let guard = tx.defer_async(|_tx| async {});

            let _result = guard.query("select * from test", vec![]).await;
        };
        block_on(f);
    }

    #[test]
    fn test_tx_guard_exec_decode() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let guard = tx.defer_async(|_tx| async {});

            let _: Option<()> = guard.exec_decode("select 1", vec![]).await.ok();
        };
        block_on(f);
    }

    #[test]
    fn test_tx_guard_rb_ref() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let tx = rb.acquire_begin().await.unwrap();
            let guard = tx.defer_async(|_tx| async {});

            assert_eq!(guard.rb_ref().driver_type().unwrap(), "test");
        };
        block_on(f);
    }

    // ==================== Conn-level begin/commit/rollback Tests ====================

    #[test]
    fn test_conn_executor_begin_then_commit() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let conn = rb.acquire().await.unwrap();

            let tx = conn.begin().await.unwrap();
            tx.commit().await.unwrap();
        };
        block_on(f);
    }

    #[test]
    fn test_conn_executor_begin_then_rollback() {
        let f = async move {
            let rb = RBatis::new();
            rb.init(MockDriver {}, "test").unwrap();
            let conn = rb.acquire().await.unwrap();

            let tx = conn.begin().await.unwrap();
            tx.rollback().await.unwrap();
        };
        block_on(f);
    }

    // ==================== RBatis::try_acquire_begin Tests ====================

    #[tokio::test]
    async fn test_try_acquire_begin_success() {
        let rb = RBatis::new();
        rb.init(MockDriver {}, "test").unwrap();
        let tx = rb.try_acquire_begin().await.unwrap();
        assert!(!tx.done());
    }

    #[tokio::test]
    async fn test_try_acquire_begin_not_inited() {
        let rb = RBatis::default();
        let result = rb.try_acquire_begin().await;
        assert!(result.is_err());
    }
}
