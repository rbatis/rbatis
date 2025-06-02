use rbs::Value;
use rbatis::crud_gen::gen_insert_batch;

#[tokio::test]
async fn test_gen_insert_batch() {
    let v = vec![rbs::value! {
        "a":1,
        "b":2,
    }];
   let (sql,args) = gen_insert_batch("table",Value::Array(v.clone())).await.unwrap();
   println!("{}", sql);
   println!("{}", Value::Array(args.clone())); 
}