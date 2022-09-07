#[derive(Debug, Clone)]
pub enum CandleType{
    Minute = 1,
    Hour = 2,
    Day = 3,
    Month = 4
}

impl CandleType{
    pub fn format_date_by_type(&self, date: u64) -> u64{
        match self {
            CandleType::Minute => date - date % 60,
            CandleType::Hour => date - date % 3600,
            CandleType::Day => date - date % 86400,
            CandleType::Month => date - date % 2592000,
        }
    }
}