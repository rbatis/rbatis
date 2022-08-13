use chrono::Utc;
use rbs::Value;
use tiberius::numeric::BigDecimal;
use tiberius::xml::XmlData;
use tiberius::ColumnType;
use rbdc::Error;

pub trait Decode {
    fn decode(row: &tiberius::Row, i: usize, t: ColumnType) -> Result<Value,Error>;
}

impl Decode for Value {
    fn decode(row: &tiberius::Row, i: usize, t: ColumnType) -> Result<Value,Error> {
        Ok(match t {
            ColumnType::Null => Value::Null,
            ColumnType::Bit => {
                let data: bool = row.get(i).unwrap_or_default();
                Value::I32(data as i32)
            }
            ColumnType::Int1 => {
                let data: u8 = row.get(i).unwrap_or_default();
                Value::I32(data as i32)
            }
            ColumnType::Int2 => {
                let data: i16 = row.get(i).unwrap_or_default();
                Value::I32(data as i32)
            }
            ColumnType::Int4 => {
                let data: i32 = row.get(i).unwrap_or_default();
                Value::I32(data)
            }
            ColumnType::Int8 => {
                let data: i64 = row.get(i).unwrap_or_default();
                Value::I64(data)
            }
            ColumnType::Datetime4 => {
                let v: chrono::NaiveDateTime = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Datetime4 fail or none"));
                        }
                    }
                };
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Float4 => {
                let data: f32 = row.get(i).unwrap_or_default();
                Value::F32(data)
            }
            ColumnType::Float8 => {
                let data: f64 = row.get(i).unwrap_or_default();
                Value::F64(data)
            }
            ColumnType::Money => {
                let v: f64 = row.get(i).unwrap_or_default();
                Value::F64(v)
            }
            ColumnType::Datetime => {
                let v: chrono::NaiveDateTime = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Datetime fail or none"));
                        }
                    }
                };
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Money4 => {
                let v: f32 = row.get(i).unwrap_or_default();
                Value::F32(v)
            }
            ColumnType::Guid => {
                let data: uuid::Uuid = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Guid fail or none"));
                        }
                    }
                };
                Value::String(data.to_string()).into_ext("Guid")
            }
            ColumnType::Intn => {
                let data: i32 = row.get(i).unwrap_or_default();
                Value::I32(data.to_owned())
            }
            ColumnType::Bitn => {
                let data: bool = row.get(i).unwrap_or_default();
                Value::Bool(data)
            }
            ColumnType::Decimaln => {
                let data: BigDecimal = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Decimaln fail or none"));
                        }
                    }
                };
                Value::String(data.to_string()).into_ext("Decimal")
            }
            ColumnType::Numericn => {
                let v: BigDecimal = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Numericn fail or none"));
                        }
                    }
                };
                Value::String(v.to_string()).into_ext("Decimal")
            }
            ColumnType::Floatn => {
                let data: f64 = row.get(i).unwrap_or_default();
                Value::F64(data)
            }
            ColumnType::Datetimen => {
                let v: chrono::NaiveDateTime = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Datetimen fail or none"));
                        }
                    }
                };
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Daten => {
                let v: chrono::NaiveDate = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Daten fail or none"));
                        }
                    }
                };
                Value::String(v.to_string()).into_ext("Date")
            }
            ColumnType::Timen => {
                let v: chrono::NaiveTime = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Timen fail or none"));
                        }
                    }
                };
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::Datetime2 => {
                let v: chrono::NaiveDateTime = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Datetime2 fail or none"));
                        }
                    }
                };
                Value::String(v.to_string()).into_ext("Datetime")
            }
            ColumnType::DatetimeOffsetn => {
                let data: chrono::DateTime<Utc> = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode DatetimeOffsetn fail or none"));
                        }
                    }
                };
                Value::String(data.to_string()).into_ext("Datetime")
            }
            ColumnType::BigVarBin => {
                let data: &[u8] = row.get(i).unwrap_or_default();
                Value::Binary(data.to_owned()).into_ext("BigVarBin")
            }
            ColumnType::BigVarChar => {
                let v: &str = row.get(i).unwrap_or_default();
                Value::String(v.to_string())
            }
            ColumnType::BigBinary => {
                let data: &[u8] = row.get(i).unwrap_or_default();
                Value::Binary(data.to_owned())
            }
            ColumnType::BigChar => {
                let v: &str = row.get(i).unwrap_or_default();
                Value::String(v.to_string())
            }
            ColumnType::NVarchar => {
                let v: &str = row.get(i).unwrap_or_default();
                Value::String(v.to_string())
            }
            ColumnType::NChar => {
                let v: &str = row.get(i).unwrap_or_default();
                Value::String(v.to_string())
            }
            ColumnType::Xml => {
                let data: &XmlData = {
                    match row.get(i){
                        Some(v)=>{
                            v
                        }
                        None=>{
                            return Err(Error::from("decode Xml fail or none"));
                        }
                    }
                };
                Value::String(data.to_string()).into_ext("Xml")
            }
            ColumnType::Udt => {
                let data: &[u8] = row.get(i).unwrap_or_default();
                Value::Binary(data.to_owned()).into_ext("Udt")
            }
            ColumnType::Text => {
                let v: &str = row.get(i).unwrap_or_default();
                Value::String(v.to_string())
            }
            ColumnType::Image => {
                let data: &[u8] = row.get(i).unwrap_or_default();
                Value::Binary(data.to_owned())
            }
            ColumnType::NText => {
                let v: &str = row.get(i).unwrap_or_default();
                Value::String(v.to_string())
            }
            ColumnType::SSVariant => {
                let data: &[u8] = row.get(i).unwrap_or_default();
                Value::Binary(data.to_owned()).into_ext("SSVariant")
            }
        })
    }
}
