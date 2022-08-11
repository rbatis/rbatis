use crate::arguments::PgArgumentBuffer;
use crate::type_info::{PgType, PgTypeInfo, PgTypeKind};
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::types::{Oid, TypeInfo};
use crate::value::{PgValue, PgValueFormat};
use bytes::Buf;
use rbdc::Error;
use rbs::Value;
use std::borrow::Cow;

impl<T: Decode + TypeInfo> Decode for Vec<T> {
    fn decode(value: PgValue) -> Result<Self, Error> {
        let format = value.format();
        match format {
            PgValueFormat::Binary => {
                // https://github.com/postgres/postgres/blob/a995b371ae29de2d38c4b7881cf414b1560e9746/src/backend/utils/adt/arrayfuncs.c#L1548

                let mut buf = value.as_bytes()?;

                // number of dimensions in the array
                let ndim = buf.get_i32();

                if ndim == 0 {
                    // zero dimensions is an empty array
                    return Ok(Vec::new());
                }

                if ndim != 1 {
                    return Err(format!("encountered an array of {} dimensions; only one-dimensional arrays are supported", ndim).into());
                }

                // appears to have been used in the past to communicate potential NULLS
                // but reading source code back through our supported postgres versions (9.5+)
                // this is never used for anything
                let _flags = buf.get_i32();

                // the OID of the element
                let element_type_oid = Oid(buf.get_u32());
                let element_type_info: PgTypeInfo = PgTypeInfo::try_from_oid(element_type_oid)
                    .or_else(|| value.type_info.try_array_element().map(Cow::into_owned))
                    .ok_or_else(|| {
                        Error::from(format!(
                            "failed to resolve array element type for oid {}",
                            element_type_oid.0
                        ))
                    })?;

                // length of the array axis
                let len = buf.get_i32();

                // the lower bound, we only support arrays starting from "1"
                let lower = buf.get_i32();

                if lower != 1 {
                    return Err(format!("encountered an array with a lower bound of {} in the first dimension; only arrays starting at one are supported", lower).into());
                }

                let mut elements = Vec::with_capacity(len as usize);

                for _ in 0..len {
                    elements.push(T::decode(PgValue::get(
                        &mut buf,
                        format,
                        element_type_info.clone(),
                    ))?)
                }

                Ok(elements)
            }

            PgValueFormat::Text => {
                // no type is provided from the database for the element
                let mut element_type_info = PgTypeInfo::UNKNOWN;
                match value.type_info.kind() {
                    PgTypeKind::Simple => {}
                    PgTypeKind::Pseudo => {}
                    PgTypeKind::Domain(_) => {}
                    PgTypeKind::Composite(_) => {}
                    PgTypeKind::Array(item) => {
                        element_type_info = item.clone();
                    }
                    PgTypeKind::Enum(_) => {}
                    PgTypeKind::Range(_) => {}
                }

                let s = value.as_str()?;

                // https://github.com/postgres/postgres/blob/a995b371ae29de2d38c4b7881cf414b1560e9746/src/backend/utils/adt/arrayfuncs.c#L718

                // trim the wrapping braces
                let s = &s[1..(s.len() - 1)];

                if s.is_empty() {
                    // short-circuit empty arrays up here
                    return Ok(Vec::new());
                }

                // NOTE: Nearly *all* types use ',' as the sequence delimiter. Yes, there is one
                //       that does not. The BOX (not PostGIS) type uses ';' as a delimiter.

                // TODO: When we add support for BOX we need to figure out some way to make the
                //       delimiter selection

                let delimiter = ',';
                let mut done = false;
                let mut in_quotes = false;
                let mut in_escape = false;
                let mut value = String::with_capacity(10);
                let mut chars = s.chars();
                let mut elements = Vec::with_capacity(4);

                while !done {
                    loop {
                        match chars.next() {
                            Some(ch) => match ch {
                                _ if in_escape => {
                                    value.push(ch);
                                    in_escape = false;
                                }

                                '"' => {
                                    in_quotes = !in_quotes;
                                }

                                '\\' => {
                                    in_escape = true;
                                }

                                _ if ch == delimiter && !in_quotes => {
                                    break;
                                }

                                _ => {
                                    value.push(ch);
                                }
                            },

                            None => {
                                done = true;
                                break;
                            }
                        }
                    }

                    let value_opt = if value == "NULL" {
                        None
                    } else {
                        Some(value.as_bytes().to_vec())
                    };
                    elements.push(T::decode(PgValue {
                        value: value_opt,
                        type_info: element_type_info.clone(),
                        format,
                    })?);

                    value.clear();
                }

                Ok(elements)
            }
        }
    }
}

fn element_type_info<T: TypeInfo>(arg: &Vec<T>) -> PgTypeInfo {
    if arg.len() == 0 {
        PgTypeInfo::UNKNOWN
    } else {
        arg[0].type_info()
    }
}

impl Encode for Vec<Value> {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let type_info = element_type_info(&self);
        buf.extend(&1_i32.to_be_bytes()); // number of dimensions
        buf.extend(&0_i32.to_be_bytes()); // flags
                                          // element type
        match type_info.0 {
            PgType::DeclareWithName(name) => buf.patch_type_by_name(&name),
            ty => {
                buf.extend(&ty.oid().0.to_be_bytes());
            }
        }
        buf.extend(&(self.len() as i32).to_be_bytes()); // len
        buf.extend(&1_i32.to_be_bytes()); // lower bound
        for element in self {
            buf.encode(element)?;
        }
        Ok(IsNull::No)
    }
}
