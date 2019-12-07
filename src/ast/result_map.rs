
struct ResultItem{
    pub tag:String,
    pub column:String,
    pub property:String,
    pub langType:String,
    pub isLogicType:bool,
    pub isVersionType:bool,
}

struct  ResultMap {
     items:Vec<ResultItem>,
}

impl ResultMap{
    pub fn new(arg:&str)->Self{

        return Self{
            items: vec![],
        }
    }
}