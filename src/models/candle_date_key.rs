use rust_extensions::date_time::{DateTimeAsMicroseconds, DateTimeStruct, TimeStruct};

use crate::CandleType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, core::hash::Hash, PartialOrd, Ord)]
pub struct CandleDateKey(u64);

impl CandleDateKey {
    pub fn new(date: u64) -> Self {
        Self(date)
    }

    pub fn get_value(&self) -> u64 {
        self.0
    }

    pub fn as_ref(&self) -> &u64 {
        &self.0
    }

    pub fn to_date_time_struct(&self) -> DateTimeStruct {
        let value = self.0;
        let year = value / 100000000;

        let value = value - year * 100000000;

        let month = value / 1000000;

        if month > 12 {
            panic!("Invalid month {}", month);
        }

        let value = value - month * 1000000;

        let day = value / 10000;

        if day > 31 {
            panic!("Invalid day {}", day);
        }

        let value = value - day * 10000;

        let hour = value / 100;

        if hour > 23 {
            panic!("Invalid hour {}", hour);
        }

        let minute = value - hour * 100;

        if minute > 59 {
            panic!("Invalid minute {}", minute);
        }

        DateTimeStruct {
            year: year as i32,
            month: month as u32,
            day: day as u32,
            time: TimeStruct {
                hour: hour as u32,
                min: minute as u32,
                sec: 0,
                micros: 0,
            },
            dow: None,
        }
    }

    pub fn get_next_period_date_key(&self, candle_type: CandleType) -> CandleDateKey {
        match candle_type {
            CandleType::Minute => {
                let mut dt: DateTimeAsMicroseconds = self.into();
                dt.add_minutes(1);
                return dt.into_candle_date_key(candle_type);
            }
            CandleType::Hour => {
                let mut dt: DateTimeAsMicroseconds = self.into();
                dt.add_hours(1);
                return dt.into_candle_date_key(candle_type);
            }
            CandleType::Day => {
                let mut dt: DateTimeAsMicroseconds = self.into();
                dt.add_days(1);
                return dt.into_candle_date_key(candle_type);
            }
            CandleType::Month => {
                let dt: DateTimeAsMicroseconds = self.into();
                let mut dt: DateTimeStruct = dt.into();
                dt.inc_month();
                let dt = dt.to_date_time_as_microseconds().unwrap();
                return dt.into_candle_date_key(candle_type);
            }
        }
    }

    pub fn get_prev_period_date_key(&self, candle_type: CandleType) -> CandleDateKey {
        match candle_type {
            CandleType::Minute => {
                let mut dt: DateTimeAsMicroseconds = self.into();
                dt.add_minutes(-1);
                return dt.into_candle_date_key(candle_type);
            }
            CandleType::Hour => {
                let mut dt: DateTimeAsMicroseconds = self.into();
                dt.add_hours(-1);
                return dt.into_candle_date_key(candle_type);
            }
            CandleType::Day => {
                let mut dt: DateTimeAsMicroseconds = self.into();
                dt.add_days(-1);
                return dt.into_candle_date_key(candle_type);
            }
            CandleType::Month => {
                let dt: DateTimeAsMicroseconds = self.into();
                let mut dt: DateTimeStruct = dt.into();
                dt.dec_month();
                let dt = dt.to_date_time_as_microseconds().unwrap();
                return dt.into_candle_date_key(candle_type);
            }
        }
    }
}

impl Into<CandleDateKey> for u64 {
    fn into(self) -> CandleDateKey {
        CandleDateKey::new(self)
    }
}

impl Into<CandleDateKey> for i64 {
    fn into(self) -> CandleDateKey {
        CandleDateKey::new(self as u64)
    }
}

pub trait GetCandleDateKey {
    fn into_candle_date_key(&self, candle_type: CandleType) -> CandleDateKey;
}

impl GetCandleDateKey for DateTimeAsMicroseconds {
    fn into_candle_date_key(&self, candle_type: CandleType) -> CandleDateKey {
        let format = match candle_type {
            CandleType::Minute => "%Y%m%d%H%M",
            CandleType::Hour => "%Y%m%d%H",
            CandleType::Day => "%Y%m%d",
            CandleType::Month => "%Y%m",
        };

        let date = format!("{:0<12}", self.to_chrono_utc().format(format));

        return CandleDateKey::new(date.parse().unwrap());
    }
}

