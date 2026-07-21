use async_trait::async_trait;
use futures::StreamExt;
use rbatis::intercept::{Intercept, StreamResult, StreamStatus};
use rbatis::{Error, Executor, RBatis};
use rbdc_sqlite::SqliteDriver;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::time::Duration;

async fn database() -> RBatis {
    let rb = RBatis::new();
    rb.link(SqliteDriver {}, "sqlite://:memory:").await.unwrap();
    rb.exec("CREATE TABLE stream_test (id INTEGER PRIMARY KEY)", vec![])
        .await
        .unwrap();
    for id in 1..=8 {
        rb.exec(
            "INSERT INTO stream_test (id) VALUES (?)",
            vec![id.into()],
        )
        .await
        .unwrap();
    }
    rb
}

#[tokio::test]
async fn streams_native_rows_in_order() {
    let rb = database().await;
    let mut stream = rb
        .query_stream("SELECT id FROM stream_test ORDER BY id", vec![], 2)
        .await
        .unwrap();
    let mut rows = Vec::new();
    while let Some(row) = stream.next().await {
        rows.push(row.unwrap());
    }
    assert_eq!(rows.len(), 8);
    assert_eq!(rows[0]["id"].as_i64(), Some(1));
    assert_eq!(rows[7]["id"].as_i64(), Some(8));
}

#[derive(Debug, Default)]
struct LifecycleProbe {
    status: AtomicU8,
    rows: AtomicUsize,
}

#[async_trait]
impl Intercept for LifecycleProbe {
    fn supports_query_stream(&self) -> bool {
        true
    }

    async fn after_stream(
        &self,
        _task_id: i64,
        _rb: &dyn Executor,
        _sql: &str,
        _args: &[rbs::Value],
        result: &StreamResult,
    ) -> Result<(), Error> {
        self.rows.store(result.rows, Ordering::SeqCst);
        self.status.store(
            match result.status {
                StreamStatus::Completed => 1,
                StreamStatus::Cancelled => 2,
                StreamStatus::Failed => 3,
            },
            Ordering::SeqCst,
        );
        Ok(())
    }
}

#[tokio::test]
async fn dropping_receiver_cancels_producer_and_releases_connection() {
    let mut rb = database().await;
    let probe = Arc::new(LifecycleProbe::default());
    rb.set_intercepts(vec![probe.clone()]);

    let mut stream = rb
        .query_stream("SELECT id FROM stream_test ORDER BY id", vec![], 1)
        .await
        .unwrap();
    assert!(stream.next().await.unwrap().is_ok());
    drop(stream);

    tokio::time::timeout(Duration::from_secs(2), async {
        while probe.status.load(Ordering::SeqCst) == 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert_eq!(probe.status.load(Ordering::SeqCst), 2);
    assert!(probe.rows.load(Ordering::SeqCst) < 8);
    assert!(rb.query("SELECT 1", vec![]).await.is_ok());
}

#[derive(Debug)]
struct MaterializingInterceptor;

#[async_trait]
impl Intercept for MaterializingInterceptor {}

#[tokio::test]
async fn rejects_interceptor_that_does_not_opt_in_to_streaming() {
    let mut rb = database().await;
    rb.set_intercepts(vec![Arc::new(MaterializingInterceptor)]);
    let error = rb
        .query_stream("SELECT id FROM stream_test", vec![], 1)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("MaterializingInterceptor"));
}
