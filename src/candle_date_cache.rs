use std::{collections::BTreeMap, time::Duration};

use rust_extensions::{date_time::DateTimeAsMicroseconds, lazy::LazyVec};

use crate::{CandleData, CandleDateKey, CandleModel, CandleType, GetCandleDateKey};

pub struct CandleDateCache {
    pub candles: BTreeMap<u64, CandleModel>,
    pub candle_type: CandleType,
}

impl CandleDateCache {
    pub fn new(candle_type: CandleType) -> Self {
        Self {
            candles: BTreeMap::new(),
            candle_type,
        }
    }

    pub fn insert_or_update(&mut self, candle_to_load: CandleModel) {
        let date_index = candle_to_load.get_candle_date_key();
        let model: CandleModel = candle_to_load.into();
        self.candles.insert(date_index.get_value(), model);
    }

    pub fn get_in_date_range(
        &self,
        date_from: DateTimeAsMicroseconds,
        date_to: DateTimeAsMicroseconds,
    ) -> Vec<CandleModel> {
        let mut candles = Vec::new();

        let date_from = date_from.into_candle_date_key(self.candle_type);
        let date_to = date_to.into_candle_date_key(self.candle_type);

        for (_, candle) in self
            .candles
            .range(date_from.get_value()..=date_to.get_value())
        {
            candles.push(candle.clone());
        }

        return candles;
    }

    pub fn get_all_from_cache(&self) -> Vec<CandleModel> {
        let mut result = vec![];

        for (date, candle) in &self.candles {
            result.push(CandleModel {
                date_key: CandleDateKey::new(*date),
                data: candle.data.clone(),
            });
        }

        return result;
    }

    pub fn handle_price(&mut self, price: f64, date_key: CandleDateKey) -> CandleData {
        if let Some(candle) = self.candles.get_mut(&date_key.get_value()) {
            candle.data.update_from_price(price, 0.0);
            return candle.data.clone();
        } else {
            let data = CandleData::new_from_price(price, 0.0);
            let candle = CandleModel {
                date_key,
                data: data.clone(),
            };
            self.candles.insert(date_key.get_value(), candle.clone());
            return data;
        }
    }

    pub fn gc_candles(&mut self, rotation_period: Duration) -> Option<Vec<CandleModel>> {
        let mut removed_candles = LazyVec::new();
        if let Some(ids_to_remove) = self.get_candles_ids_to_rotate(rotation_period) {
            println!(
                "Rotating {} candles for type: {:?}",
                ids_to_remove.len(),
                self.candle_type
            );
            for date in ids_to_remove {
                let removed_candle = self.candles.remove(&date);

                if let Some(removed_candle) = removed_candle {
                    removed_candles.add(removed_candle);
                }
            }
        }

        removed_candles.get_result()
    }

    fn get_candles_ids_to_rotate(&self, rotation_period: Duration) -> Option<Vec<u64>> {
        let max_possible_date = DateTimeAsMicroseconds::now().sub(rotation_period);

        let key_date = max_possible_date.into_candle_date_key(self.candle_type);

        let mut result = LazyVec::new();

        for date in self.candles.keys() {
            if *date < key_date.get_value() {
                result.add(date.clone())
            } else {
                break;
            }
        }

        result.get_result()
    }

    pub fn get_candle(&self, date_key: CandleDateKey) -> Option<CandleModel> {
        self.candles.get(&date_key.get_value()).cloned()
    }
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{CandleDateCache, GetCandleDateKey};

    #[test]
    fn test() {
        let mut cache = CandleDateCache::new(crate::CandleType::Day);

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
