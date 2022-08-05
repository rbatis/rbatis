#[macro_use]
extern crate rbatis;


#[cfg(test)]
mod test{
    use rbs::Value;

    #[test]
    fn test_value_iter(){
        let v=Value::Array(vec![Value::I32(1)]);
        for (k,v) in v {
            println!("{},{}",k,v);
        }
    }
}