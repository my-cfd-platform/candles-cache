use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::CandleType;

use super::candle_date_key_utils::DateTimeComponents;

#[derive(Debug, Clone, Copy, PartialEq, Eq, core::hash::Hash, PartialOrd, Ord)]
pub struct CandleDateKey(u64);

impl CandleDateKey {
    pub fn new(date: u64) -> Self {
        Self(date)
    }

    pub fn get_value(&self) -> u64 {
        self.0
    }

    pub fn as_unix_seconds(&self) -> i64 {
        let dt: DateTimeAsMicroseconds = self.into();
        dt.unix_microseconds / 1000000
    }

    pub fn as_unix_milliseconds(&self) -> i64 {
        let dt: DateTimeAsMicroseconds = self.into();
        dt.unix_microseconds / 1000
    }

    pub fn as_unix_microseconds(&self) -> i64 {
        let dt: DateTimeAsMicroseconds = self.into();
        dt.unix_microseconds
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

        return CandleDateKey::new(date.to_string().parse().unwrap());
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
    let c = DateTimeComponents::from_date_key(key);

    DateTimeAsMicroseconds::create(
        c.year as i32,
        c.month as u32,
        if c.day == 0 { 1 } else { c.day as u32 },
        c.hour as u32,
        c.minute as u32,
        0,
        0,
    )
}
#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{CandleType, GetCandleDateKey};

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

        let result: DateTimeAsMicroseconds = db_key.into();

        assert_eq!("2021-01-01T00:00:00", &result.to_rfc3339()[..19])
    }
}
