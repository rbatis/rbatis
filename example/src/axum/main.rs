use axum::extract::Extension;
use axum::prelude::response::Json;
use axum::prelude::*;
use axum::AddExtensionLayer;
use example::BizActivity;
use rbatis::core::runtime::sync::Arc;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use serde_json::Value;
use std::net::SocketAddr;

//mysql driver url
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

#[tokio::main]
async fn main() {
    //log
    fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);

    log::info!("linking database...");
    let rb = Rbatis::new();
    rb.link(MYSQL_URL).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);
    log::info!("linking database successful!");

    // build our application with a route
    let app = route("/", get(handler)).layer(AddExtensionLayer::new(rb));
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Extension(rb): Extension<Arc<Rbatis>>) -> Json<Value> {
    let v = rb.fetch_list::<BizActivity>().await.unwrap();
    response::Json(serde_json::json!(v))
}
