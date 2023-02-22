mod abstractions;
mod candles_cache;
mod models;
mod settings;
mod candles_instrument_cache;

pub use abstractions::*;
pub use candles_cache::*;
pub use settings::*;
pub use candles_instrument_cache::*;
use chrono::{DateTime, Utc};
pub use models::*;

pub fn format_date(date: DateTime<Utc>, candle_type: &CandleType) -> u64 {
    let format = match candle_type {
        CandleType::Minute => "%Y%m%d%H%M",
        CandleType::Hour => "%Y%m%d%H",
        CandleType::Day => "%Y%m%d",
        CandleType::Mounth => "%Y%m",
    };

    return date.format(format).to_string().parse().unwrap();
}
