use rust_extensions::date_time::DateTimeAsMicroseconds;

pub enum BidOrAsk {
    Bid,
    Ask,
}

pub trait CandlesBidAsk {
    fn get_instrument(&self) -> String;
    fn get_bid(&self) -> f64;
    fn get_ask(&self) -> f64;
    fn get_timestamp(&self) -> DateTimeAsMicroseconds;
}
