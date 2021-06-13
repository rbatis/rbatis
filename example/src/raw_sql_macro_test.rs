#[cfg(test)]
mod test {
    use crate::BizActivity;
    use rbatis::rbatis::Rbatis;

    lazy_static! {
        static ref RB: Rbatis = Rbatis::new();
    }

    #[py_sql(
        RB,
        "select * from biz_activity where id = #{name}
                  if name != '':
                    and name != #{name}","mysql"
    )]
    async fn py_select(name: &str) -> Option<BizActivity> {todo!()}

    #[tokio::test]
    pub async fn test_macro_py_select() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        RB.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_select("1").await.unwrap();
    }

    /// doc you can see https://rbatis.github.io/rbatis.io/#/en/
    #[sql(RB, "select * from biz_activity where id = ?")]
    async fn select(name: &str) -> BizActivity {todo!()}

    #[tokio::test]
    pub async fn test_macro_select() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        RB.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = select("1").await.unwrap();
        println!("{:?}", a);
    }

    #[py_sql(
        rbatis,
        "select a1.name as name,a2.create_time as create_time
                      from test.biz_activity a1,biz_activity a2
                      where a1.id=a2.id
                      and a1.name=#{name}","mysql")]
    async fn join_select(rbatis: &Rbatis, name: &str) -> Option<Vec<BizActivity>> {todo!()}

    #[tokio::test]
    pub async fn test_join() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        RB.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let results = join_select(&RB, "test").await.unwrap();
        println!("data: {:?}", results);
    }
}
