use log::LevelFilter;
use rbatis::RBatis;
use rbatis::rbdc::DateTime;
use rbatis::table_sync::SqliteTableMapper;


/// this just only to create database table show example, you don't need this code
fn main() {
   tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(async {
      _ = fast_log::init(fast_log::Config::new().console().level(LevelFilter::Debug));
      let rb = RBatis::new();
      // ------------choose database driver------------
      // rb.init(rbdc_mysql::driver::MysqlDriver {}, "mysql://root:123456@localhost:3306/test").unwrap();
      // rb.init(rbdc_pg::driver::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres").unwrap();
      // rb.init(rbdc_mssql::driver::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;").unwrap();
      let r = rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://../target/sqlite.db");
      if r.is_err(){
         return;
      }
      let conn = rb.acquire().await;
      if conn.is_err(){
         return;
      }
      #[derive(serde::Serialize, serde::Deserialize, Clone)]
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
      
      _ = RBatis::sync(
         &conn.unwrap(),
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

      log::logger().flush();
   });
}