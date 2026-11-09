use log::LevelFilter;
use rbatis::dark_std::defer;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{Error, RBatis};
use rbs::value;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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

/// All methods read SQL from example.html (resolved relative to the example/ directory)
#[rbatis::html_sql("example.html")]
impl Activity {
    /// Maps to <select id="select_by_condition"> in HTML
    pub async fn select_by_condition(
        rb: &dyn rbatis::Executor,
        name: &str,
        dt: &DateTime,
    ) -> rbatis::Result<Vec<Activity>> {
        impled!()
    }

    /// Maps to <select id="select_page_data"> in HTML
    pub async fn select_page_data(
        rb: &dyn rbatis::Executor,
        page_req: &rbatis::PageRequest,
        name: &str,
        dt: &DateTime,
    ) -> rbatis::Result<rbatis::Page<Activity>> {
        impled!()
    }

    /// Maps to <update id="update_by_id"> in HTML
    pub async fn update_by_id(
        rb: &dyn rbatis::Executor,
        arg: &Activity,
    ) -> rbatis::Result<rbatis::rbdc::ExecResult> {
        impled!()
    }

    /// Paginated query - automatically detected by return type Page<Activity>
    /// Maps to <select id="select_by_page"> in HTML
    /// The macro automatically generates pagination logic using PageIntercept
    pub async fn select_by_page(
        rb: &dyn rbatis::Executor,
        page_req: &rbatis::PageRequest,
        name: &str,
        dt: Option<DateTime>,
    ) -> rbatis::Result<rbatis::Page<Activity>> {
        impled!()
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    _ = fast_log::init(
        fast_log::Config::new()
            .console()
            .level(LevelFilter::Debug),
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
    // Use impl block Mapper
    let results = Activity::select_by_page(&rb, &rbatis::PageRequest::new(1, 10), "", None).await?;
    println!("select_by_page = {}", value!(results));
    Ok(())
}
