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
            _ => panic!("Invalid candle type {}", value),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            CandleType::Minute => 0u8,
            CandleType::Hour => 1u8,
            CandleType::Day => 2u8,
            CandleType::Month => 3u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CandleType;

    #[test]
    fn tests() {
        let src = 0;
        let ct = CandleType::from_u8(src);
        assert_eq!(ct.to_u8(), src);
    }
}
