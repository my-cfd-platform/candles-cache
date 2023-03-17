mod abstractions;
mod candle_persist_cache;
mod candles_cache;
mod candles_instrument_cache;
mod models;
mod settings;

pub use abstractions::*;
pub use candle_persist_cache::*;
pub use candles_cache::*;
pub use candles_instrument_cache::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;
pub use settings::*;

pub use models::*;

pub fn format_date(date: DateTimeAsMicroseconds, candle_type: &CandleType) -> u64 {
    let format = match candle_type {
        CandleType::Minute => "%Y%m%d%H%M",
        CandleType::Hour => "%Y%m%d%H",
        CandleType::Day => "%Y%m%d",
        CandleType::Month => "%Y%m",
    };

    let date = format!("{:0<12}", date.to_chrono_utc().format(format));

    return date.to_string().parse().unwrap();
}
