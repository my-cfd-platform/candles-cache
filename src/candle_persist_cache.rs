use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::{CandleDateKey, CandlePersistModel};

pub struct CandlesPersistCache {
    pub to_persist: RwLock<HashMap<String, HashMap<CandleDateKey, CandlePersistModel>>>,
}

impl CandlesPersistCache {
    pub fn new() -> Self {
        Self {
            to_persist: RwLock::new(HashMap::new()),
        }
    }

    pub async fn handle_candle(&self, candle_model: CandlePersistModel) {
        let mut to_persist = self.to_persist.write().await;
        let candles = to_persist
            .entry(candle_model.instrument.clone())
            .or_insert(HashMap::new());
        candles.insert(candle_model.date, candle_model);
    }

    pub async fn get_model_to_persist(&self) -> Vec<CandlePersistModel> {
        let mut to_persist = self.to_persist.write().await;
        let mut result = vec![];

        for (_, candle) in to_persist.iter() {
            for candle in candle.values() {
                result.push(candle.clone());
            }
        }

        to_persist.clear();

        return result;
    }
}
