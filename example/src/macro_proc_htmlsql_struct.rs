//! Example: Using #[html_sql] at impl block level

use rbatis::dark_std::defer;
use rbatis::executor::Executor;
use rbatis::plugin::PageRequest;
use rbatis::rbdc::datetime::DateTime;
use rbatis::{Error, RBatis};

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

/// All methods read SQL from example/example.html
#[rbatis::html_sql("example/example.html")]
impl Activity {
    /// Maps to <select id="select_by_condition"> in HTML
    pub async fn select_by_condition(
        rb: &dyn Executor,
        name: &str,
        dt: &DateTime,
    ) -> rbatis::Result<Vec<Activity>> {
        impled!()
    }

    /// Maps to <select id="select_page_data"> in HTML
    pub async fn select_page_data(
        rb: &dyn Executor,
        name: &str,
        dt: &DateTime,
    ) -> rbatis::Result<Vec<Activity>> {
        impled!()
    }

    /// Maps to <update id="update_by_id"> in HTML
    pub async fn update_by_id(
        rb: &dyn Executor,
        arg: &Activity,
    ) -> rbatis::Result<rbatis::rbdc::db::ExecResult> {
        impled!()
    }

    /// Paginated query - automatically detected by return type Page<Activity>
    /// Maps to <select id="select_by_page"> in HTML
    /// The macro automatically generates pagination logic using PageIntercept
    pub async fn select_by_page(
        rb: &dyn Executor,
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
            .level(log::LevelFilter::Debug),
    );
    defer!(|| {
        log::logger().flush();
    });

    let rb = RBatis::new();
    rb.init(
        rbdc_sqlite::driver::SqliteDriver {},
        "sqlite://target/sqlite.db",
    )
    .unwrap();

    // Use impl block Mapper
    let results = Activity::select_by_page(&rb, &PageRequest::new(1, 10), "", None).await?;
    println!("Query by condition: {:?}", results);
    Ok(())
}
