use std::{
    collections::{BTreeMap, HashMap},
    time::Duration,
};

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{BidOrAsk, CandleData, CandleDateKey, CandleModel, CandleType, CandlesCacheByType};

#[derive(Debug, Clone)]
pub struct CandleToPersist {
    pub date_key: CandleDateKey,
    pub candle_type: CandleType,
    pub data: CandleData,
}

pub struct HandleBidAskChanges {
    pub bids_to_persist: Vec<CandleToPersist>,
    pub asks_to_persist: Vec<CandleToPersist>,
}

pub struct CandlesInstrumentsCache {
    pub bids_candles: BTreeMap<String, CandlesCacheByType>,
    pub asks_candles: BTreeMap<String, CandlesCacheByType>,
}

pub struct CleanIntervalParameters {
    pub from: CandleDateKey,
    pub to: CandleDateKey,
}

impl CandlesInstrumentsCache {
    pub fn new() -> Self {
        Self {
            bids_candles: BTreeMap::new(),
            asks_candles: BTreeMap::new(),
        }
    }

    pub async fn handle_bid_ask(
        &mut self,
        instrument_id: &str,
        bid: f64,
        ask: f64,
        time_stamp: DateTimeAsMicroseconds,
        rotation_period: Option<Duration>,
    ) -> HandleBidAskChanges {
        if !self.bids_candles.contains_key(instrument_id) {
            self.bids_candles
                .insert(instrument_id.to_string(), CandlesCacheByType::new());
        }

        let bids_to_persist = self
            .bids_candles
            .get_mut(instrument_id)
            .unwrap()
            .handle_new_price(bid, time_stamp, rotation_period);

        if !self.asks_candles.contains_key(instrument_id) {
            self.asks_candles
                .insert(instrument_id.to_string(), CandlesCacheByType::new());
        }

        let asks_to_persist = self
            .asks_candles
            .get_mut(instrument_id)
            .unwrap()
            .handle_new_price(ask, time_stamp, rotation_period);

        HandleBidAskChanges {
            bids_to_persist,
            asks_to_persist,
        }
    }

    fn get_candles_cache(&self, bid_or_ask: BidOrAsk) -> &BTreeMap<String, CandlesCacheByType> {
        match bid_or_ask {
            BidOrAsk::Bid => &self.bids_candles,
            BidOrAsk::Ask => &self.asks_candles,
        }
    }

    fn get_bid_ask_candles_mut(
        &mut self,
        bid_or_ask: BidOrAsk,
    ) -> &mut BTreeMap<String, CandlesCacheByType> {
        match bid_or_ask {
            BidOrAsk::Bid => &mut self.bids_candles,
            BidOrAsk::Ask => &mut self.asks_candles,
        }
    }

    pub fn init_candles(
        &mut self,
        bid_or_ask: BidOrAsk,
        instrument: &str,
        candle_type: CandleType,
        candles_to_init: impl Iterator<Item = CandleModel>,
    ) {
        let candles = self.get_bid_ask_candles_mut(bid_or_ask);

        for candle_to_init in candles_to_init {
            match candles.get_mut(instrument) {
                Some(candles) => {
                    candles.insert_or_update(candle_type, candle_to_init);
                }
                None => {
                    let mut candles_cache = CandlesCacheByType::new();
                    candles_cache.insert_or_update(candle_type, candle_to_init);
                    candles.insert(instrument.to_string(), candles_cache);
                }
            };
        }
    }

    pub fn clean_by_type(
        &mut self,
        bid_or_ask: BidOrAsk,
        instrument_id: &str,
        candle_type: CandleType,
    ) {
        let candles = self.get_bid_ask_candles_mut(bid_or_ask);
        if let Some(candles) = candles.get_mut(instrument_id) {
            candles.clean_by_type(candle_type);
        }
    }

    pub fn bulk_insert_or_update(
        &mut self,
        bid_or_ask: BidOrAsk,
        instrument: &str,
        candle_type: CandleType,
        candles_to_init: impl Iterator<Item = CandleModel>,
    ) {
        let candles = self.get_bid_ask_candles_mut(bid_or_ask);

        for candle_to_init in candles_to_init {
            match candles.get_mut(instrument) {
                Some(candles) => {
                    candles.insert_or_update(candle_type, candle_to_init);
                }
                None => {
                    let mut candles_cache = CandlesCacheByType::new();
                    candles_cache.insert_or_update(candle_type, candle_to_init);
                    candles.insert(instrument.to_string(), candles_cache);
                }
            };
        }
    }

    pub fn get_candle(
        &self,
        instrument: &str,
        date_key: CandleDateKey,
        candle_type: CandleType,
        bid_or_ask: BidOrAsk,
    ) -> Option<CandleModel> {
        let cache_by_type = self.get_candles_cache(bid_or_ask).get(instrument);

        if cache_by_type.is_none() {
            println!("No cache for instrument {}", instrument);
        }

        let result = cache_by_type.unwrap();

        result.get_candle(date_key, candle_type)
    }

    pub fn get_in_date_range(
        &self,
        instrument: &str,
        date_from: DateTimeAsMicroseconds,
        date_to: DateTimeAsMicroseconds,
        candle_type: CandleType,
        bid_or_ask: BidOrAsk,
    ) -> Option<Vec<CandleModel>> {
        let cache_by_instrument = self.get_candles_cache(bid_or_ask).get(instrument)?;
        cache_by_instrument.get_in_date_range(date_from, date_to, candle_type)
    }

    pub fn get_all_from_cache(
        &self,
        bid_or_ask: BidOrAsk,
    ) -> HashMap<String, Vec<(CandleType, Vec<CandleModel>)>> {
        let cache_by_type = self.get_candles_cache(bid_or_ask);

        let mut result = HashMap::new();

        for (instrument, cache) in cache_by_type {
            let from_cache = cache.get_all_from_cache();
            result.insert(instrument.to_string(), from_cache);
        }

        return result;
    }

    pub fn get_all_by_instrument(
        &self,
        bid_or_ask: BidOrAsk,
        instrument_id: &str,
    ) -> Option<Vec<(CandleType, Vec<CandleModel>)>> {
        let cache_by_type = self.get_candles_cache(bid_or_ask);

        if let Some(by_instrument) = cache_by_type.get(instrument_id) {
            return Some(by_instrument.get_all_from_cache());
        }

        return None;
    }
}
