
struct Example{
   pub selectByCondition:fn()
}


#[test]
fn testWriteMethod(){
    let e=Example{
        selectByCondition: || {println!("select * from table");}
    };
    (e.selectByCondition)();
}