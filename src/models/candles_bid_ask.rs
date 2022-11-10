#[derive(Debug, Clone)]
pub struct CandlesBidAsk{
    pub date: u64,
    pub instrument: String,
    pub bid: f64,
    pub ask: f64,
}