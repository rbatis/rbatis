use std::collections::HashMap;

enum NodeType {
  NArg,            //参数节点
  NString,          //string 节点
  NFloat,           //float节点
  NInt,             //int 节点
  NUInt,           //uint节点
  NBool,           //bool节点
  NNull,           //空节点
  NBinary,         //二元计算节点
  NOpt,           //操作符节点
}

//抽象语法树节点
trait Node {
    fn Type() -> NodeType;
    fn Eval(env: HashMap<&str, &str>) -> (String, String);
}