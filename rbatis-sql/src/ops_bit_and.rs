use crate::ops::BitAnd;
use crate::ops::Value;

impl BitAnd for Value{
    type Output = bool;

    fn op_bitand(self, rhs: Self) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<Value> for bool{
    type Output = bool;

    fn op_bitand(self, rhs: Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}

//ref value
impl BitAnd<Value> for &Value{
    type Output = bool;

    fn op_bitand(self, rhs: Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&Value> for &Value{
    type Output = bool;

    fn op_bitand(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&&Value> for &Value{
    type Output = bool;

    fn op_bitand(self, rhs: &&Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<bool> for &Value{
    type Output = bool;

    fn op_bitand(self, rhs: bool) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs
    }
}


//rhs ref
impl BitAnd<&Value> for Value{
    type Output = bool;

    fn op_bitand(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&Value> for bool{
    type Output = bool;

    fn op_bitand(self, rhs: &Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&&Value> for Value{
    type Output = bool;

    fn op_bitand(self, rhs: &&Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&&Value> for bool{
    type Output = bool;

    fn op_bitand(self, rhs: &&Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}