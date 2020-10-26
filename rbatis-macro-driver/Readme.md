

### rbatis macro driver
### rbatis的过程宏项目，免去手写 impl CRUDEnable 接口的实现简化操作


### use way,使用方法

#### toml
```toml
#rbatis dep
rbatis = { path = "../" }
rbatis-core = { path = "../rbatis-core", features = ["all"] }
rbatis-macro-driver = { path = "../rbatis-macro-driver"}
```
#### code
```rust

#[macro_use]
extern crate rbatis_macro_driver;

///数据库表模型,支持BigDecimal ,DateTime ,rust基本类型（int,float,uint,string,Vec,Array）
/// CRUDEnable 特性会自动识别 id为表的id类型(识别String)，自动识别结构体名称为蛇形命名的表名 biz_activity。没有id的表 请手动指定
#[derive(CRUDEnable,Serialize, Deserialize, Clone, Debug)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub version: Option<i32>,
    pub delete_flag: Option<i32>,
}

fn main(){
   async_std::task::block_on(async {
          fast_log::init_log("requests.log", 1000,log::Level::Info,true);
          let rb = Rbatis::new();
          rb.link("mysql://root:123456@localhost:3306/test").await.unwrap();
          let r = rb.fetch_by_id::<Option<BizActivity>>("", &"1".to_string()).await.unwrap();
          println!("{}",serde_json::to_string(&r).unwrap());
      });
}
```