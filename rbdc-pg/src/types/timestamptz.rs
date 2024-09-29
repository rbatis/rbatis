use rbdc::DateTime;

pub type Timestamptz = DateTime;


#[cfg(test)]
mod test {
    use crate::types::timestamptz::Timestamptz;

    #[test]
    fn test_de() {
        let tz = Timestamptz::now();
        let v = rbs::to_value(&tz).unwrap();
        println!("{:?}", v);
        let r: Timestamptz = rbs::from_value(v).unwrap();
        assert_eq!(r, tz);
    }

    //2024-07-26 09:03:48+00
    #[test]
    fn test_de_date() {
        let v = rbs::Value::String("2024-07-26 09:03:48+00".to_string());
        println!("{:?}", v);
        let r: Timestamptz = rbs::from_value(v).unwrap();
        println!("{:?}", r);
    }
}
