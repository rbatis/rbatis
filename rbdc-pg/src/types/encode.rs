use std::borrow::Cow;
use crate::arguments::{PgArgumentBuffer, PgArguments};
use crate::type_info::PgTypeInfo;
use crate::types::Oid;
use rbs::Value;
use std::mem;
use crate::types::json::Json;

pub enum IsNull {
    No,
    Yes,
}
pub trait TypeInfo{
    fn type_info(&self) -> PgTypeInfo;
}

pub trait Encode {
    fn encode(self, arg: &mut PgArgumentBuffer) -> IsNull;
}

impl From<Vec<Value>> for PgArguments {
    fn from(args: Vec<Value>) -> Self {
        let mut arg = PgArguments {
            types: Vec::with_capacity(args.len()),
            buffer: PgArgumentBuffer::default(),
        };
        for x in args {
            arg.add(x);
        }
        arg
    }
}

impl TypeInfo for Value{
    fn type_info(&self) -> PgTypeInfo {
        match self {
            Value::Null => PgTypeInfo::UNKNOWN,
            Value::Bool(_) => {
                PgTypeInfo::BOOL
            }
            Value::I32(_) => {
                PgTypeInfo::INT4
            }
            Value::I64(_) => {
                PgTypeInfo::INT8
            }
            Value::U32(_) => {
                PgTypeInfo::INT4
            }
            Value::U64(_) => {
                PgTypeInfo::INT8
            }
            Value::F32(_) => {
                PgTypeInfo::FLOAT4
            }
            Value::F64(_) => {
                PgTypeInfo::FLOAT8
            }
            Value::String(_) => PgTypeInfo::VARCHAR,
            Value::Binary(_) => {
                PgTypeInfo::BYTEA_ARRAY
            }
            Value::Array(arr) => {
                if arr.len()==0{
                    return PgTypeInfo::UNKNOWN;
                }
                arr[0].type_info().clone().to_array_element().unwrap_or(PgTypeInfo::UNKNOWN)
            }
            Value::Map(_) => {
                unimplemented!()
            }
            Value::Ext(type_name, _) => {
                match *type_name {
                    "Uuid" => {
                        PgTypeInfo::UUID
                    }
                    //decimal = 12345678
                    "Decimal" => {
                        PgTypeInfo::NUMERIC
                    }
                    //Date = "1993-02-06"
                    "Date" => {
                        PgTypeInfo::DATE
                    }
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => {
                        PgTypeInfo::TIME
                    }
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => {
                        PgTypeInfo::TIMESTAMP
                    }
                    "DateTime" => {
                        PgTypeInfo::TIMESTAMP
                    }
                    "Bool" => {
                        PgTypeInfo::BOOL
                    }
                    "Bytea" => {
                        PgTypeInfo::BYTEA
                    }
                    "Char" => {
                        PgTypeInfo::CHAR
                    }
                    "Name" => {
                        PgTypeInfo::NAME
                    }
                    "Int8" => {
                        PgTypeInfo::INT8
                    }
                    "Int2" => {
                        PgTypeInfo::INT2
                    }
                    "Int4" => {
                        PgTypeInfo::INT4
                    }
                    "Text" => {
                        PgTypeInfo::TEXT
                    }
                    "Oid" => PgTypeInfo::OID,
                    "Json" => PgTypeInfo::JSON,
                    "Point" => {
                        PgTypeInfo::POINT
                    }
                    "Lseg" => {
                        PgTypeInfo::LSEG
                    }
                    "Path" => {
                        PgTypeInfo::PATH
                    }
                    "Box" => {
                        PgTypeInfo::BOX
                    }
                    "Polygon" => {
                        PgTypeInfo::POLYGON
                    }
                    "Line" => {
                        PgTypeInfo::LINE
                    }
                    "Cidr" => {
                        PgTypeInfo::CIDR
                    }
                    "Float4" => {
                        PgTypeInfo::FLOAT4
                    }
                    "Float8" => {
                        PgTypeInfo::FLOAT8
                    }
                    "Unknown" => {
                        PgTypeInfo::UNKNOWN
                    }
                    "Circle" => {
                        PgTypeInfo::CIRCLE
                    }
                    "Macaddr8" => {
                        PgTypeInfo::MACADDR8
                    }
                    "Macaddr" => {
                        PgTypeInfo::MACADDR
                    }
                    "Inet" => {
                        PgTypeInfo::INET
                    }
                    "Bpchar" => {
                        PgTypeInfo::BPCHAR
                    }
                    "Varchar" => {
                        PgTypeInfo::VARCHAR
                    }
                    "Timestamptz" => {
                        PgTypeInfo::TIMESTAMPTZ
                    }
                    "Interval" => {
                        PgTypeInfo::INTERVAL
                    }
                    "Timetz" => {
                        PgTypeInfo::TIMETZ
                    }
                    "Bit" => {
                        PgTypeInfo::BIT
                    }
                    "Varbit" => {
                        PgTypeInfo::VARBIT
                    }
                    "Numeric" => {
                        PgTypeInfo::NUMERIC
                    }
                    "Record" => {
                        PgTypeInfo::RECORD
                    }
                    "Jsonb" => {
                        PgTypeInfo::JSONB
                    }
                    "Int4Range" => {
                        PgTypeInfo::INT4_RANGE
                    }
                    "NumRange" => {
                        PgTypeInfo::NUM_RANGE
                    }
                    "TsRange" => {
                        PgTypeInfo::TS_RANGE
                    }
                    "TstzRange" => {
                        PgTypeInfo::TSTZ_RANGE
                    }
                    "DateRange" => {
                        PgTypeInfo::DATE_RANGE
                    }
                    "Int8Range" => {
                        PgTypeInfo::INT8_RANGE
                    }
                    "Jsonpath" => {
                        PgTypeInfo::JSONPATH
                    }
                    "Money" => {
                        PgTypeInfo::MONEY
                    }
                    "Void" => {
                        PgTypeInfo::VOID
                    }
                    "Custom" => {
                        PgTypeInfo::UNKNOWN
                    }
                    "DeclareWithName" => {
                        PgTypeInfo::UNKNOWN
                    }
                    "DeclareWithOid" => {
                        PgTypeInfo::UNKNOWN
                    }
                    _ => PgTypeInfo::UNKNOWN,
                }
            }
        }
    }
}

