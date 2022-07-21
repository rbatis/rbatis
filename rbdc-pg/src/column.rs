use crate::type_info::PgTypeInfo;
use rbdc::ext::ustr::UStr;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PgColumn {
    pub(crate) ordinal: usize,
    pub(crate) name: UStr,
    pub(crate) type_info: PgTypeInfo,
    #[serde(skip)]
    pub(crate) relation_id: Option<i32>,
    #[serde(skip)]
    pub(crate) relation_attribute_no: Option<i16>,
}

impl PgColumn {
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }

    pub fn name(&self) -> &str {
        &*self.name
    }

    pub fn type_info(&self) -> &PgTypeInfo {
        &self.type_info
    }
}
