

use crate::ops::{Value, From, AsProxy};

impl From<bool> for bool{
    fn op_from(arg: bool) -> Self {
        arg
    }
}
impl From<&bool> for bool{
    fn op_from(arg: &bool) -> Self {
        *arg
    }
}
impl From<&&bool> for bool{
    fn op_from(arg: &&bool) -> Self {
        **arg
    }
}


impl From<&Value> for bool{
    fn op_from(arg: &Value) -> Self {
        arg.bool()
    }
}

impl From<&&Value> for bool{
    fn op_from(arg: &&Value) -> Self {
        arg.bool()
    }
}

impl From<Value> for bool{
    fn op_from(arg: Value) -> Self {
        arg.bool()
    }
}