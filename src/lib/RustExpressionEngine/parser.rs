use std::collections::HashMap;

pub struct OptMap<'a> {
    pub Map: HashMap<&'a str, bool>,
    //全部操作符
    pub MulOpsMap: HashMap<&'a str, bool>,
    //复合操作符
    pub SingleOptMap: HashMap<&'a str, bool>, //单操作符
}

impl<'a> OptMap<'a> {
    pub fn new() -> Self {
        let mut defMap = HashMap::new();
        defMap.insert("+", true);
        defMap.insert("-", true);
        defMap.insert("*", true);
        defMap.insert("/", true);

        defMap.insert("==", true);
        defMap.insert("!=", true);
        defMap.insert(">=", true);
        defMap.insert("<=", true);

        let mut MulOpsMap = HashMap::new();
        let mut SingleOptMap = HashMap::new();
        for (k, v) in &defMap {
            if k.len() > 1 {
                MulOpsMap.insert(k.clone(), v.clone());
            } else {
                SingleOptMap.insert(k.clone(), v.clone());
            }
        }

        Self {
            Map: defMap,
            MulOpsMap: MulOpsMap,
            SingleOptMap: SingleOptMap,
        }
    }
}

pub fn Parser(str: String) -> String {
    let optMap = OptMap::new();

    for (k, v) in optMap.MulOpsMap {}

    return String::from("ds");
}

