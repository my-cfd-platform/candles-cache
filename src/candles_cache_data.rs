use std::collections::HashMap;

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    CandleDateCache, CandleDateKey, CandleModel, CandleToPersist, CandleType, GetCandleDateKey,
    RotateSettings,
};

pub struct CandlesTypesCache {
    pub candles: HashMap<u8, CandleDateCache>,
}

impl CandlesTypesCache {
    pub fn new(rotate_settings: RotateSettings) -> Self {
        Self {
            candles: HashMap::from([
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

    pub fn load_candle(&mut self, instrument: &str, candle_type: CandleType, candle: CandleModel) {
        match self.candles.get_mut(&candle_type.to_u8()) {
            Some(candles) => {
                candles.load(candle);
            }
            None => {
                panic!(
                    "Invalid candle type {} in candle {} of date_key {}",
                    candle_type.to_u8(),
                    instrument,
                    candle.date_key.get_value()
                )
            }
        }
    }

    pub fn handle_new_price(
        &mut self,
        price: f64,
        price_date: DateTimeAsMicroseconds,
    ) -> Vec<CandleToPersist> {
        let mut result = vec![];
        for (candle_type, candle_cache) in &mut self.candles {
            let candle_type = CandleType::from_u8(*candle_type);
            let date_key = price_date.into_candle_date_key(candle_type);

            let new_candle_data = candle_cache.handle_price(price, date_key);
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
}
