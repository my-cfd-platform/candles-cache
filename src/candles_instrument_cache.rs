use std::collections::{BTreeMap, HashMap};

use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    BidOrAsk, CandleData, CandleDateKey, CandleModel, CandleType, CandlesTypesCache, RotateSettings,
};

#[derive(Debug, Clone)]
pub struct CandleResult {
    pub date_key: CandleDateKey,
    pub data: CandleData,
}

pub struct HandleBidAskChanges {
    pub bids_to_persist: Vec<CandleResult>,
    pub asks_to_persist: Vec<CandleResult>,
}

pub struct CandlesInstrumentsCache {
    pub bids_candles: BTreeMap<String, CandlesTypesCache>,
    pub asks_candles: BTreeMap<String, CandlesTypesCache>,
    pub rotate_settings: RotateSettings,
}

impl CandlesInstrumentsCache {
    pub fn new(rotate_settings: RotateSettings) -> Self {
        Self {
            bids_candles: BTreeMap::new(),
            asks_candles: BTreeMap::new(),
            rotate_settings,
        }
    }

    pub async fn handle_bid_ask(
        &mut self,
        instrument_id: &str,
        bid: f64,
        ask: f64,
        time_stamp: DateTimeAsMicroseconds,
    ) -> HandleBidAskChanges {
        if !self.bids_candles.contains_key(instrument_id) {
            self.bids_candles.insert(
                instrument_id.to_string(),
                CandlesTypesCache::new(self.rotate_settings.clone()),
            );
        }

        let bids_to_persist = self
            .bids_candles
            .get_mut(instrument_id)
            .unwrap()
            .handle_new_price(bid, time_stamp);

        if !self.asks_candles.contains_key(instrument_id) {
            self.asks_candles.insert(
                instrument_id.to_string(),
                CandlesTypesCache::new(self.rotate_settings.clone()),
            );
        }

        let asks_to_persist = self
            .asks_candles
            .get_mut(instrument_id)
            .unwrap()
            .handle_new_price(ask, time_stamp);

        HandleBidAskChanges {
            bids_to_persist,
            asks_to_persist,
        }
    }

    fn get_bid_ask_candles_mut(
        &mut self,
        bid_or_ask: BidOrAsk,
    ) -> &mut BTreeMap<String, CandlesTypesCache> {
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
        let rotate_settings = self.rotate_settings.clone();
        let candles = self.get_bid_ask_candles_mut(bid_or_ask);

        for candle_to_init in candles_to_init {
            match candles.get_mut(instrument) {
                Some(candles) => {
                    candles.load_candle(instrument, candle_type, candle_to_init);
                }
                None => {
                    let mut candles_cache = CandlesTypesCache::new(rotate_settings.clone());
                    candles_cache.load_candle(instrument, candle_type, candle_to_init);
                    candles.insert(instrument.to_string(), candles_cache);
                }
            };
        }
    }

    fn get_candles_cache(&self, bid_or_ask: BidOrAsk) -> &BTreeMap<String, CandlesTypesCache> {
        match bid_or_ask {
            BidOrAsk::Bid => &self.bids_candles,
            BidOrAsk::Ask => &self.asks_candles,
        }
    }

    pub fn get_candle(
        &self,
        instrument: &str,
        date_key: CandleDateKey,
        candle_type: CandleType,
        bid_or_ask: BidOrAsk,
    ) -> Option<CandleModel> {
        let result = self.get_candles_cache(bid_or_ask).get(instrument);

        if result.is_none() {
            println!("No cache for instrument {}", instrument);
        }

        let result = result.unwrap();

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

    pub fn get_all_from_cache(&self, bid_or_ask: BidOrAsk) -> HashMap<String, Vec<CandleResult>> {
        let caches = match bid_or_ask {
            BidOrAsk::Bid => &self.bids_candles,
            BidOrAsk::Ask => &self.asks_candles,
        };

        let mut result = HashMap::new();

        for (instrument, cache) in caches {
            let from_cache: Vec<CandleResult> = cache
                .get_all_from_cache()
                .iter()
                .map(|candle| candle.clone())
                .collect();

            result.insert(instrument.to_string(), from_cache);
        }

        return result;
    }
}
