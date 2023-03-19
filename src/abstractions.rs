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
}
