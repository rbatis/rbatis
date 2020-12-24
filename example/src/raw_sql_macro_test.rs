#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::rbatis::Rbatis;

    lazy_static! { static ref RB:Rbatis=Rbatis::new();}

    #[py_sql(RB, "select * from biz_activity where id = #{name}
                  if name != '':
                    and name != #{name}")]
    async fn py_select(name: &str) -> Option<BizActivity> {}

    #[async_std::test]
    pub async fn test_macro_py_select() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = py_select("1").await.unwrap();
    }


    /// RB是本地依赖Rbatis引用的名称,例如  dao::RB, com::xxx::RB....都可以
/// 第二个参数是标准的驱动sql，注意对应数据库参数mysql为？,pg为$1...
/// 宏会自动转换函数为  pub async fn select(name: &str) -> rbatis::core::Result<BizActivity> {}
///
    #[sql(RB, "select * from biz_activity where id = ?")]
    async fn select(name: &str) -> BizActivity {}

    #[async_std::test]
    pub async fn test_macro_select() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let a = select("1").await.unwrap();
        println!("{:?}", a);
    }


    #[py_sql(rbatis, "SELECT a1.name as name,a2.create_time as create_time
                      FROM test.biz_activity a1,biz_activity a2
                      WHERE a1.id=a2.id
                      AND a1.name=#{name}")]
    async fn join_select(rbatis: &Rbatis, name: &str) -> Option<Vec<BizActivity>> {}

    #[async_std::test]
    pub async fn test_join() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        RB.link("mysql://root:123456@localhost:3306/test").await.unwrap();
        let results = join_select(&RB, "test").await.unwrap();
        println!("data: {:?}", results);
    }
}