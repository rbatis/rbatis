use std::ops::Range;

use bytes::Bytes;

#[derive(Debug)]
pub struct Row {
    pub storage: Bytes,
    pub values: Vec<Option<Range<usize>>>,
}

impl Row {
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        self.values[index]
            .as_ref()
            .map(|col| &self.storage[(col.start as usize)..(col.end as usize)])
    }
}
