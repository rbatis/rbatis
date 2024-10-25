pub use self::de::{from_value, from_value_ref};
pub use self::se::{to_value, to_value_def};

mod de;
mod se;

#[cfg(test)]
mod test {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::ser::SerializeMap;
    use crate::{to_value, Value};

    #[test]
    fn test_ser() {
        let s = to_value!(1);
        assert_eq!(s, Value::I32(1));
    }

    pub struct A {}
    impl Serialize for A {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            println!("{}", std::any::type_name::<S>());
            serializer.serialize_map(None)
                ?.end()
        }
    }

    impl<'de> Deserialize<'de> for A {
        fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            println!("{}", std::any::type_name::<D>());
            Ok(A {})
        }
    }

    #[test]
    fn test_ser_struct() {
        let v = to_value!(A{});
        let _: A = crate::from_value(v).unwrap();
    }
}