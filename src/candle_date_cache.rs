use std::{collections::BTreeMap, time::Duration};

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{CandleData, CandleDateKey, CandleModel, CandleResult, CandleType, GetCandleDateKey};

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

    pub fn load(&mut self, candle_to_load: CandleModel) {
        let date_index = candle_to_load.get_candle_date_key();
        let model: CandleModel = candle_to_load.into();
        self.candles.insert(date_index, model);
    }

    pub fn get_in_date_range(
        &self,
        date_from: DateTimeAsMicroseconds,
        date_to: DateTimeAsMicroseconds,
    ) -> Vec<CandleModel> {
        println!(
            "Requesting candles from cache {}-{}. Candles has {} elements ",
            date_from.to_rfc3339(),
            date_to.to_rfc3339(),
            self.candles.len()
        );

        let mut candles = Vec::new();

        let date_from = date_from.into_candle_date_key(self.candle_type);
        let date_to = date_to.into_candle_date_key(self.candle_type);

        for (_, candle) in self.candles.range(date_from..date_to) {
            candles.push(candle.clone());
        }

        return candles;
    }

    pub fn get_all_from_cache(&self) -> Vec<CandleResult> {
        let mut result = vec![];

        for (date, candle) in &self.candles {
            result.push(CandleResult {
                date_key: *date,
                data: candle.data.clone(),
            });
        }

        return result;
    }

    pub fn handle_price(&mut self, price: f64, date_key: CandleDateKey) -> CandleData {
        self.rotate_candles();

        if let Some(candle) = self.candles.get_mut(&date_key) {
            candle.data.update_from_price(price, 0.0);

            return candle.data.clone();
        } else {
            let data = CandleData::new_from_price(price, 0.0);
            let candle = CandleModel {
                date_key,
                data: data.clone(),
            };
            self.candles.insert(date_key, candle.clone());

            return data;
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

    pub fn get_candle(&self, date_key: CandleDateKey) -> Option<CandleModel> {
        self.candles.get(&date_key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{CandleDateCache, GetCandleDateKey};

    #[test]
    fn test() {
        let mut cache = CandleDateCache::new(crate::CandleType::Day, None);

        let now = DateTimeAsMicroseconds::from_str("2015-01-01T12:12:12").unwrap();
        let date_key = now.into_candle_date_key(crate::CandleType::Day);

        cache.handle_price(0.55, date_key);

        let mut from = now;
        let mut to = now;

        from.add_days(-1);

        to.add_days(1);

        let a = cache.get_in_date_range(from, to);

        let r = a.get(0).unwrap();

        assert_eq!(201501010000, r.date_key.get_value());

        let mut from = now;
        let mut to = now;

        from.add_days(1);

        to.add_days(2);

        let a = cache.get_in_date_range(from, to);

        assert_eq!(0, a.len())
    }
}
