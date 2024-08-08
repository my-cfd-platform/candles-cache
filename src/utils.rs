use rust_extensions::date_time::{
    DateTimeAsMicroseconds, MICRO_SECONDS_IN_ONE_DAY, MICRO_SECONDS_IN_ONE_HOUR,
    MICRO_SECONDS_IN_ONE_MINUTE,
};

use crate::{CandleDateKey, CandleType};

pub fn get_candles_amount(from: CandleDateKey, to: CandleDateKey, candle_type: CandleType) -> i64 {
    let from = from.to_date_time_struct();
    let to = to.to_date_time_struct();
    match candle_type {
        CandleType::Minute => {
            let from: DateTimeAsMicroseconds = from.try_into().unwrap();
            let to: DateTimeAsMicroseconds = to.try_into().unwrap();
            return (to.unix_microseconds - from.unix_microseconds) / MICRO_SECONDS_IN_ONE_MINUTE;
        }
        CandleType::Hour => {
            let from: DateTimeAsMicroseconds = from.try_into().unwrap();
            let to: DateTimeAsMicroseconds = to.try_into().unwrap();
            return (to.unix_microseconds - from.unix_microseconds) / MICRO_SECONDS_IN_ONE_HOUR;
        }
        CandleType::Day => {
            let from: DateTimeAsMicroseconds = from.try_into().unwrap();
            let to: DateTimeAsMicroseconds = to.try_into().unwrap();
            return (to.unix_microseconds - from.unix_microseconds) / MICRO_SECONDS_IN_ONE_DAY;
        }
        CandleType::Month => {
            if from.year == to.year {
                return (to.month - from.month) as i64;
            }

            let result = if from.year < to.year {
                (to.year - from.year) * 12 + to.month as i32 - from.month as i32
            } else {
                0
            };

            result as i64
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_minutes_calculation() {
        let from = crate::CandleDateKey::new(202101010001);

        let to = crate::CandleDateKey::new(202101010002);

        let amount = super::get_candles_amount(from, to, crate::CandleType::Minute);

        assert_eq!(amount, 1);

        let from = crate::CandleDateKey::new(202101010000);

        let to = crate::CandleDateKey::new(202101010100);

        let amount = super::get_candles_amount(from, to, crate::CandleType::Minute);

        assert_eq!(amount, 60);
    }

    #[test]
    fn test_hours_calculation() {
        let from = crate::CandleDateKey::new(202101010000);

        let to = crate::CandleDateKey::new(202101010100);

        let amount = super::get_candles_amount(from, to, crate::CandleType::Hour);

        assert_eq!(amount, 1);

        let from = crate::CandleDateKey::new(202101010000);

        let to = crate::CandleDateKey::new(202101020000);

        let amount = super::get_candles_amount(from, to, crate::CandleType::Hour);

        assert_eq!(amount, 24);
    }

    #[test]
    fn test_days_calculation() {
        let from = crate::CandleDateKey::new(202101010000);

        let to = crate::CandleDateKey::new(202101020000);

        let amount = super::get_candles_amount(from, to, crate::CandleType::Day);

        assert_eq!(amount, 1);

        let from = crate::CandleDateKey::new(202101010000);

        let to = crate::CandleDateKey::new(202101310000);

        let amount = super::get_candles_amount(from, to, crate::CandleType::Day);

        assert_eq!(amount, 30);
    }
}
