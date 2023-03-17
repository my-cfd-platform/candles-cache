use std::time::Duration;

use crate::CandleType;

#[derive(Debug, Clone)]
pub struct RotateSettings {
    pub minute: Option<Duration>,
    pub hour: Option<Duration>,
    pub day: Option<Duration>,
    pub month: Option<Duration>,
}

impl RotateSettings {
    pub fn get_target(&self, candle_type: &CandleType) -> Option<Duration> {
        match candle_type {
            CandleType::Minute => self.minute,
            CandleType::Hour => self.hour,
            CandleType::Day => self.day,
            CandleType::Month => self.month,
        }
    }
}
