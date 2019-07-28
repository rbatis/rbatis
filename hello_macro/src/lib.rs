

pub trait HelloMacro {
    fn hello_macro() -> bool;
}


impl HelloMacro for String{
    fn hello_macro() -> bool{
        println!("String not array");
        return false;
    }
}


impl <T>HelloMacro for Vec<T>{
    fn hello_macro() -> bool {
        println!("vec is array");
        return true;
    }
}