
//过程宏
pub trait HelloMacro {
    fn decode_name() -> &'static str;
}

//array
impl <T>HelloMacro for Vec<T>{
    fn decode_name() -> &'static str{
        return "Vec";
    }
}
//array
impl  <T>HelloMacro for [T]{
    fn decode_name() -> &'static str{
        return "Array";
    }
}
//array
impl  <T>HelloMacro for &[T]{
    fn decode_name() -> &'static str{
        return "Slice";
    }
}