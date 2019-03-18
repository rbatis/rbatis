use std::collections::HashMap;

pub struct  OptMap<'a> {
    pub OpsMap:HashMap< &'a str,bool>
}

impl <'a>OptMap<'a>{
    pub fn new()->Self{
        let mut defMap=HashMap::new();
        defMap.insert("+",true);
        defMap.insert("-",true);
        defMap.insert("*",true);
        defMap.insert("/",true);

        defMap.insert("==",true);
        defMap.insert("!=",true);
        defMap.insert(">=",true);
        defMap.insert("<=",true);

        Self{
            OpsMap:defMap
        }
    }
}

pub fn Parser() -> String {
    return String::from("ds");
}

