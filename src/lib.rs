mod abstractions;
mod candle_persist_cache;
mod candles_cache_data;
mod candles_instrument_cache;
mod models;
mod settings;

pub use abstractions::*;
pub use candle_persist_cache::*;
pub use candles_cache_data::*;
pub use candles_instrument_cache::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;
pub use settings::*;

pub use models::*;

pub trait CandleDateTimeKey {
    fn format_date(&self, candle_type: CandleType) -> u64;
}

impl CandleDateTimeKey for DateTimeAsMicroseconds {
    fn format_date(&self, candle_type: CandleType) -> u64 {
        let format = match candle_type {
            CandleType::Minute => "%Y%m%d%H%M",
            CandleType::Hour => "%Y%m%d%H",
            CandleType::Day => "%Y%m%d",
            CandleType::Month => "%Y%m",
        };

        let date = format!("{:0<12}", self.to_chrono_utc().format(format));

        return date.to_string().parse().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use crate::{CandleDateTimeKey, CandleType};

    #[test]
    fn test() {
        let date_time = DateTimeAsMicroseconds::now();

        let d = date_time.format_date(CandleType::Minute);

        println!("{}", d);

        let d = date_time.format_date(CandleType::Hour);

        println!("{}", d);

        let d = date_time.format_date(CandleType::Day);

        println!("{}", d);

        let d = date_time.format_date(CandleType::Month);

        println!("{}", d);
    }
}
