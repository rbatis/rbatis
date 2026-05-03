use log::LevelFilter;
use rbatis::crud;
use rbatis::dark_std::defer;
use rbatis::table_sync::SqliteTableMapper;
use rbatis::{table_sync, Error, RBatis};
use rbs::value;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: Option<u64>,
    pub name: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Option<u64>,
    //json support json object/array
    pub account1: Account,
    //json support json object/array
    pub account2: Vec<Account>,
}

crud!(User {});

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(log::LevelFilter::Debug),
    );
    defer!(|| {
        log::logger().flush();
    });

    let rb = RBatis::new();
    // ------------choose database driver------------
    // rb.init(rbdc_mysql::MysqlDriver {}, "mysql://root:123456@localhost:3306/test")?;
    // rb.init(rbdc_pg::PgDriver {}, "postgres://postgres:123456@localhost:5432/postgres")?;
    // rb.init(rbdc_mssql::MssqlDriver {}, "mssql://jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=master;")?;
    rb.init(rbdc_sqlite::SqliteDriver {}, "sqlite://target/sqlite.db")?;
    create_table(&rb).await?;
    let user = User {
        id: Some(1),
        account1: Account {
            id: Some(2),
            name: Some("xxx".to_string()),
        },
        account2: vec![Account {
            id: Some(2),
            name: Some("xxx".to_string()),
        }],
    };

    let v = User::insert(&rb.clone(), &user).await;
    println!("insert:{:?}", v);

    let users = User::select_by_map(&rb.clone(), value! {"id":1}).await;
    println!("select:{}", value!(users));
    Ok(())
}

async fn create_table(rb: &RBatis) -> Result<(), Error> {
    fast_log::logger().set_level(LevelFilter::Off);
    defer!(|| {
        fast_log::logger().set_level(LevelFilter::Info);
    });
    let table = value! {
        "id":"INTEGER PRIMARY KEY AUTOINCREMENT",
        "account1":"JSON",
        "account2":"JSON",
    };
    let conn = rb.acquire().await?;
    _ = table_sync::sync(&conn, &SqliteTableMapper {}, value!(&table), "user").await;
    Ok(())
}
