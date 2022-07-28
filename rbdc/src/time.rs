#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ParseDate {
    pub year: u16,
    pub mon: u8,
    pub day: u8,
}
#[derive(Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ParseTime {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub ms: u32,
}

pub fn parse_year(arg: &str) -> u16 {
    arg.parse().unwrap_or_default()
}
/// RFC3339 Date = "1993-02-06"
pub fn parse_date(arg: &str) -> ParseDate {
    if arg.len() < 10 {
        return ParseDate {
            year: 0,
            mon: 0,
            day: 0,
        };
    }
    let year: u16 = arg[0..4].parse().unwrap_or_default();
    let mon: u8 = arg[5..7].parse().unwrap_or_default();
    let day: u8 = arg[8..10].parse().unwrap_or_default();
    return ParseDate { year, mon, day };
}
/// RFC3339Ms Time = "15:04:05.999999"
pub fn parse_time(arg: &str) -> ParseTime {
    if arg.len() < 8 {
        return ParseTime {
            hour: 0,
            min: 0,
            sec: 0,
            ms: 0,
        };
    }
    let hour: u8 = arg[0..2].parse().unwrap_or_default();
    let min: u8 = arg[4..5].parse().unwrap_or_default();
    let sec: u8 = arg[6..8].parse().unwrap_or_default();
    let ms: u32 = {
        if arg.len() > 9 {
            arg[9..arg.len()].parse().unwrap_or_default()
        } else {
            0
        }
    };
    return ParseTime { hour, min, sec, ms };
}

/// RFC3339Nano = "2006-01-02 15:04:05.999999999"
pub fn parse_date_time(arg: &str) -> (ParseDate, ParseTime) {
    let date = parse_date(&arg[0..10]);
    let mut time = ParseTime {
        hour: 0,
        min: 0,
        sec: 0,
        ms: 0,
    };
    if arg.len() >= 19 {
        time = parse_time(&arg[11..arg.len()]);
    }
    return (date, time);
}

#[cfg(test)]
mod test {
    use crate::time::{parse_date, parse_date_time, parse_time, ParseDate, ParseTime};

    #[test]
    fn test_parse() {
        assert_eq!(
            ParseDate {
                year: 2022,
                mon: 12,
                day: 12
            },
            parse_date("2022-12-12")
        );
        assert_eq!(
            ParseTime {
                hour: 15,
                min: 04,
                sec: 05,
                ms: 999999
            },
            parse_time("15:04:05.999999")
        );
        assert_eq!(
            (
                ParseDate {
                    year: 2022,
                    mon: 11,
                    day: 12
                },
                ParseTime {
                    hour: 15,
                    min: 04,
                    sec: 05,
                    ms: 999999
                }
            ),
            parse_date_time("2022-11-12 15:04:05.999999")
        );
    }
}
