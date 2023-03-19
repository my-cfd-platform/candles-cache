use std::collections::BTreeMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    CandleDateCache, CandleDateKey, CandleLoadModel, CandleModel, CandleResult, CandleType,
    RotateSettings,
};

pub struct CandlesTypesCache {
    pub candles: BTreeMap<u8, CandleDateCache>,
}

impl CandlesTypesCache {
    pub fn new(rotate_settings: RotateSettings) -> Self {
        Self {
            candles: BTreeMap::from([
                (
                    CandleType::Minute.to_u8(),
                    CandleDateCache::new(
                        CandleType::Minute,
                        rotate_settings.get_target(&CandleType::Minute),
                    ),
                ),
                (
                    CandleType::Hour.to_u8(),
                    CandleDateCache::new(
                        CandleType::Hour,
                        rotate_settings.get_target(&CandleType::Hour),
                    ),
                ),
                (
                    CandleType::Day.to_u8(),
                    CandleDateCache::new(
                        CandleType::Day,
                        rotate_settings.get_target(&CandleType::Day),
                    ),
                ),
                (
                    CandleType::Month.to_u8(),
                    CandleDateCache::new(
                        CandleType::Month,
                        rotate_settings.get_target(&CandleType::Month),
                    ),
                ),
            ]),
        }
    }

    pub fn load_candle(&mut self, candle: CandleLoadModel) {
        let Some(date_candle) = self.candles.get_mut(&(candle.candle_type.to_u8())) else{
            panic!("Invalid candle type")
        };

        date_candle.load(candle);
    }

    pub fn handle_new_price(
        &mut self,
        price: f64,
        price_date: DateTimeAsMicroseconds,
    ) -> Vec<CandleResult> {
        let mut result = vec![];
        for (_, candle_cache) in self.candles.iter_mut() {
            let to_return = candle_cache.handle_price(price, price_date);
            result.push(to_return)
        }

        return result;
    }

    pub fn get_in_date_range(
        &self,
        date_from: DateTimeAsMicroseconds,
        date_to: DateTimeAsMicroseconds,
        candle_type: CandleType,
    ) -> Vec<(CandleDateKey, CandleModel)> {
        let Some(candle_cache) = self.candles.get(&(candle_type.to_u8())) else{
            return Vec::new();
        };

        return candle_cache.get_in_date_range(date_from, date_to);
    }

    pub fn get_all_from_cache(&self) -> Vec<CandleResult> {
        let mut result = vec![];

        for (_, candle_cache) in &self.candles {
            let mut candles = candle_cache.get_all_from_cache();
            result.append(&mut candles)
        }

        return result;
    }

    pub fn get_candle(
        &self,
        date_key: CandleDateKey,
        candle_type: CandleType,
    ) -> Option<CandleModel> {
        if let Some(cache) = self.candles.get(&(candle_type.to_u8())) {
            return cache.get_candle(date_key);
        }

        println!(
            "Candle type not found: {:?} as {}u8",
            candle_type,
            candle_type.to_u8()
        );

        None
    }
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::CandleDateCache;

    #[test]
    fn test() {
        let mut cache = CandleDateCache::new(crate::CandleType::Day, None);

        let now = DateTimeAsMicroseconds::from_str("2015-01-01T12:12:12").unwrap();

        cache.handle_price(0.55, now);

        let mut from = now;
        let mut to = now;

        from.add_days(-1);

        to.add_days(1);

        let a = cache.get_in_date_range(from, to);

        let r = a.get(0).unwrap();

        assert_eq!(201501010000, r.0.get_value());

        let mut from = now;
        let mut to = now;

        from.add_days(1);

        to.add_days(2);

        let a = cache.get_in_date_range(from, to);

        assert_eq!(0, a.len())
    }
}
