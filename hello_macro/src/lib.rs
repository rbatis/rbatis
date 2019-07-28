

pub trait HelloMacro {
    fn is_array() -> bool;
}


impl HelloMacro for String{
    fn is_array() -> bool{
        println!("String not array");
        return false;
    }
}


//array
impl <T>HelloMacro for Vec<T>{
    fn is_array() -> bool {
        println!("vec is array");
        return true;
    }
}
//array
impl  <T>HelloMacro for [T]{
    fn is_array() -> bool {
        println!("[] is array");
        return true;
    }
}
//array
impl  <T>HelloMacro for &[T]{
    fn is_array() -> bool {
        println!("&[] is array");
        return true;
    }
}