impl Encode for Value {
    fn encode(self, arg: &mut PgArgumentBuffer) -> IsNull {
        match self {
            Value::Null => IsNull::Yes,
            Value::Bool(v) => todo!(),
            Value::I32(v) => {
                todo!()
            }
            Value::I64(v) => {
                todo!()
            }
            Value::U32(v) => {
                todo!()
            }
            Value::U64(v) => {
                todo!()
            }
            Value::F32(v) => {
                todo!()
            }
            Value::F64(v) => {
                todo!()
            }
            Value::String(v) => {
                //default -> string
                v.encode(arg)
            }
            Value::Binary(v) => {
                todo!()
            }
            Value::Array(v) => {
                v.encode(arg)
            }
            Value::Map(v) => {
                todo!()
            }
            Value::Ext(type_name, v) => {
                match type_name {
                    "Uuid" => {
                        todo!()
                    }
                    //decimal = 12345678
                    "Decimal" => {
                        todo!()
                    }
                    //Date = "1993-02-06"
                    "Date" => {
                        todo!()
                    }
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => {
                        todo!()
                    }
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => {
                        todo!()
                    }
                    "DateTime" => {
                        todo!()
                    }
                    "Bool" => {
                        todo!()
                    }
                    "Bytea" => {
                        todo!()
                    }
                    "Char" => {
                        todo!()
                    }
                    "Name" => {
                        todo!()
                    }
                    "Int8" => {
                        todo!()
                    }
                    "Int2" => {
                        todo!()
                    }
                    "Int4" => {
                        todo!()
                    }
                    "Text" => {
                        todo!()
                    }
                    "Oid" => Oid::from(v.as_u64().unwrap_or_default() as u32).encode_by_ref(arg),
                    "Json" => Json::from(v.into_string().unwrap_or_default()).encode(arg),
                    "Point" => {
                        todo!()
                    }
                    "Lseg" => {
                        todo!()
                    }
                    "Path" => {
                        todo!()
                    }
                    "Box" => {
                        todo!()
                    }
                    "Polygon" => {
                        todo!()
                    }
                    "Line" => {
                        todo!()
                    }
                    "Cidr" => {
                        todo!()
                    }
                    "Float4" => {
                        todo!()
                    }
                    "Float8" => {
                        todo!()
                    }
                    "Unknown" => {
                        todo!()
                    }
                    "Circle" => {
                        todo!()
                    }
                    "Macaddr8" => {
                        todo!()
                    }
                    "Macaddr" => {
                        todo!()
                    }
                    "Inet" => {
                        todo!()
                    }
                    "Bpchar" => {
                        todo!()
                    }
                    "Varchar" => {
                        todo!()
                    }
                    "Timestamptz" => {
                        todo!()
                    }
                    "Interval" => {
                        todo!()
                    }
                    "Timetz" => {
                        todo!()
                    }
                    "Bit" => {
                        todo!()
                    }
                    "Varbit" => {
                        todo!()
                    }
                    "Numeric" => {
                        todo!()
                    }
                    "Record" => {
                        todo!()
                    }
                    "Jsonb" => {
                        todo!()
                    }
                    "Int4Range" => {
                        todo!()
                    }
                    "NumRange" => {
                        todo!()
                    }
                    "TsRange" => {
                        todo!()
                    }
                    "TstzRange" => {
                        todo!()
                    }
                    "DateRange" => {
                        todo!()
                    }
                    "Int8Range" => {
                        todo!()
                    }
                    "Jsonpath" => {
                        todo!()
                    }
                    "Money" => {
                        todo!()
                    }
                    "Void" => {
                        todo!()
                    }
                    "Custom" => {
                        todo!()
                    }
                    "DeclareWithName" => {
                        todo!()
                    }
                    "DeclareWithOid" => {
                        todo!()
                    }
                    _ => IsNull::Yes,
                }
            }
        }
    }
}

impl TypeInfo for String{
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::VARCHAR
    }
}
impl Encode for String {
    fn encode(self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(self.into_bytes());
        IsNull::No
    }
}

impl TypeInfo for i8{
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::BYTEA
    }
}
impl Encode for i8 {

    fn encode(self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(&self.to_be_bytes());
        IsNull::No
    }
}