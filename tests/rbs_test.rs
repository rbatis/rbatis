#[cfg(test)]
mod test {
    use rbs::Value;

    #[test]
    fn test_ser_i32() {
        let v = rbs::to_value!(1);
        assert_eq!(v, Value::I32(1));
    }

    #[test]
    fn test_ser_i64() {
        let v = rbs::to_value!(1i64);
        assert_eq!(v, Value::I64(1));
    }

    #[test]
    fn test_ser_u32() {
        let v = rbs::to_value!(1u32);
        assert_eq!(v, Value::U32(1));
    }

    #[test]
    fn test_ser_u64() {
        let v = rbs::to_value!(1u64);
        assert_eq!(v, Value::U64(1));
    }

    #[test]
    fn test_ser_f32() {
        let v = rbs::to_value!(1f32);
        assert_eq!(v, Value::F32(1.0));
    }

    #[test]
    fn test_ser_f64() {
        let v = rbs::to_value!(1f64);
        assert_eq!(v, Value::F64(1.0));
    }

    #[test]
    fn test_ser_bool() {
        let v = rbs::to_value!(true);
        assert_eq!(v, Value::Bool(true));
    }

    #[test]
    fn test_ser_null() {
        let v = rbs::to_value!(());
        assert_eq!(v, Value::Null);
    }

    #[test]
    fn test_ser_str() {
        let v = rbs::to_value!("1");
        assert_eq!(v, Value::String("1".to_string()));
    }
}
