use crate::utils::string_util::findConvertString;

#[test]
fn TestStringUtil(){
    let s=findConvertString("#{arg1},,,#{arg2}sadfsadf#{arg3}");
    for i in s{
        println!("{}",i);
    }
}