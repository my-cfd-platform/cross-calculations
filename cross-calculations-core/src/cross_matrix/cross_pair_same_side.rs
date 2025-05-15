use serde::{Deserialize, Serialize};

use crate::{
    CrossCalculationsBidAsk, CrossCalculationsCrossRate, CrossCalculationsError,
    CrossCalculationsPriceSource, InstrumentId,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CrossPairSameSideType {
    pub left_side_match_asset: bool,
    pub left: InstrumentId,
    pub right: InstrumentId,
}

impl CrossPairSameSideType {
    pub fn calculate_cross(
        &self,
        base: &str,
        quote: &str,
        with_source: bool,
        price_src: &impl CrossCalculationsPriceSource,
    ) -> Result<CrossCalculationsCrossRate, CrossCalculationsError> {
        let left_bid_ask = price_src.get_bid_ask(&self.left).ok_or(
            CrossCalculationsError::FailedToFindSourceBidAsk(self.left.get_source().to_string()),
        )?;
        let right_bid_ask = price_src.get_bid_ask(&self.right).ok_or(
            CrossCalculationsError::FailedToFindSourceBidAsk(self.right.get_source().to_string()),
        )?;

        let source = if with_source {
            Some((self.left.clone(), self.right.clone()))
        } else {
            None
        };

        let date = left_bid_ask.get_date().max(right_bid_ask.get_date());

        let (bid, ask) = match self.left_side_match_asset {
            true => {
                let bid = 1.0 / left_bid_ask.get_ask() * right_bid_ask.get_bid();
                let ask = 1.0 / left_bid_ask.get_bid() * right_bid_ask.get_ask();

                (bid, ask)
            }
            false => (
                left_bid_ask.get_bid() / right_bid_ask.get_ask(),
                left_bid_ask.get_ask() / right_bid_ask.get_bid(),
            ),
        };

        Ok(CrossCalculationsCrossRate {
            base: base.to_string(),
            quote: quote.to_string(),
            bid,
            ask,
            source,
            date,
        })
    }
}
