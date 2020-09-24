use geo_types::Point;

use crate::decode::Decode;
use crate::encode::Encode;
use crate::postgres::protocol::TypeId;
use crate::postgres::{PgData, PgRawBuffer, PgTypeInfo, PgValue, Postgres};
use crate::types::Type;

impl Type<Postgres> for Point<f64> {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::new(TypeId::POINT, "POINT")
    }
}

impl Type<Postgres> for [Point<f64>] {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::new(TypeId::ARRAY_POINT, "POINT[]")
    }
}
impl Type<Postgres> for Vec<Point<f64>> {
    fn type_info() -> PgTypeInfo {
        <[Point<f64>] as Type<Postgres>>::type_info()
    }
}

impl Encode<Postgres> for Point<f64> {
    fn encode(&self, buf: &mut PgRawBuffer) {
      Encode::<Postgres>::encode(&(format!("({:?},{:?})",&self.x(),&self.y())), buf);
        // (&mut **buf).append(string_data.as_mut_vec())
    }
}
fn str_to_point(string: &str) -> crate::Result<Point<f64>> {
    let coordinates: Vec<&str> = string
        .trim()
        .trim_matches(|c| c == '(' || c == ')')
        .split(",")
        .collect();
    Ok(Point::new(
        coordinates[0].parse::<f64>().unwrap(),
        coordinates[1].parse::<f64>().unwrap(),
    ))
}

impl<'de> Decode<'de, Postgres> for Point<f64> {
    fn decode(value: PgValue<'de>) -> crate::Result<Self> {
        match value.try_get()? {
            PgData::Text(s) => str_to_point(s),
            PgData::Binary(buf) => {
                str_to_point(std::str::from_utf8(&buf).unwrap())
            }
        }
    }
}
#[test]
fn test_encode_point() {
    let mut buf = PgRawBuffer::default();
    let p1: Point<f64> = (0., 1.).into();

    Encode::<Postgres>::encode(&p1, &mut buf);
     assert_eq!(
        &**buf,
        [40, 48, 46, 48, 44, 49, 46, 48, 41]
    ); 
    buf.clear();
}
#[test]
fn test_decode_point() {
     let buf = [40, 48, 46, 48, 44, 49, 46, 48, 41];
    let point: Point<f64> = Decode::<Postgres>::decode(PgValue::from_bytes(&buf)).unwrap();
    assert_eq!(point, Point::new(0., 1.));
 
    let point_str = r#"(52.0907,5.1214)"#;
    let point: Point<f64> = Decode::<Postgres>::decode(PgValue::from_str(&point_str)).unwrap();
    assert_eq!(point, Point::new(52.0907, 5.1214));
}
