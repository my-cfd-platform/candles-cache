use crate::{CandleModel, CandleType, CandleTypeCache, CandlesBidAsk};
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct CandlesInstrumentsCache {
    pub bid_candles: RwLock<HashMap<String, CandleTypeCache>>,
    pub ask_candles: RwLock<HashMap<String, CandleTypeCache>>,
}

impl CandlesInstrumentsCache {
    pub fn new() -> Self {
        Self {
            bid_candles: RwLock::new(HashMap::new()),
            ask_candles: RwLock::new(HashMap::new()),
        }
    }

    pub async fn update(&self, prices: Vec<CandlesBidAsk>) {
        self.update_bid_or_ask(true, &prices).await;
        self.update_bid_or_ask(false, &prices).await;
    }

    async fn update_bid_or_ask(&self, is_bid: bool, prices: &Vec<CandlesBidAsk>) {
        let mut write_lock = match is_bid {
            true => self.bid_candles.write().await,
            false => self.ask_candles.write().await,
        };

        for bid_ask in prices.iter() {
            let target_instruments_cache = write_lock.get_mut(&bid_ask.instrument);
            let rarget_rate = match is_bid {
                true => bid_ask.bid,
                false => bid_ask.ask,
            };

            match target_instruments_cache {
                Some(cache) => {
                    cache.handle_new_rate(rarget_rate, bid_ask.date);
                }
                None => {
                    let mut cache = CandleTypeCache::new(bid_ask.instrument.clone());
                    cache.handle_new_rate(rarget_rate, bid_ask.date);
                    write_lock.insert(bid_ask.instrument.clone(), cache);
                }
            }
        }
    }

    pub async fn init(
        &self,
        instument_id: String,
        is_bid: bool,
        candle_type: CandleType,
        candle: CandleModel,
    ) {
        let mut target_cache = match is_bid {
            true => self.bid_candles.write().await,
            false => self.ask_candles.write().await,
        };

        let instrument_cache = target_cache.get_mut(&instument_id);

        match instrument_cache {
            Some(cache) => {
                cache.init(candle, candle_type);
            }
            None => {
                let mut cache = CandleTypeCache::new(instument_id.clone());
                cache.init(candle, candle_type);
                target_cache.insert(instument_id, cache);
            }
        }
    }

    pub async fn get_by_date_range(
        &self,
        instument_id: String,
        candle_type: CandleType,
        is_bid: bool,
        start_date: u64,
        end_date: u64,
    ) -> Vec<CandleModel> {
        let target_cache = match is_bid {
            true => self.bid_candles.read().await,
            false => self.ask_candles.read().await,
        };

        let instrument_cache = target_cache.get(&instument_id);

        match instrument_cache {
            Some(cache) => cache.get_by_date_range(candle_type, start_date, end_date),
            None => {
                vec![]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::CandlesBidAsk;

    use super::CandlesInstrumentsCache;

    #[tokio::test]
    async fn test_sinle_quote() {
        let cache = CandlesInstrumentsCache::new();
        let instument = String::from("EURUSD");

        let bid_ask = CandlesBidAsk {
            date: 1662559404,
            instrument: instument.clone(),
            bid: 25.55,
            ask: 36.55,
        };

        cache.update(vec![bid_ask]).await;

        let result_bid_minute = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Minute,
                true,
                1660559404,
                2660559404,
            )
            .await;
        let result_ask_minute = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Minute,
                false,
                1660559404,
                2660559404,
            )
            .await;

        let result_bid_hour = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Hour,
                true,
                1660559404,
                2660559404,
            )
            .await;
        let result_ask_hour = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Hour,
                false,
                1660559404,
                2660559404,
            )
            .await;

