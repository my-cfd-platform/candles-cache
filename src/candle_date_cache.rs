use std::time::Duration;

use rust_extensions::{date_time::DateTimeAsMicroseconds, lazy::LazyVec, sorted_vec::*};

use crate::{CandleData, CandleDateKey, CandleModel, CandleType, GetCandleDateKey};

pub struct CandleDateCache {
    pub candles: SortedVec<u64, CandleModel>,
    pub candle_type: CandleType,
}

impl CandleDateCache {
    pub fn new(candle_type: CandleType) -> Self {
        Self {
            candles: SortedVec::new(),
            candle_type,
        }
    }

    pub fn insert_or_update(&mut self, candle_to_load: CandleModel) {
        let model: CandleModel = candle_to_load.into();
        match self
            .candles
            .insert_or_update(model.get_candle_date_key().as_ref())
        {
            InsertOrUpdateEntry::Insert(entry) => entry.insert(model),
            InsertOrUpdateEntry::Update(entry) => entry.item.data = model.data,
        }
    }

    pub fn get_in_date_range(&self, from: CandleDateKey, to: CandleDateKey) -> &[CandleModel] {
        self.candles.range(from.get_value()..to.get_value())
    }

    pub fn get_all_from_cache(&self) -> Vec<CandleModel> {
        self.candles.as_slice().to_vec()
        /*
        let mut result = vec![];

        for (date, candle) in &self.candles {
            result.push(CandleModel {
                date_key: CandleDateKey::new(*date),
                data: candle.data.clone(),
            });
        }

        return result;
         */
    }

    pub fn handle_price(&mut self, price: f64, date_key: CandleDateKey) -> CandleData {
        match self.candles.insert_or_update(date_key.as_ref()) {
            InsertOrUpdateEntry::Insert(entry) => {
                let data = CandleData::new_from_price(price, 0.0);
                let candle = CandleModel {
                    date_key,
                    data: data.clone(),
                };

                let result = candle.data.clone();

                entry.insert(candle.clone());

                return result;
            }

            InsertOrUpdateEntry::Update(entry) => {
                entry.item.data.update_from_price(price, 0.0);
                return entry.item.data.clone();
            }
        }
    }

    pub fn gc_candles(
        &mut self,
        now: DateTimeAsMicroseconds,
        rotation_period: Duration,
    ) -> Option<Vec<CandleModel>> {
        let mut removed_candles = LazyVec::new();
        if let Some(ids_to_remove) = self.get_candles_ids_to_rotate(now, rotation_period) {
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

    fn get_candles_ids_to_rotate(
        &self,
        now: DateTimeAsMicroseconds,
        rotation_period: Duration,
    ) -> Option<Vec<u64>> {
        let max_possible_date = now.sub(rotation_period);

        let key_date = max_possible_date.into_candle_date_key(self.candle_type);

        let mut result = LazyVec::new();

        for candle in self.candles.iter() {
            let candle_key = candle.date_key.get_value();
            if candle_key < key_date.get_value() {
                result.add(candle_key)
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
        let candle_type = crate::CandleType::Day;
        let mut cache = CandleDateCache::new(candle_type);

        let now = DateTimeAsMicroseconds::from_str("2015-01-01T12:12:12").unwrap();
        let date_key = now.into_candle_date_key(candle_type);

        cache.handle_price(0.55, date_key);

        let mut from = now;
        let mut to = now;

        from.add_days(-1);

        to.add_days(1);

        let a = cache.get_in_date_range(
            from.into_candle_date_key(candle_type),
            to.into_candle_date_key(candle_type),
        );

        let r = a.get(0).unwrap();

        assert_eq!(201501010000, r.date_key.get_value());

        let mut from = now;
        let mut to = now;

        from.add_days(1);

        to.add_days(2);

        let a = cache.get_in_date_range(
            from.into_candle_date_key(candle_type),
            to.into_candle_date_key(candle_type),
        );

        assert_eq!(0, a.len())
    }
}
