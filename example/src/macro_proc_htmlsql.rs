use std::fs::File;
use std::io::Read;

use rbatis::executor::RbatisExecutor;
use rbatis::sql::page::{Page, PageRequest};
use rbatis::rbatis::Rbatis;

use crate::{init_sqlite, BizActivity};

///select page must have  '?:&PageRequest' arg and return 'Page<?>'
#[html_sql("example/example.html")]
async fn select_by_condition(
    mut rb: RbatisExecutor<'_>,
    page_req: &PageRequest,
    name: &str,
    dt: &rbatis::core::datetime::DateTime,
) -> Page<BizActivity> {
    impled!()
}

#[tokio::main]
pub async fn main() {
    fast_log::init(fast_log::config::Config::new().console());
    //use static ref
    let rb = init_sqlite().await;
    let a = select_by_condition(
        rb.as_executor(),
        &PageRequest::new(1, 10),
        "test",
        &rbatis::core::datetime::DateTime::now(),
    )
        .await
        .unwrap();
    println!("{:?}", a);
}

