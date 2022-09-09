#[cfg(test)]
mod test {
    use rbdc::db::Placeholder;
    use rbdc::impl_exchange;

    #[test]
    fn test_exchange() {
        pub struct MyDriver{}
        impl rbdc::db::Placeholder for MyDriver {
            fn exchange(&self, sql: &str) -> String {
                impl_exchange("@P",1,sql)
            }
        }
        let v = "insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)";
        let d = MyDriver {};
        let sql = d.exchange(v);
        assert_eq!("insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag) VALUES (@P1,@P2,@P3,@P4,@P5,@P6,@P7,@P8,@P9,@P10,@P11,@P12)", sql);
    }
}