use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::executor::RBatisTxExecutor;
use rbatis::rbdc::datetime::DateTime;
use rbatis::table_sync::SqliteTableMapper;
use rbatis::{Error, RBatis};

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
rbatis::impl_insert!(Activity {});
rbatis::impl_delete!(Activity {});
#[tokio::main]
pub async fn main() {
    _ = fast_log::init(fast_log::Config::new().console().level(LevelFilter::Debug));
    defer!(|| log::logger().flush());
    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
    .unwrap();
    // table sync done
    fast_log::LOGGER.set_level(LevelFilter::Off);
    _ = RBatis::sync(
        &rb.acquire().await.unwrap(),
        &SqliteTableMapper {},
        &Activity {
            id: Some(String::new()),
            name: Some(String::new()),
            pc_link: Some(String::new()),
            h5_link: Some(String::new()),
            pc_banner_img: Some(String::new()),
            h5_banner_img: Some(String::new()),
            sort: Some(String::new()),
            status: Some(0),
            remark: Some(String::new()),
            create_time: Some(DateTime::now()),
            version: Some(0),
            delete_flag: Some(0),
        },
        "activity",
    )
    .await;
    fast_log::LOGGER.set_level(LevelFilter::Debug);
    //clear data
    let _ = Activity::delete_in_column(&rb.clone(), "id", &["3"]).await;
    //use rb.acquire_begin() create tx
    tx_insert(
        {
            // acquire_begin() = {let conn=rb.acquire(); conn.begin()}
            rb.acquire_begin().await.unwrap()
        },
        "3",
        true,
    )
    .await
    .unwrap();
    //use conn.begin() create tx
    tx_insert(
        {
            // acquire_begin() = {let conn=rb.acquire(); conn.begin()}
            rb.acquire_begin().await.unwrap()
        },
        "3",
        false,
    )
    .await
    .unwrap();
}

async fn tx_insert(tx: RBatisTxExecutor, id: &str, forget_commit: bool) -> Result<(), Error> {
    let mut tx = tx.defer_async(|mut tx| async move {
        if tx.done {
            println!("---------------tx complete---------------");
        } else {
            let r = tx.rollback().await;
            if let Err(e) = r {
                log::error!("rollback fail {}", e);
            }
            println!("---------------tx not complete, rollback---------------");
        }
    });
    let _ = Activity::insert(
        &mut tx,
        &Activity {
            id: Some(id.into()),
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
