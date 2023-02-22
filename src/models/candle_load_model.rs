use chrono::{DateTime, Utc};

use crate::{format_date, CandleModel, CandleType};

#[derive(Debug, Clone)]
pub struct CandleLoadModel {
    pub insetument: String,
    pub candle_type: CandleType,
    pub candle_model: CandleModel,
    pub candle_date: DateTime<Utc>,
}

impl CandleLoadModel {
    pub fn get_formatted_date(&self) -> u64 {
        return format_date(self.candle_date, &self.candle_type);
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
