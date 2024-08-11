use std::collections::{BTreeMap, HashMap, HashSet};

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
    pub bid_candles: BTreeMap<String, CandlesCacheByType>,
    pub ask_candles: BTreeMap<String, CandlesCacheByType>,
}

pub struct CleanIntervalParameters {
    pub from: CandleDateKey,
    pub to: CandleDateKey,
}

impl CandlesInstrumentsCache {
    pub fn new() -> Self {
        Self {
            bid_candles: BTreeMap::new(),
            ask_candles: BTreeMap::new(),
        }
    }

    pub async fn handle_bid_ask(
        &mut self,
        instrument_id: &str,
        bid: f64,
        ask: f64,
        time_stamp: DateTimeAsMicroseconds,
        max_candles_amount: usize,
    ) -> HandleBidAskChanges {
        if !self.bid_candles.contains_key(instrument_id) {
            self.bid_candles
                .insert(instrument_id.to_string(), CandlesCacheByType::new());
        }

        let bids_to_persist = self
            .bid_candles
            .get_mut(instrument_id)
            .unwrap()
            .handle_new_price(bid, time_stamp, max_candles_amount);

        if !self.ask_candles.contains_key(instrument_id) {
            self.ask_candles
                .insert(instrument_id.to_string(), CandlesCacheByType::new());
        }

        let asks_to_persist = self
            .ask_candles
            .get_mut(instrument_id)
            .unwrap()
            .handle_new_price(ask, time_stamp, max_candles_amount);

        HandleBidAskChanges {
            bids_to_persist,
            asks_to_persist,
        }
    }

    fn get_candles_cache(&self, bid_or_ask: BidOrAsk) -> &BTreeMap<String, CandlesCacheByType> {
        match bid_or_ask {
            BidOrAsk::Bid => &self.bid_candles,
            BidOrAsk::Ask => &self.ask_candles,
        }
    }

    fn get_candles_cache_mut(
        &mut self,
        bid_or_ask: BidOrAsk,
    ) -> &mut BTreeMap<String, CandlesCacheByType> {
        match bid_or_ask {
            BidOrAsk::Bid => &mut self.bid_candles,
            BidOrAsk::Ask => &mut self.ask_candles,
        }
    }

    pub fn init_candles(
        &mut self,
        bid_or_ask: BidOrAsk,
        instrument: &str,
        candle_type: CandleType,
        candles_to_init: impl Iterator<Item = CandleModel>,
    ) {
        let candles = self.get_candles_cache_mut(bid_or_ask);

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

    pub fn get_instruments(&self) -> HashSet<String> {
        let mut result = HashSet::new();
        let candles = self.get_candles_cache(BidOrAsk::Bid);

        for instrument in candles.keys() {
            result.insert(instrument.to_string());
        }

        let candles = self.get_candles_cache(BidOrAsk::Ask);

        for instrument in candles.keys() {
            if !result.contains(instrument) {
                result.insert(instrument.to_string());
            }
        }

        result
    }

    pub fn clean_by_type(
        &mut self,
        bid_or_ask: BidOrAsk,
        instrument_id: &str,
        candle_type: CandleType,
    ) {
        let candles = self.get_candles_cache_mut(bid_or_ask);
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
        let candles = self.get_candles_cache_mut(bid_or_ask);

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
        from: CandleDateKey,
        to: CandleDateKey,
        candle_type: CandleType,
        bid_or_ask: BidOrAsk,
    ) -> Option<&[CandleModel]> {
        let cache_by_instrument = self.get_candles_cache(bid_or_ask).get(instrument)?;
        cache_by_instrument.get_in_date_range(from, to, candle_type)
    }

    pub fn get_highest_and_below(
        &self,
        bid_or_ask: BidOrAsk,
        instrument: &str,
        candle_type: CandleType,
        highest: CandleDateKey,
        amount: usize,
    ) -> Option<&[CandleModel]> {
        let cache_by_instrument = self.get_candles_cache(bid_or_ask).get(instrument)?;
        cache_by_instrument.get_highest_and_below(candle_type, highest, amount)
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

    pub fn iter_all_by_instrument(
        &self,
        bid_or_ask: BidOrAsk,
        instrument_id: &str,
        candle_type: CandleType,
    ) -> Option<impl Iterator<Item = &CandleModel>> {
        let cache_by_type = self.get_candles_cache(bid_or_ask);
        let instruments = cache_by_type.get(instrument_id)?;
        instruments.iter_by_type(candle_type)
    }

    pub fn iter_all(&self, bid_or_ask: BidOrAsk) -> impl Iterator<Item = &CandleModel> {
        let cache_by_type = self.get_candles_cache(bid_or_ask);
        cache_by_type.values().flat_map(|itm| itm.iter_all())
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

    pub fn gc_candles_by_instrument(
        &mut self,
        instrument: &str,
        candle_type: CandleType,
        max_candles_amount: usize,
    ) {
        if let Some(cache) = self.bid_candles.get_mut(instrument) {
            cache.gc_by_type(candle_type, max_candles_amount);
        }

        if let Some(cache) = self.ask_candles.get_mut(instrument) {
            cache.gc_by_type(candle_type, max_candles_amount);
        }
    }

    pub fn get_first_candle(
        &self,
        bid_or_ask: BidOrAsk,
        instrument: &str,
        candle_type: CandleType,
    ) -> Option<&CandleModel> {
        let candles_cache = self.get_candles_cache(bid_or_ask);

        let by_instrument = candles_cache.get(instrument)?;
        by_instrument.get_first_candle(candle_type)
    }

    pub fn gc_candles(&mut self, candle_type: CandleType, max_candles_amount: usize) {
        for cache in self.bid_candles.values_mut() {
            cache.gc_by_type(candle_type, max_candles_amount);
        }

        for cache in self.ask_candles.values_mut() {
            cache.gc_by_type(candle_type, max_candles_amount);
        }
    }
}
