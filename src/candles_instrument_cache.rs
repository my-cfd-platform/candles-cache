use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

use crate::{
    CandleLoadModel, CandleModel, CandlePersistModel, CandleType, CandlesBidAsk,
    CandlesPersistCache, CandlesTypesCache, RotateSettings,
};

pub struct CandlesInstrumentsCache {
    pub bids_candles: BTreeMap<String, CandlesTypesCache>,
    pub asks_candles: BTreeMap<String, CandlesTypesCache>,
    pub rotate_settings: RotateSettings,
    pub bids_persist_cache: CandlesPersistCache,
    pub asks_persist_cache: CandlesPersistCache,
}

impl CandlesInstrumentsCache {
    pub fn new(rotate_settings: RotateSettings) -> Self {
        Self {
            bids_candles: BTreeMap::new(),
            asks_candles: BTreeMap::new(),
            rotate_settings,
            bids_persist_cache: CandlesPersistCache::new(),
            asks_persist_cache: CandlesPersistCache::new(),
        }
    }

    pub async fn handle_bid_ask(&mut self, bid_ask: &impl CandlesBidAsk) {
        let bids_cache = self
            .bids_candles
            .entry(bid_ask.get_instrument())
            .or_insert_with(|| CandlesTypesCache::new(self.rotate_settings.clone()));

        let bids_to_persist =
            bids_cache.handle_new_price(bid_ask.get_bid(), bid_ask.get_timestamp());

        for (date, candle_type, candle) in bids_to_persist {
            self.bids_persist_cache
                .handle_candle(CandlePersistModel {
                    instrument: bid_ask.get_instrument(),
                    high: candle.high,
                    low: candle.low,
                    open: candle.open,
                    close: candle.close,
                    volume: candle.volume,
                    date: date.clone(),
                    candle_type,
                })
                .await;
        }

        let asks_cache = self
            .asks_candles
            .entry(bid_ask.get_instrument())
            .or_insert_with(|| CandlesTypesCache::new(self.rotate_settings.clone()));

        let asks_to_persist =
            asks_cache.handle_new_price(bid_ask.get_bid(), bid_ask.get_timestamp());

        for (date, candle_type, candle) in asks_to_persist {
            self.asks_persist_cache
                .handle_candle(CandlePersistModel {
                    instrument: bid_ask.get_instrument(),
                    high: candle.high,
                    low: candle.low,
                    open: candle.open,
                    close: candle.close,
                    volume: candle.volume,
                    date: date.clone(),
                    candle_type,
                })
                .await;
        }
    }

    pub fn load_bids_candles(&mut self, candles: Vec<CandleLoadModel>) {
        for candle in candles {
            match self.bids_candles.get_mut(&candle.instrument) {
                Some(candles) => {
                    candles.load_candle(candle);
                }
                None => {
                    let mut candles = CandlesTypesCache::new(self.rotate_settings.clone());
                    candles.load_candle(candle.clone());
                    self.bids_candles.insert(candle.instrument, candles);
                }
            };
        }
    }

    pub fn load_asks_candles(&mut self, candles: Vec<CandleLoadModel>) {
        for candle in candles {
            match self.asks_candles.get_mut(&candle.instrument) {
                Some(candles) => {
                    candles.load_candle(candle);
                }
                None => {
                    let mut candles = CandlesTypesCache::new(self.rotate_settings.clone());
                    candles.load_candle(candle.clone());
                    self.asks_candles.insert(candle.instrument, candles);
                }
            };
        }
    }

    pub fn get_in_date_range(
        &self,
        instrument: &str,
        date_from: DateTime<Utc>,
        date_to: DateTime<Utc>,
        candle_type: CandleType,
        is_bid: bool,
    ) -> Option<Vec<(u64, CandleModel)>> {
        let mut result = None;
        match is_bid {
            true => {
                if let Some(candles) = self.bids_candles.get(instrument) {
                    result = Some(candles.get_in_date_range(date_from, date_to, candle_type));
                }
            }
            false => {
                if let Some(candles) = self.asks_candles.get(instrument) {
                    result = Some(candles.get_in_date_range(date_from, date_to, candle_type));
                }
            }
        };

        return result;
    }

    pub fn get_all_from_cache(&self, is_bid: bool) -> Vec<(String, u64, CandleType, CandleModel)> {
        let caches = match is_bid {
            true => &self.bids_candles,
            false => &self.asks_candles,
        };

        let mut result = vec![];

        for (instrument, cache) in caches {
            let mut from_cache = cache
                .get_all_from_cache()
                .iter()
                .map(|(date, candle_type, candle)| {
                    (
                        instrument.clone(),
                        date.to_owned(),
                        candle_type.to_owned(),
                        candle.to_owned(),
                    )
                })
                .collect::<Vec<(String, u64, CandleType, CandleModel)>>();

            result.append(&mut from_cache)
        }

        return result;
    }
}
