use rbs::Value;

pub enum IsNull {
    No,
    Yes,
}
pub trait Encode {
    fn encode(self) -> IsNull;
}

impl Encode for Value {
    fn encode(self) -> IsNull {
        todo!()
    }
}
