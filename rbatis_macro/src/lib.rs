
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