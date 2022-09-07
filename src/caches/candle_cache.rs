use std::{collections::BTreeMap};

use crate::{CandleType, CandleModel};

pub struct CandlesCache{
    pub candle_type: CandleType,
    pub candles: BTreeMap<u64, CandleModel>
}

impl CandlesCache {
    pub fn new(candle_type: CandleType) -> Self{
        Self { candle_type: candle_type, candles: BTreeMap::new() }
    }

    pub fn init(&mut self, candle: CandleModel){
        self.candles.insert(candle.datetime, candle);
    }

    pub fn handle_new_rate(&mut self, date: u64, rate: f64){
        let date = self.candle_type.format_date_by_type(date);

        let target_candle = self.candles.get_mut(&date);

        match target_candle {
            Some(candle) => candle.update_by_rate(rate),
            None => {
                let candle_model = CandleModel::new_from_rate(self.candle_type.clone(), date, rate);
                self.candles.insert(date, candle_model);
            },
        }
    }

    pub fn get_by_date_range(&self, date_from: u64, date_to:u64) -> Vec<CandleModel>{
        let mut result = Vec::new();

        for (_, candle) in self.candles.range(date_from..date_to){
            result.push(candle.clone());
        }

        result
    }
}

