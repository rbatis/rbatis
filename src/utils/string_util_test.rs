use crate::utils::string_util::find_convert_string;

#[test]
fn test_string_util(){
    let s= find_convert_string("#{arg1},,,#{arg2}sadfsadf#{arg3}");
    for i in s{
        println!("{}",i);
    }
}