impl Into<DateTimeAsMicroseconds> for &CandleDateKey {
    fn into(self) -> DateTimeAsMicroseconds {
        from_key_to_date_time(*self)
    }
}

impl Into<DateTimeAsMicroseconds> for CandleDateKey {
    fn into(self) -> DateTimeAsMicroseconds {
        from_key_to_date_time(self)
    }
}

fn from_key_to_date_time(key: CandleDateKey) -> DateTimeAsMicroseconds {
    let mut c = key.to_date_time_struct();

    if c.day == 0 {
        c.day = 1;
    }

    match c.try_into() {
        Ok(v) => v,
        Err(e) => panic!("Invalid date key: {}", e),
    }
}
#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{CandleType, GetCandleDateKey};

    use super::CandleDateKey;

    #[test]
    fn test() {
        let date_time =
            DateTimeAsMicroseconds::parse_iso_string("2021-01-01T10:22:33.000000Z").unwrap();

        let d = date_time.into_candle_date_key(CandleType::Minute);

        assert_eq!(202101011022, d.get_value());

        let d = date_time.into_candle_date_key(CandleType::Hour);

        assert_eq!(202101011000, d.get_value());

        let d = date_time.into_candle_date_key(CandleType::Day);

        assert_eq!(202101010000, d.get_value());

        let d = date_time.into_candle_date_key(CandleType::Month);

        assert_eq!(202101000000, d.get_value());
    }

    #[test]
    fn test_into_date_time() {
        let date_time =
            DateTimeAsMicroseconds::parse_iso_string("2021-01-01T10:22:33.000000Z").unwrap();

        let db_key = date_time.into_candle_date_key(CandleType::Minute);

        let result: DateTimeAsMicroseconds = db_key.into();

        assert_eq!("2021-01-01T10:22:00", &result.to_rfc3339()[..19])
    }

    #[test]
    fn test_round_to_hour() {
        let date_time =
            DateTimeAsMicroseconds::parse_iso_string("2021-01-01T10:22:33.000000Z").unwrap();

        let db_key = date_time.into_candle_date_key(CandleType::Hour);

        let result: DateTimeAsMicroseconds = db_key.into();

        assert_eq!("2021-01-01T10:00:00", &result.to_rfc3339()[..19])
    }

    #[test]
    fn test_round_to_day() {
        let date_time =
            DateTimeAsMicroseconds::parse_iso_string("2021-01-01T10:22:33.000000Z").unwrap();

        let db_key = date_time.into_candle_date_key(CandleType::Day);

        let result: DateTimeAsMicroseconds = db_key.into();

        assert_eq!("2021-01-01T00:00:00", &result.to_rfc3339()[..19])
    }

    #[test]
    fn test_round_to_month() {
        let date_time =
            DateTimeAsMicroseconds::parse_iso_string("2021-01-01T10:22:33.000000Z").unwrap();

        let db_key = date_time.into_candle_date_key(CandleType::Month);

        println!("{:?}", db_key);

        let result: DateTimeAsMicroseconds = db_key.into();

        assert_eq!("2021-01-01T00:00:00", &result.to_rfc3339()[..19]);
    }

    #[test]
    fn test_month_get_next_candle_key() {
        let month_key = CandleDateKey::new(202101000000);
        let next_month_key = month_key.get_next_period_date_key(CandleType::Month);
        assert_eq!(202102000000, next_month_key.get_value());

        let month_key = CandleDateKey::new(202102000000);
        let next_month_key = month_key.get_next_period_date_key(CandleType::Month);
        assert_eq!(202103000000, next_month_key.get_value());

        let month_key = CandleDateKey::new(202103000000);
        let next_month_key = month_key.get_next_period_date_key(CandleType::Month);
        assert_eq!(202104000000, next_month_key.get_value());

        let month_key = CandleDateKey::new(202112000000);
        let next_month_key = month_key.get_next_period_date_key(CandleType::Month);
        assert_eq!(202201000000, next_month_key.get_value());
    }

    #[test]
    fn test_day_get_next_candle_key() {
        let key = CandleDateKey::new(202101010000);
        let next_key = key.get_next_period_date_key(CandleType::Day);
        assert_eq!(202101020000, next_key.get_value());

        let key: CandleDateKey = CandleDateKey::new(202112310000);
        let next_key = key.get_next_period_date_key(CandleType::Day);
        assert_eq!(202201010000, next_key.get_value());
    }
}
