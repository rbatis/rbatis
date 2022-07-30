use tiberius::{ ColumnType, FromSql, FromSqlOwned};
use rbs::Value;

pub trait Decode{
    fn decode(data:&[u8],t:ColumnType)->Value;
}

impl Decode for Value{
    fn decode(data: &[u8], t: ColumnType) -> Value {
        match t{
            ColumnType::Null => {Value::Null}
            ColumnType::Bit => {
                todo!()
            }
            ColumnType::Int1 => {Value::Bool(data[0] != 0)}
            ColumnType::Int2 => {todo!()}
            ColumnType::Int4 => {todo!()}
            ColumnType::Int8 => {todo!()}
            ColumnType::Datetime4 => {todo!()}
            ColumnType::Float4 => {todo!()}
            ColumnType::Float8 => {todo!()}
            ColumnType::Money => {todo!()}
            ColumnType::Datetime => {todo!()}
            ColumnType::Money4 => {todo!()}
            ColumnType::Guid => {todo!()}
            ColumnType::Intn => {todo!()}
            ColumnType::Bitn => {todo!()}
            ColumnType::Decimaln => {todo!()}
            ColumnType::Numericn => {todo!()}
            ColumnType::Floatn => {todo!()}
            ColumnType::Datetimen => {todo!()}
            ColumnType::Daten => {todo!()}
            ColumnType::Timen => {todo!()}
            ColumnType::Datetime2 => {todo!()}
            ColumnType::DatetimeOffsetn => {todo!()}
            ColumnType::BigVarBin => {todo!()}
            ColumnType::BigVarChar => {todo!()}
            ColumnType::BigBinary => {todo!()}
            ColumnType::BigChar => {todo!()}
            ColumnType::NVarchar => {todo!()}
            ColumnType::NChar => {todo!()}
            ColumnType::Xml => {todo!()}
            ColumnType::Udt => {todo!()}
            ColumnType::Text => {todo!()}
            ColumnType::Image => {todo!()}
            ColumnType::NText => {todo!()}
            ColumnType::SSVariant => {todo!()}
        }
    }
}