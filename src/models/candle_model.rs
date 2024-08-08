use rust_extensions::sorted_vec::EntityWithKey;

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

impl EntityWithKey<u64> for CandleModel {
    fn get_key(&self) -> &u64 {
        return self.date_key.as_ref();
    }
}
