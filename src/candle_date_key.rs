use rust_extensions::date_time::DateTimeAsMicroseconds;

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

impl Into<DateTimeAsMicroseconds> for CandleDateKey {
    fn into(self) -> DateTimeAsMicroseconds {
        let value = self.get_value();

        let year = value / 100000000;

        let value = value - year * 100000000;

        let month = value / 1000000;

        let value = value - month * 1000000;

        let day = value / 10000;

        let value = value - day * 10000;

        let hour = value / 100;

        let value = value - hour * 100;

        DateTimeAsMicroseconds::create(
            year as i32,
            month as u32,
            day as u32,
            hour as u32,
            value as u32,
            0,
            0,
        )
    }
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
}
