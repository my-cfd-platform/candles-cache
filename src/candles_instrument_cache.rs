use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

use crate::{CandleLoadModel, CandleModel, CandleType, CandlesTypesCache, RotateSettigns};

pub struct CandlesInstrumentsCache {
    pub candles: BTreeMap<String, CandlesTypesCache>,
    pub rotate_settings: RotateSettigns,
}

impl CandlesInstrumentsCache {
    pub fn new(rotate_settings: RotateSettigns) -> Self {
        Self {
            candles: BTreeMap::new(),
            rotate_settings,
        }
    }

    pub fn load_candle(&mut self, candle: CandleLoadModel) {
        match self.candles.get_mut(&candle.insetument) {
            Some(candles) => {
                candles.load_candle(candle);
            }
            None => {
                let mut candles = CandlesTypesCache::new(self.rotate_settings.clone());
                candles.load_candle(candle.clone());
                self.candles.insert(candle.insetument, candles);
            }
        };
    }

    pub fn get_in_date_range(
        &self,
        instrument: &str,
        date_from: DateTime<Utc>,
        date_to: DateTime<Utc>,
        candle_type: CandleType,
    ) -> Vec<CandleModel> {
        let Some(candles) = self.candles.get(instrument) else{
            panic!("Invalid instrument")
        };

        candles.get_in_date_range(date_from, date_to, candle_type)
    }
}
