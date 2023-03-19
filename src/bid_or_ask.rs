#[derive(Debug, Clone, Copy)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

impl BidOrAsk {
    pub fn from_is_bid(is_bid: bool) -> Self {
        if is_bid {
            Self::Bid
        } else {
            Self::Ask
        }
    }

    pub fn to_is_bid(&self) -> bool {
        match self {
            Self::Bid => true,
            Self::Ask => false,
        }
    }
}
