use crate::CandleDateKey;

#[derive(Debug, Clone, Copy)]
pub enum CandleType {
    Minute = 0,
    Hour = 1,
    Day = 2,
    Month = 3,
}

impl CandleType {
    pub const ALL_CANDLE_TYPES: [Self; 4] = [Self::Minute, Self::Hour, Self::Day, Self::Month];

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Minute,
            1 => Self::Hour,
            2 => Self::Day,
            3 => Self::Month,
            _ => panic!("Invalid candle type {}", value),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            CandleType::Minute => 0u8,
            CandleType::Hour => 1u8,
            CandleType::Day => 2u8,
            CandleType::Month => 3u8,
        }
    }

    pub fn verify_date_key(&self, candle_date_key: CandleDateKey) -> Result<(), String> {
        let c = candle_date_key.to_date_time_struct();
        match self {
            CandleType::Minute => super::candle_date_key_utils::good_as_minute_key(&c),
            CandleType::Hour => super::candle_date_key_utils::good_as_hour_key(&c),
            CandleType::Day => super::candle_date_key_utils::good_as_day_key(&c),
            CandleType::Month => super::candle_date_key_utils::good_as_month_key(&c),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CandleDateKey, CandleType};

    #[test]
    fn tests() {
        let src = 0;
        let ct = CandleType::from_u8(src);
        assert_eq!(ct.to_u8(), src);
    }

    #[test]
    fn test_date_key_verification_as_minute() {
        let src = 202301011201;
        let candle_date_key = CandleDateKey::new(src);

        assert!(CandleType::Minute.verify_date_key(candle_date_key).is_ok());
        assert!(CandleType::Hour.verify_date_key(candle_date_key).is_err());
        assert!(CandleType::Day.verify_date_key(candle_date_key).is_err());
        assert!(CandleType::Month.verify_date_key(candle_date_key).is_err());
    }

    #[test]
    fn test_date_key_verification_as_hour() {
        let src = 202301011200;
        let candle_date_key = CandleDateKey::new(src);

        assert!(CandleType::Minute.verify_date_key(candle_date_key).is_ok());
        assert!(CandleType::Hour.verify_date_key(candle_date_key).is_ok());
        assert!(CandleType::Day.verify_date_key(candle_date_key).is_err());
        assert!(CandleType::Month.verify_date_key(candle_date_key).is_err());
    }

    #[test]
    fn test_date_key_verification_as_day() {
        let src = 202301010000;
        let candle_date_key = CandleDateKey::new(src);

        assert!(CandleType::Minute.verify_date_key(candle_date_key).is_ok());
        assert!(CandleType::Hour.verify_date_key(candle_date_key).is_ok());
        assert!(CandleType::Day.verify_date_key(candle_date_key).is_ok());
        assert!(CandleType::Month.verify_date_key(candle_date_key).is_err());
    }

    #[test]
    fn test_date_key_verification_as_month() {
        let src = 202301000000;
        let candle_date_key = CandleDateKey::new(src);

        assert!(CandleType::Minute.verify_date_key(candle_date_key).is_err());
        assert!(CandleType::Hour.verify_date_key(candle_date_key).is_err());
        assert!(CandleType::Day.verify_date_key(candle_date_key).is_err());
        assert!(CandleType::Month.verify_date_key(candle_date_key).is_ok());

        println!("{:?}", CandleType::ALL_CANDLE_TYPES);
    }
}
