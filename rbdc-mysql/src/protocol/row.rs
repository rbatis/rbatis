use std::ops::Range;

#[derive(Debug)]
pub struct Row {
    pub storage: Vec<Option<Vec<u8>>>,
    pub values: Vec<Option<Range<usize>>>,
}

impl From<(Vec<Option<Range<usize>>>, Vec<u8>)> for Row {
    fn from((ranges, data): (Vec<Option<Range<usize>>>, Vec<u8>)) -> Self {
        let mut row = Row {
            storage: Vec::with_capacity(ranges.len()),
            values: Vec::with_capacity(ranges.len()),
        };
        for x in ranges {
            if let Some(col) = x {
                row.storage.push(Some(data[col.start..col.end].to_vec()));
                row.values.push(Some(col));
            } else {
                row.storage.push(None);
                row.values.push(None);
            }
        }
        row
    }
}

impl Row {
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        let mut idx = 0;
        for x in &self.values {
            if index == idx {
                match x {
                    None => return None,
                    Some(_) => match &self.storage[idx] {
                        None => {
                            return None;
                        }
                        Some(v) => {
                            return Some(v);
                        }
                    },
                }
            }
            idx += 1;
        }
        None
    }

    pub fn take(&mut self, index: usize) -> Option<Vec<u8>> {
        let mut idx = 0;
        for x in &self.values {
            if index == idx {
                match x {
                    None => return None,
                    Some(_) => {
                        return match self.storage[idx].take() {
                            None => None,
                            Some(v) => Some(v),
                        }
                    }
                }
            }
            idx += 1;
        }
        None
    }
}
