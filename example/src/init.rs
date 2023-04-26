use log::LevelFilter;
use rbatis::rbatis::Rbatis;
use rbatis::dark_std::defer;

/// make an Rbatis
pub async fn init_db() -> Rbatis {
    let rb = Rbatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
    // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://SA:TestPass!123456@localhost:1433/test").unwrap();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
    .unwrap();

    // ------------create tables way 2------------
    let sql = std::fs::read_to_string("example/table_sqlite.sql").unwrap();
    let raw = fast_log::LOGGER.get_level().clone();
    fast_log::LOGGER.set_level(LevelFilter::Off);
    defer!(||{
         fast_log::LOGGER.set_level(raw);
    });
    let _ = rb.exec(&sql, vec![]).await;
    // ------------create tables way 2 end------------
    return rb;
}
