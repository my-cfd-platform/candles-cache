use std::collections::HashMap;

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

    pub fn pre_allocate_memory_if_needed(&mut self, candle_type: CandleType, amount: usize) {
        self.get_or_create_by_candle_type_mut(candle_type)
            .pre_allocate_memory_if_needed(amount);
    }

    fn get_or_create_by_candle_type_mut(
        &mut self,
        candle_type: CandleType,
    ) -> &mut CandleDateCache {
        let candle_type_as_u8 = candle_type.to_u8();
        if !self.candles.contains_key(&candle_type_as_u8) {
            let result = CandleDateCache::new(candle_type);
            self.candles.insert(candle_type.to_u8(), result);
        }

        self.candles.get_mut(&candle_type.to_u8()).unwrap()
    }

    pub fn insert_or_update(&mut self, candle_type: CandleType, candle: CandleModel) {
        self.get_or_create_by_candle_type_mut(candle_type)
            .insert_or_update(candle)
    }

    pub fn get_first_candle(&self, candle_type: CandleType) -> Option<&CandleModel> {
        let candles_by_type = self.candles.get(&(candle_type.to_u8()))?;
        candles_by_type.get_first_candle()
    }

    pub fn iter_all(&self) -> impl Iterator<Item = &CandleModel> {
        self.candles.values().flat_map(|itm| itm.iter())
    }

    pub fn iter_by_type(
        &self,
        candle_type: CandleType,
    ) -> Option<impl Iterator<Item = &CandleModel>> {
        let candles_by_type = self.candles.get(&(candle_type.to_u8()))?;
        Some(candles_by_type.iter())
    }

    pub fn handle_new_price(
        &mut self,
        price: f64,
        price_date: DateTimeAsMicroseconds,
        max_candles_amount: usize,
    ) -> Vec<CandleToPersist> {
        let mut result = Vec::new();

        for candle_type in CandleType::ALL_CANDLE_TYPES {
            let cache_data = self.get_or_create_by_candle_type_mut(candle_type);

            let date_key = price_date.into_candle_date_key(candle_type);

            let new_candle_data =
                cache_data.handle_price(price, date_key, Some(max_candles_amount));
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
        from: CandleDateKey,
        to: CandleDateKey,
        candle_type: CandleType,
    ) -> Option<&[CandleModel]> {
        let candles_by_type = self.candles.get(&(candle_type.to_u8()))?;

        Some(candles_by_type.get_in_date_range(from, to))
    }

    pub fn get_highest_and_below(
        &self,
        candle_type: CandleType,
        highest: CandleDateKey,
        amount: usize,
    ) -> Option<&[CandleModel]> {
        let candles_by_type = self.candles.get(&(candle_type.to_u8()))?;
        Some(candles_by_type.get_highest_and_below(highest, amount))
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

    pub fn gc_by_type(&mut self, candle_type: CandleType, max_candles_amount: usize) {
        if let Some(candle_type) = self.candles.get_mut(&candle_type.to_u8()) {
            candle_type.gc_candles(max_candles_amount);
        }
    }
}
