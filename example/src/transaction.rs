use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{Error, RBatis, RBatisTxExecutorGuard};
use rbs::value;

/// table
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Activity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

rbatis::crud!(Activity {});

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    _ = fast_log::init(fast_log::Config::new().console().level(LevelFilter::Debug));
    defer!(|| log::logger().flush());
    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::MysqlDriver {}, "mysql://root:123456@localhost:3306/test")?;
    // rb.init(rbdc_pg::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres")?;
    // rb.init(rbdc_mssql::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;")?;
    rb.init(rbdc_sqlite::SqliteDriver {}, "sqlite://target/sqlite.db")?;

    //clear data
    let _ = Activity::delete_by_map(&rb, value! {"id":["3"]}).await;

    // will forget commit
    let tx = rb.acquire_begin().await?.auto_commit();
    transaction(tx, true).await?;
    // forget commit ,tx will rollback here.
    // will do commit
    let conn = rb.acquire().await?;
    let tx2 = conn.begin().await?.auto_commit();
    transaction(tx2, false).await?;
    // tx is commit here
    Ok(())
}

async fn transaction(tx: RBatisTxExecutorGuard, forget_commit: bool) -> Result<(), Error> {
    log::info!("transaction [{}] start", tx.tx_id());
    let _ = Activity::insert(
        &tx,
        &Activity {
            id: Some("3".into()),
            name: Some("3".into()),
            pc_link: Some("3".into()),
            h5_link: Some("3".into()),
            pc_banner_img: None,
            h5_banner_img: None,
            sort: None,
            status: Some(3),
            remark: Some("3".into()),
            create_time: Some(DateTime::now()),
            version: Some(1),
            delete_flag: Some(1),
        },
    )
    .await;
    //if not commit or rollback,tx.done = false,
    if !forget_commit {
        tx.commit().await?;
    }
    Ok(())
}
