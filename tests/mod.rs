#[cfg(test)]
mod test {
    use rbs::value::map::ValueMap;
    use rbs::Value;

    #[test]
    fn test_value_iter() {
        let v = Value::Array(vec![Value::I32(1)]);
        for (k, v) in &v {
            println!("{},{}", k, v);
        }
        for (k, v) in v {
            println!("{},{}", k, v);
        }
        let mut m = ValueMap::new();
        m.insert(1.into(), 1.into());
        let v = Value::Map(m);
        for (k, v) in &v {
            println!("{},{}", k, v);
        }
        for (k, v) in v {
            println!("{},{}", k, v);
        }
    }
}
