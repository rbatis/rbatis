#[cfg(test)]
mod test {
    use rbdc::db::Placeholder;
    use rbdc::impl_exchange;
    use rbs::Value;

    #[test]
    fn test_bytes_iter() {
        let bytes1 = rbdc::Bytes::from(vec![1u8]);
        for v in bytes1 {
            assert_eq!(v, 1u8);
        }
    }

    #[test]
    fn test_bytes_old() {
        let bytes1 = rbdc::Bytes::from(vec![1u8]);
        let bytes2 = rbdc::Bytes::from(vec![0u8]);
        assert_eq!(bytes1 > bytes2, true);
    }

    #[test]
    fn test_bytes_eq() {
        let bytes1 = rbdc::Bytes::from(vec![0u8]);
        let bytes2 = rbdc::Bytes::from(vec![0u8]);
        assert_eq!(bytes1, bytes2);
    }

    #[test]
    fn test_bytes_debug() {
        let bytes = rbdc::Bytes::from(vec![0u8]);
        assert_eq!(format!("{:?}", bytes), "[0]");
    }

    #[test]
    fn test_ser_bytes() {
        let bytes = rbdc::Bytes::from(vec![0u8]);
        let v = rbs::to_value!(bytes);
        assert_eq!(v, Value::Binary(vec![0u8]));
    }

    #[test]
    fn test_de_bytes() {
        let bytes = rbdc::Bytes::from(vec![0u8]);
        let v = rbs::to_value!(&bytes);
        let r: rbdc::Bytes = rbs::from_value(v).unwrap();
        assert_eq!(r, bytes);
    }

    #[test]
    fn test_exchange() {
        pub struct MyDriver {}
        impl rbdc::db::Placeholder for MyDriver {
            fn exchange(&self, sql: &str) -> String {
                impl_exchange("@P", 1, sql)
            }
        }
        let v = "insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)";
        let d = MyDriver {};
        let sql = d.exchange(v);
        assert_eq!("insert into biz_activity (id,name,pc_link,h5_link,pc_banner_img,h5_banner_img,sort,status,remark,create_time,version,delete_flag) VALUES (@P1,@P2,@P3,@P4,@P5,@P6,@P7,@P8,@P9,@P10,@P11,@P12)", sql);
    }
}
