use chrono::Duration;

use crate::CandleType;

#[derive(Debug, Clone)]
pub struct RotateSettigns {
    pub minite: Option<Duration>,
    pub hour: Option<Duration>,
    pub day: Option<Duration>,
    pub mounth: Option<Duration>,
}

impl RotateSettigns {
    pub fn get_target(&self, candle_type: &CandleType) -> Option<Duration> {
        match candle_type {
            CandleType::Minute => self.minite,
            CandleType::Hour => self.hour,
            CandleType::Day => self.day,
            CandleType::Mounth => self.mounth,
        }
    }
}
