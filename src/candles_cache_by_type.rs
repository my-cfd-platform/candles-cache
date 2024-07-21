use std::{collections::HashMap, time::Duration};

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    CandleDateCache, CandleDateKey, CandleModel, CandleToPersist, CandleType, GetCandleDateKey,
};

pub struct CandlesCacheByType {
    pub candles: HashMap<u8, CandleDateCache>,
}

impl CandlesCacheByType {
    pub fn new() -> Self {
        Self {
            candles: HashMap::new(),
        }
    }

    fn get_or_create_by_candle_type_mut(
        &mut self,
        candle_type: CandleType,
    ) -> &mut CandleDateCache {
        let candle_type_as_u8 = candle_type.to_u8();
        if self.candles.contains_key(&candle_type_as_u8) {
            let result = CandleDateCache::new(candle_type);
            self.candles.insert(candle_type.to_u8(), result);
        }

        self.candles.get_mut(&candle_type.to_u8()).unwrap()
    }

    pub fn insert_or_update(&mut self, candle_type: CandleType, candle: CandleModel) {
        self.get_or_create_by_candle_type_mut(candle_type)
            .insert_or_update(candle)
    }

    pub fn handle_new_price(
        &mut self,
        price: f64,
        price_date: DateTimeAsMicroseconds,
        rotation_period: Option<Duration>,
    ) -> Vec<CandleToPersist> {
        let mut result = Vec::new();

        for candle_type in CandleType::ALL_CANDLE_TYPES {
            let cache_data = self.get_or_create_by_candle_type_mut(candle_type);

            let date_key = price_date.into_candle_date_key(candle_type);

            let new_candle_data = cache_data.handle_price(price, date_key, rotation_period);
            result.push(CandleToPersist {
                date_key,
                candle_type,
                data: new_candle_data,
            })
        }

        return result;
    }

    pub fn get_in_date_range(
        &self,
        date_from: DateTimeAsMicroseconds,
        date_to: DateTimeAsMicroseconds,
        candle_type: CandleType,
    ) -> Option<Vec<CandleModel>> {
        let candles_by_type = self.candles.get(&(candle_type.to_u8()))?;

        Some(candles_by_type.get_in_date_range(date_from, date_to))
    }

    pub fn get_all_from_cache(&self) -> Vec<(CandleType, Vec<CandleModel>)> {
        let mut result = Vec::new();

        for (candle_type, candle_cache) in &self.candles {
            let candles = candle_cache.get_all_from_cache();
            result.push((CandleType::from_u8(*candle_type), candles));
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

    pub fn clean_by_type(&mut self, candle_type: CandleType) {
        self.candles.remove(&candle_type.to_u8());
    }
}
