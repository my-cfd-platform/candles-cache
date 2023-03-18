use crate::{CandleDateKey, CandleModel, CandleType};

#[derive(Debug, Clone)]
pub struct CandleLoadModel {
    pub instrument: String,
    pub candle_type: CandleType,
    pub candle_model: CandleModel,
    pub candle_date: CandleDateKey,
}

impl CandleLoadModel {
    pub fn get_candle_date_key(&self) -> CandleDateKey {
        return self.candle_date;
    }
}

impl Into<CandleModel> for CandleLoadModel {
    fn into(self) -> CandleModel {
        CandleModel {
            open: self.candle_model.open,
            close: self.candle_model.close,
            high: self.candle_model.high,
            low: self.candle_model.low,
            volume: self.candle_model.volume,
        }
    }
}
