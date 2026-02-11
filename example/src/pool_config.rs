use rbatis::RBatis;
use rbdc_sqlite::{SqliteDriver};
use std::time::Duration;

/// Example: Configure the default connection pool
///
/// This example demonstrates how to configure the default connection pool parameters:
/// - max_open_conns: Maximum number of open connections to the database
/// - max_idle_conns: Maximum number of idle connections in the pool
/// - conn_max_lifetime: Maximum lifetime of a connection
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    )?;
    let rb = RBatis::new();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )?;
    // Configure connection pool parameters
    let pool = rb.get_pool()?;
    // Set the maximum number of open connections
    pool.set_max_open_conns(100).await;
    // Set the maximum number of idle connections
    pool.set_max_idle_conns(10).await;
    // Set the maximum lifetime of a connection
    pool.set_conn_max_lifetime(Some(Duration::from_secs(3600)))
        .await;
    // Print the current pool state
    println!(">>>>> state={}", pool.state().await);
    log::logger().flush();
    Ok(())
}
