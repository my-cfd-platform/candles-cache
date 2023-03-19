use crate::{CandleData, CandleDateKey};

#[derive(Debug, Clone)]
pub struct CandleModel {
    pub date_key: CandleDateKey,
    pub data: CandleData,
}

impl CandleModel {
    pub fn get_candle_date_key(&self) -> CandleDateKey {
        return self.date_key;
    }
}
