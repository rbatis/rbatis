use chrono::{Local, NaiveDate, NaiveDateTime};

pub trait DateTimeNow {
    fn now() -> NaiveDateTime;
}

impl DateTimeNow for NaiveDateTime {
    fn now() -> NaiveDateTime {
        let dt = Local::now();
        dt.naive_local()
    }
}