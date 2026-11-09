use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::{Error, RBatis};
use std::time::Duration;

/// Example: Configure the default connection pool
///
/// This example demonstrates how to configure the default connection pool parameters:
/// - max_open_conns: Maximum number of open connections to the database
/// - max_idle_conns: Maximum number of idle connections in the pool
/// - conn_max_lifetime: Maximum lifetime of a connection
#[tokio::main]
pub async fn main() -> Result<(), Error> {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(LevelFilter::Debug),
    );
    defer!(|| log::logger().flush());
    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::MysqlDriver {}, "mysql://root:123456@localhost:3306/test")?;
    // rb.init(rbdc_pg::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres")?;
    // rb.init(rbdc_mssql::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;")?;
    rb.init(rbdc_sqlite::SqliteDriver {}, "sqlite://target/sqlite.db")?;
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
    Ok(())
}
