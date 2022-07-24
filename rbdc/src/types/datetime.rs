use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename = "datetime")]
pub struct DateTime(String);

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "datetime")?;
        write!(f, "{}", self.0)?;
        write!(f, ")")
    }
}

#[test]
fn test() {
    let date = DateTime("2017-02-06T00-00-00".to_string());
    let v = rbs::to_value_ref(&date).unwrap();
    println!("{}", v);
}
