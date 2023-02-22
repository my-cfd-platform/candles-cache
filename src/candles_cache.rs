use std::collections::BTreeMap;

use chrono::{DateTime, Duration, Utc};

use crate::{format_date, CandleLoadModel, CandleModel, CandleType, RotateSettigns};

pub struct CandlesTypesCache {
    pub candles: BTreeMap<u8, CandleDateCache>,
}

impl CandlesTypesCache {
    pub fn new(rotate_settings: RotateSettigns) -> Self {
        Self {
            candles: BTreeMap::from([
                (
                    CandleType::Minute as u8,
                    CandleDateCache::new(
                        CandleType::Minute,
                        rotate_settings.get_target(&CandleType::Minute),
                    ),
                ),
                (
                    CandleType::Hour as u8,
                    CandleDateCache::new(
                        CandleType::Hour,
                        rotate_settings.get_target(&CandleType::Hour),
                    ),
                ),
                (
                    CandleType::Day as u8,
                    CandleDateCache::new(
                        CandleType::Day,
                        rotate_settings.get_target(&CandleType::Day),
                    ),
                ),
                (
                    CandleType::Mounth as u8,
                    CandleDateCache::new(
                        CandleType::Mounth,
                        rotate_settings.get_target(&CandleType::Mounth),
                    ),
                ),
            ]),
        }
    }

    pub fn load_candle(&mut self, candle: CandleLoadModel) {
        let Some(date_candle) = self.candles.get_mut(&(candle.candle_type as u8)) else{
            panic!("Invalid candle type")
        };

        date_candle.load(candle);
    }

    pub fn handle_new_price(mut self, price: f64, price_date: DateTime<Utc>) {
        for (_, candle_cache) in self.candles.iter_mut() {
            candle_cache.handle_price(price, price_date);
        }
    }

    pub fn get_in_date_range(
        &self,
        date_from: DateTime<Utc>,
        date_to: DateTime<Utc>,
        candle_type: CandleType,
    ) -> Vec<CandleModel> {
        let Some(candle_cache) = self.candles.get(&(candle_type as u8)) else{
            return Vec::new();
        };

        return candle_cache.get_in_date_range(date_from, date_to);
    }
}

pub struct CandleDateCache {
    pub candles: BTreeMap<u64, CandleModel>,
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
        let date_index = candle_to_load.get_formatted_date();
        let model: CandleModel = candle_to_load.into();
        self.candles.insert(date_index, model);
    }

    pub fn get_in_date_range(
        &self,
        date_from: DateTime<Utc>,
        date_to: DateTime<Utc>,
    ) -> Vec<CandleModel> {
        let mut candles = Vec::new();

        let date_from = format_date(date_from, &self.candle_type);
        let date_to = format_date(date_to, &self.candle_type);

        for (_, candle) in self.candles.range(date_from..date_to) {
            candles.push(candle.clone());
        }

        return candles;
    }

    pub fn handle_price(&mut self, price: f64, price_date: DateTime<Utc>) {
        let date: u64 = format_date(price_date, &self.candle_type);

        let Some(candle) = self.candles.get_mut(&date) else{
            let candle = CandleModel::new_from_price(price, 0.0);
            self.candles.insert(date, candle);
            return;
        };

        candle.update_from_price(price, 0.0);
        self.rotate_candles();
    }

    fn rotate_candles(&mut self) {
        let ids_to_remove = self.get_candles_ids_to_rotate();

        for date in ids_to_remove {
            self.candles.remove(&date);
        }
    }

    fn get_candles_ids_to_rotate(&self) -> Vec<u64> {
        let Some(cache_load_duration) = self.rotate_period else{
            return Vec::new();
        };

        let now = Utc::now();
        let max_possible_date = now - cache_load_duration;
        let key_date = format_date(max_possible_date, &self.candle_type);
        return self
            .candles
            .range(..key_date)
            .map(|(date, _)| *date)
            .collect();
    }
}
