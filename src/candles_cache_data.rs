use std::{collections::BTreeMap, time::Duration};

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    CandleDateKey, CandleLoadModel, CandleModel, CandleType, GetCandleDateKey, RotateSettings,
};

#[derive(Debug, Clone)]
pub struct CandleResult {
    pub date: CandleDateKey,
    pub candles_type: CandleType,
    pub candle: CandleModel,
}

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
}

pub struct CandleDateCache {
    pub candles: BTreeMap<CandleDateKey, CandleModel>,
    pub candle_type: CandleType,
    pub rotate_period: Option<Duration>,
}

impl CandleDateCache {
    pub fn new(candle_type: CandleType, rotate_period: Option<Duration>) -> Self {
        Self {
            candles: BTreeMap::new(),
            candle_type,
            rotate_period,
        }
    }

    pub fn load(&mut self, candle_to_load: CandleLoadModel) {
        let date_index = candle_to_load.get_candle_date_key();
        let model: CandleModel = candle_to_load.into();
        self.candles.insert(date_index, model);
    }

    pub fn get_in_date_range(
        &self,
        date_from: DateTimeAsMicroseconds,
        date_to: DateTimeAsMicroseconds,
    ) -> Vec<(CandleDateKey, CandleModel)> {
        println!(
            "Requesting candles from cache {}-{}. Candles has {} elements ",
            date_from.to_rfc3339(),
            date_to.to_rfc3339(),
            self.candles.len()
        );

        let mut candles = Vec::new();

        let date_from = date_from.into_candle_date_key(self.candle_type);
        let date_to = date_to.into_candle_date_key(self.candle_type);

        for (date, candle) in self.candles.range(date_from..date_to) {
            candles.push((date.to_owned(), candle.clone()));
        }

        return candles;
    }

    pub fn get_all_from_cache(&self) -> Vec<CandleResult> {
        let mut result = vec![];

        for (date, candle) in &self.candles {
            result.push(CandleResult {
                date: *date,
                candles_type: self.candle_type,
                candle: candle.clone(),
            });
        }

        return result;
    }

    pub fn handle_price(&mut self, price: f64, price_date: DateTimeAsMicroseconds) -> CandleResult {
        let date: CandleDateKey = price_date.into_candle_date_key(self.candle_type);

        self.rotate_candles();

        if let Some(candle) = self.candles.get_mut(&date) {
            candle.update_from_price(price, 0.0);

            return CandleResult {
                date,
                candles_type: self.candle_type,
                candle: candle.clone(),
            };
        } else {
            let candle = CandleModel::new_from_price(price, 0.0);
            self.candles.insert(date, candle.clone());

            return CandleResult {
                date,
                candles_type: self.candle_type,
                candle: candle.clone(),
            };
        }
    }

    fn rotate_candles(&mut self) {
        if let Some(ids_to_remove) = self.get_candles_ids_to_rotate() {
            for date in ids_to_remove {
                self.candles.remove(&date);
            }
        }
    }

    fn get_candles_ids_to_rotate(&self) -> Option<Vec<CandleDateKey>> {
        let Some(cache_load_duration) = self.rotate_period else{
            return None;
        };

        let mut max_possible_date = DateTimeAsMicroseconds::now();
        max_possible_date.add(cache_load_duration);

        let key_date = max_possible_date.into_candle_date_key(self.candle_type);
        return Some(
            self.candles
                .range(..key_date)
                .map(|(date, _)| *date)
                .collect(),
        );
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
