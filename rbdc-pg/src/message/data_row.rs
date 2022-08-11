use std::ops::Range;

use byteorder::{BigEndian, ByteOrder};
use bytes::Bytes;
use rbdc::io::Decode;
use rbdc::Error;

/// A row of data from the database.
#[derive(Debug)]
pub struct DataRow {
    pub storage: Vec<Option<Vec<u8>>>,
    /// Ranges into the stored row data.
    /// This uses `u32` instead of usize to reduce the size of this type. Values cannot be larger
    /// than `i32` in postgres.
    pub values: Vec<Option<Range<usize>>>,
}

impl DataRow {
    #[inline]
    pub(crate) fn get(&self, index: usize) -> Option<&'_ [u8]> {
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

    #[inline]
    pub(crate) fn take(&mut self, index: usize) -> Option<Vec<u8>> {
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

impl Decode<'_> for DataRow {
    fn decode_with(buf: Bytes, _: ()) -> Result<Self, Error> {
        let cnt = BigEndian::read_u16(&buf) as usize;

        let mut values = Vec::with_capacity(cnt);
        let mut offset = 2;

        for _ in 0..cnt {
            // Length of the column value, in bytes (this count does not include itself).
            // Can be zero. As a special case, -1 indicates a NULL column value.
            // No value bytes follow in the NULL case.
            let length = BigEndian::read_i32(&buf[(offset as usize)..]);
            offset += 4;

            if length < 0 {
                values.push(None);
            } else {
                values.push(Some(offset as usize..(offset + length as u32) as usize));
                offset += length as u32;
            }
        }
        let mut storage = Vec::with_capacity(values.len());
        for x in &values {
            match x {
                None => {
                    storage.push(None);
                }
                Some(v) => storage.push(Some(buf[v.start..v.end].to_vec())),
            }
        }
        Ok(Self {
            storage: storage,
            values: values,
        })
    }
}

#[test]
fn test_decode_data_row() {
    const DATA: &[u8] = b"\x00\x08\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\n\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\x14\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00(\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00P";

    let row = DataRow::decode(DATA.into()).unwrap();

    assert_eq!(row.values.len(), 8);

    assert!(row.get(0).is_none());
    assert_eq!(row.get(1).unwrap(), &[0_u8, 0, 0, 10][..]);
    assert!(row.get(2).is_none());
    assert_eq!(row.get(3).unwrap(), &[0_u8, 0, 0, 20][..]);
    assert!(row.get(4).is_none());
    assert_eq!(row.get(5).unwrap(), &[0_u8, 0, 0, 40][..]);
    assert!(row.get(6).is_none());
    assert_eq!(row.get(7).unwrap(), &[0_u8, 0, 0, 80][..]);
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_data_row_get(b: &mut test::Bencher) {
    const DATA: &[u8] = b"\x00\x08\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\n\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\x14\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00(\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00P";

    let row = DataRow::decode(test::black_box(Bytes::from_static(DATA))).unwrap();

    b.iter(|| {
        let _value = test::black_box(&row).get(3);
    });
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_decode_data_row(b: &mut test::Bencher) {
    const DATA: &[u8] = b"\x00\x08\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\n\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00\x14\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00(\xff\xff\xff\xff\x00\x00\x00\x04\x00\x00\x00P";

    b.iter(|| {
        let _ = DataRow::decode(test::black_box(Bytes::from_static(DATA)));
    });
}
