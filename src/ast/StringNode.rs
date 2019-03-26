
use crate::utils::string_util;

/**
*  string抽象节点
**/
pub struct StringNode {
    pub value: String,
    //去重的，需要替换的要sql转换express map
    pub expressMap: Vec<String>,
    //去重的，需要替换的免sql转换express map
    pub noConvertExpressMap: Vec<String>,
}

impl StringNode{
    fn new(v:&String)->Self{
        //TODO find v #[] and find v$[]
        Self{
            value: v.clone(),
            expressMap: string_util::findConvertString(v.clone()),
            noConvertExpressMap: string_util::findNoConvertString(v.clone()),
        }
    }
}