#[derive(Debug, Clone, Copy)]
pub struct CandleData {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

impl CandleData {
    pub fn new_from_price(price: f64, volume: f64) -> Self {
        Self {
            open: price,
            close: price,
            high: price,
            low: price,
            volume,
        }
    }

    pub fn update_from_price(&mut self, price: f64, volume: f64) {
        self.close = price;
        self.high = self.high.max(price);
        self.low = self.low.min(price);
        self.volume += volume;
    }
}
