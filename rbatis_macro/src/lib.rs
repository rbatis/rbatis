use std::collections::LinkedList;
use std::iter::Map;
use std::rc::Rc;
use std::sync::Arc;

//过程宏
pub trait RbatisMacro {
    fn decode_name() -> &'static str;
}

//array
impl <T>RbatisMacro for Vec<T>{
    fn decode_name() -> &'static str{
        return "Vec";
    }
}
//array
impl  <T>RbatisMacro for [T]{
    fn decode_name() -> &'static str{
        return "Array";
    }
}
//array
impl  <T>RbatisMacro for &[T]{
    fn decode_name() -> &'static str{
        return "Slice";
    }
}
//array
impl  <T>RbatisMacro for LinkedList<T>{
    fn decode_name() -> &'static str{
        return "LinkedList";
    }
}
//map
impl  <T>RbatisMacro for Map<String,T>{
    fn decode_name() -> &'static str{
        return "Map";
    }
}

//Rc
impl  <T>RbatisMacro for Rc<T>{
    fn decode_name() -> &'static str{
        return "Rc";
    }
}
//Rc
impl  <T>RbatisMacro for Arc<T>{
    fn decode_name() -> &'static str{
        return "Arc";
    }
}

//Option
impl  <T>RbatisMacro for Option<T>{
    fn decode_name() -> &'static str{
        return "Option";
    }
}

//serde_json::Value
impl  RbatisMacro for serde_json::Value{
    fn decode_name() -> &'static str{
        return "serde_json::Value";
    }
}

impl RbatisMacro for String{
    fn decode_name() -> &'static str {
       return "String"
    }
}