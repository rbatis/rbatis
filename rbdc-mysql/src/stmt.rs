use crate::protocol::text::ColumnType;
use crate::result_set::{MySqlColumn, MySqlTypeInfo};
use crate::types::{Encode, TypeInfo};
use rbdc::ext::ustr::UStr;
use rbs::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct MySqlStatement {
    pub sql: String,
    pub metadata: MySqlStatementMetadata,
}

#[derive(Debug, Default, Clone)]
pub struct MySqlStatementMetadata {
    pub(crate) columns: Arc<Vec<MySqlColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, (usize, MySqlTypeInfo)>>,
    pub(crate) parameters: usize,
}

/// Implementation of [`Arguments`] for MySQL.
#[derive(Debug, Default, Clone)]
pub struct MySqlArguments {
    pub values: Vec<u8>,
    pub types: Vec<MySqlTypeInfo>,
    pub null_bitmap: Vec<u8>,
}

impl MySqlArguments {
    pub fn add(&mut self, arg: Value) {
        let index = self.types.len();
        let ty = arg.type_info();
        arg.encode(&mut self.values).unwrap();
        let is_null = ty.r#type.eq(&ColumnType::Null);
        self.types.push(ty);
        self.null_bitmap.resize((index / 8) + 1, 0);
        if is_null {
            self.null_bitmap[index / 8] |= (1 << (index % 8)) as u8;
        }
    }

    #[doc(hidden)]
    pub fn len(&self) -> usize {
        self.types.len()
    }
}

impl From<Vec<Value>> for MySqlArguments {
    fn from(args: Vec<Value>) -> Self {
        let mut arg = MySqlArguments::default();
        for x in args {
            arg.add(x);
        }
        arg
    }
}
