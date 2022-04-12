use std::net::SocketAddr;
use std::sync::Arc;
use rbatis::rbatis::Rbatis;
use example::BizActivity;
use rbatis::crud::CRUD;
use serde_json::Value;
use axum::extract::Extension;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use axum::routing::get;

//mysql driver url
pub const MYSQL_URL: &'static str = "mysql://root:123456@localhost:3306/test";

//handler
pub async fn handler(rb: Extension<Arc<Rbatis>>) -> Json<Value> {
    let v = rb.fetch_list::<BizActivity>().await.unwrap_or_default();
    Json(serde_json::json!(v))
}

#[tokio::main]
async fn main() {
    //log
    fast_log::init(fast_log::config::Config::new().console());

    log::info!("linking database...");
    let rb = Rbatis::new();
    rb.link(MYSQL_URL).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);
    log::info!("linking database successful!");

    // build our application with a route
    let app = Router::new().route("/", get(handler))
        .layer(Extension(rb));
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}