        let result_bid_day = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Day,
                true,
                1660559404,
                2660559404,
            )
            .await;
        let result_ask_day = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Day,
                false,
                1660559404,
                2660559404,
            )
            .await;

        let result_bid_mount = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Month,
                true,
                1660559404,
                2660559404,
            )
            .await;
        let result_ask_mount = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Month,
                false,
                1660559404,
                2660559404,
            )
            .await;

        assert_eq!(result_bid_minute.len(), 1);
        assert_eq!(result_ask_minute.len(), 1);

        assert_eq!(result_bid_hour.len(), 1);
        assert_eq!(result_ask_hour.len(), 1);

        assert_eq!(result_bid_day.len(), 1);
        assert_eq!(result_ask_day.len(), 1);

        assert_eq!(result_bid_mount.len(), 1);
        assert_eq!(result_ask_mount.len(), 1);
    }

    #[tokio::test]
    async fn test_date_rotation_minute() {
        let cache = CandlesInstrumentsCache::new();
        let instument = String::from("EURUSD");

        let bid_ask = CandlesBidAsk {
            date: 1662559404,
            instrument: instument.clone(),
            bid: 25.55,
            ask: 36.55,
        };

        cache.update(vec![bid_ask]).await;

        let bid_ask = CandlesBidAsk {
            date: 1662559474,
            instrument: instument.clone(),
            bid: 25.55,
            ask: 36.55,
        };

        cache.update(vec![bid_ask]).await;

        let result_bid_minute = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Minute,
                true,
                1660559404,
                2660559404,
            )
            .await;
        let result_ask_minute = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Minute,
                false,
                1660559404,
                2660559404,
            )
            .await;

        let result_bid_hour = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Hour,
                true,
                1660559404,
                2660559404,
            )
            .await;
        let result_ask_hour = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Hour,
                false,
                1660559404,
                2660559404,
            )
            .await;

        let result_bid_day = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Day,
                true,
                1660559404,
                2660559404,
            )
            .await;
        let result_ask_day = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Day,
                false,
                1660559404,
                2660559404,
            )
            .await;

        let result_bid_mount = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Month,
                true,
                1660559404,
                2660559404,
            )
            .await;
        let result_ask_mount = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Month,
                false,
                1660559404,
                2660559404,
            )
            .await;

        assert_eq!(result_bid_minute.len(), 2);
        assert_eq!(result_ask_minute.len(), 2);

        assert_eq!(result_bid_hour.len(), 1);
        assert_eq!(result_ask_hour.len(), 1);

        assert_eq!(result_bid_day.len(), 1);
        assert_eq!(result_ask_day.len(), 1);

        assert_eq!(result_bid_mount.len(), 1);
        assert_eq!(result_ask_mount.len(), 1);
    }

    #[tokio::test]
    async fn test_calculation() {
        let cache = CandlesInstrumentsCache::new();
        let instument = String::from("EURUSD");

        let bid_ask = CandlesBidAsk {
            date: 1662559404,
            instrument: instument.clone(),
            bid: 25.55,
            ask: 36.55,
        };

        cache.update(vec![bid_ask]).await;

        let bid_ask = CandlesBidAsk {
            date: 1662559406,
            instrument: instument.clone(),
            bid: 60.55,
            ask: 31.55,
        };

        cache.update(vec![bid_ask]).await;

        let bid_ask = CandlesBidAsk {
            date: 1662559407,
            instrument: instument.clone(),
            bid: 50.55,
            ask: 62.55,
        };

        cache.update(vec![bid_ask]).await;

        let result_bid_minute = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Minute,
                true,
                1660559404,
                2660559404,
            )
            .await;
        let result_ask_minute = cache
            .get_by_date_range(
                instument.clone(),
                crate::models::CandleType::Minute,
                false,
                1660559404,
                2660559404,
            )
            .await;

        assert_eq!(result_bid_minute.len(), 1);
        assert_eq!(result_ask_minute.len(), 1);

        let first_bid = result_bid_minute.first().unwrap();
        let first_ask = result_ask_minute.first().unwrap();

        assert_eq!(first_bid.open, 25.55);
        assert_eq!(first_bid.close, 50.55);
        assert_eq!(first_bid.high, 60.55);
        assert_eq!(first_bid.low, 25.55);

        assert_eq!(first_ask.open, 36.55);
        assert_eq!(first_ask.close, 62.55);
        assert_eq!(first_ask.high, 62.55);
        assert_eq!(first_ask.low, 31.55);

        assert_eq!(first_bid.datetime, 1662559380);
        assert_eq!(first_ask.datetime, 1662559380);
    }
}
