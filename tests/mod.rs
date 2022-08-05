#[macro_use]
extern crate rbatis;


#[cfg(test)]
mod test{
    use rbs::Value;
    use rbs::value::map::ValueMap;

    #[test]
    fn test_value_iter(){
        let v=Value::Array(vec![Value::I32(1)]);
        for (k,v) in v {
            println!("{},{}",k,v);
        }
        let v=Value::Array(vec![Value::I32(1)]);
        for (k,v) in &v {
            println!("{},{}",k,v);
        }
        let v=Value::Map(ValueMap::new());
        for (k,v) in v {
            println!("{},{}",k,v);
        }
    }
}