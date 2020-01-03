

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::thread::sleep;
use std::time::Duration;

async fn index() -> impl Responder {

    for i in 0..10000000{
       let r:serde_json::Value= serde_json::from_str(r#"{"code":1,"msg":"成功","data":{"page":0,"page_size":20,"totalPages":1,"total":8,"content":[{"create_time":"2019-12-20T19:05:48+08:00","default_order_amount":50000,"delete_flag":1,"each_time":48,"id":"16c2eff3-a11a-4d97-951c-a9534987","min_order_amount":20000,"name":"48个月5.5折","rate":55,"sold":459,"status":1,"version":4},{"create_time":"2019-12-20T18:57:59+08:00","default_order_amount":50000,"delete_flag":1,"each_time":36,"id":"d7e8a1d8-4623-4fd3-9e6e-a71ad0ac","min_order_amount":20000,"name":"36个月6.2折","rate":62,"sold":533,"status":1,"version":2},{"create_time":"2019-12-20T19:06:41+08:00","default_order_amount":50000,"delete_flag":1,"each_time":24,"id":"efc35823-8fea-483a-b20e-d5df1dc6","min_order_amount":20000,"name":"24个月6.9折","rate":69,"sold":323,"status":1,"version":3},{"create_time":"2019-06-04T16:59:15+08:00","default_order_amount":50000,"delete_flag":1,"each_time":18,"id":"d293ffeb-da2c-4cba-9b27-d37abf76a7b8","min_order_amount":20000,"name":"18月75折","rate":75,"sold":4325,"status":1,"version":1},{"create_time":"2019-06-04T16:58:43+08:00","default_order_amount":50000,"delete_flag":1,"each_time":12,"id":"17dfe81b-bf62-4439-86c8-3b869c73967f","min_order_amount":20000,"name":"12月8折","rate":80,"sold":10155,"status":1,"version":43},{"create_time":"2019-06-04T16:53:03+08:00","default_order_amount":50000,"delete_flag":1,"each_time":6,"id":"0e9a9d07-ecbb-4fb4-94f9-ce2ded6c59e2","min_order_amount":20000,"name":"8.8折6个月","rate":88,"sold":4383,"status":1,"version":28},{"create_time":"2019-06-02T23:29:37+08:00","default_order_amount":50000,"delete_flag":1,"each_time":3,"id":"e226858c-24d1-4908-b5f0-dc40ef6fe784","min_order_amount":20000,"name":"9.2折3个月","rate":92,"sold":5736,"status":1,"version":318}]}}"#).unwrap();
       println!("{}",r);
    }
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}




pub trait FutureId {
    fn id()->String;
}


#[test]
pub fn test_future(){
  //  main();
}