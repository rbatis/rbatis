use chrono::{NaiveDateTime, NaiveDate};

pub trait Now {
    fn now() -> Self;
}

impl Now for NaiveDateTime {
    fn now() -> Self {
        //TODO
        unimplemented!()
    }
}
