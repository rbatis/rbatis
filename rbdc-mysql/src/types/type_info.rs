use crate::protocol::text::ColumnType;
use crate::result_set::MySqlTypeInfo;
use crate::types::TypeInfo;
use rbs::Value;

impl TypeInfo for Value {
    fn type_info(&self) -> MySqlTypeInfo {
        match self {
            Value::Null => MySqlTypeInfo::null(),
            Value::Bool(_) => MySqlTypeInfo::from_type(ColumnType::Tiny),
            Value::I32(_) => MySqlTypeInfo::from_type(ColumnType::Long),
            Value::I64(_) => MySqlTypeInfo::from_type(ColumnType::LongLong),
            Value::U32(_) => MySqlTypeInfo::from_type(ColumnType::Long),
            Value::U64(_) => MySqlTypeInfo::from_type(ColumnType::LongLong),
            Value::F32(_) => MySqlTypeInfo::from_type(ColumnType::Float),
            Value::F64(_) => MySqlTypeInfo::from_type(ColumnType::Double),
            Value::String(_) => MySqlTypeInfo::from_type(ColumnType::VarChar),
            Value::Binary(_) => MySqlTypeInfo::from_type(ColumnType::Blob),
            Value::Array(_) => MySqlTypeInfo::null(),
            Value::Map(_) => MySqlTypeInfo::null(),
            Value::Ext(ext_type, _) => {
                match *ext_type {
                    "Uuid" => MySqlTypeInfo::from_type(ColumnType::VarChar),
                    //decimal = 12345678
                    "Decimal" => MySqlTypeInfo::from_type(ColumnType::NewDecimal),
                    //year = "1993"
                    "Year" => MySqlTypeInfo::from_type(ColumnType::Year),
                    //Date = "1993-02-06"
                    "Date" => MySqlTypeInfo::from_type(ColumnType::Date),
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => MySqlTypeInfo::from_type(ColumnType::Time),
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => {
                        //datetime=5byte
                        MySqlTypeInfo::from_type(ColumnType::Timestamp)
                    }
                    "DateTime" => MySqlTypeInfo::from_type(ColumnType::Datetime),
                    "Json" => MySqlTypeInfo::from_type(ColumnType::Json),
                    "Enum" => MySqlTypeInfo::from_type(ColumnType::Enum),
                    "Set" => MySqlTypeInfo::from_type(ColumnType::Set),
                    _ => MySqlTypeInfo::null(),
                }
            }
        }
    }
}
