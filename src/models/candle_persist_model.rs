use crate::CandleType;

#[derive(Clone, Debug)]
pub struct CandlePersistModel{
    pub instrument: String,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
    pub date: u64,
    pub candle_type: CandleType
}