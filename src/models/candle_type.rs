#[derive(Debug, Clone, Copy)]
pub enum CandleType {
    Minute = 0,
    Hour = 1,
    Day = 2,
    Month = 3,
}

impl CandleType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Minute,
            1 => Self::Hour,
            2 => Self::Day,
            3 => Self::Month,
            _ => panic!("Invalid candle type"),
        }
    }
}