pub mod string;
pub mod value;
/// this mod All common types

/// string type have subtype:
/// date="2006-01-02"
/// time="15:04:05.999999"
/// datetime="2006-01-02 15:04:05.999999"
/// timestamp=9999999999999Z
/// decimal=12345678D
/// uuid=4b3f82bc-fa70-48e5-914c-17f0c8d246e2
/// json={"abc":null} or [{"abc":null}]
pub trait Type {
    fn type_name(&self) -> &'static str;
}
