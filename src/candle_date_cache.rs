use std::{collections::BTreeMap, time::Duration};

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    CandleDateKey, CandleLoadModel, CandleModel, CandleResult, CandleType, GetCandleDateKey,
};

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

    pub fn get_candle(&self, date_key: CandleDateKey) -> Option<CandleModel> {
        self.candles.get(&date_key).cloned()
    }
}
