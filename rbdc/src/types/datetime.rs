
#[derive(serde::Serialize,serde::Deserialize,Debug,Clone,Eq, PartialEq)]
#[serde(rename = "datetime")]
pub struct DateTime(String);


#[test]
fn test(){
    let date=DateTime("2017-02-06T00-00-00".to_string());
    let v=rbs::to_value_ref(&date).unwrap();
    println!("{:?}",v);
}
