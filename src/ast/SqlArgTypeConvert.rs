use serde_json::Value;

pub trait SqlArgTypeConvert {
    fn convert(&self,arg: Value) -> String;
}
