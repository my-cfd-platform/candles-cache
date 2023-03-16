use chrono::{DateTime, Utc};

pub trait CandlesBidAsk {
    fn get_instrument(&self) -> String;
    fn get_bid(&self) -> f64;
    fn get_ask(&self) -> f64;
    fn get_timestamp(&self) -> DateTime<Utc>;
}