use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    CrossCalculationsBidAsk, CrossCalculationsCrossRate, CrossCalculationsError,
    CrossCalculationsPriceSource, InstrumentId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossPairDiffSideInnerType {
    Direct(InstrumentId),
    Reversed(InstrumentId),
}

impl CrossPairDiffSideInnerType {
    pub fn get_data(
        &self,
        price_src: &impl CrossCalculationsPriceSource,
    ) -> Result<(f64, f64, DateTime<Utc>, InstrumentId), CrossCalculationsError> {
        match self {
            CrossPairDiffSideInnerType::Direct(src) => {
                let src_bid_ask = price_src.get_bid_ask(src).ok_or(
                    CrossCalculationsError::FailedToFindSourceBidAsk(src.get_source().to_string()),
                )?;

                Ok((
                    src_bid_ask.get_bid(),
                    src_bid_ask.get_ask(),
                    src_bid_ask.get_date(),
                    src.clone(),
                ))
            }
            CrossPairDiffSideInnerType::Reversed(src) => {
                let src_bid_ask = price_src.get_bid_ask(src).ok_or(
                    CrossCalculationsError::FailedToFindSourceBidAsk(src.get_source().to_string()),
                )?;

                Ok((
                    1.0 / src_bid_ask.get_ask(),
                    1.0 / src_bid_ask.get_bid(),
                    src_bid_ask.get_date(),
                    src.clone(),
                ))
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPairDiffSideType {
    pub left: CrossPairDiffSideInnerType,
    pub right: CrossPairDiffSideInnerType,
}

impl CrossPairDiffSideType {
    pub fn calculate_cross(
        &self,
        base: &str,
        quote: &str,
        with_source: bool,
        price_src: &impl CrossCalculationsPriceSource,
    ) -> Result<CrossCalculationsCrossRate, CrossCalculationsError> {
        let (left_bid, left_ask, left_date, left_src) = self.left.get_data(price_src)?;
        let (right_bid, right_ask, right_date, right_src) = self.right.get_data(price_src)?;

        Ok(CrossCalculationsCrossRate {
            base: base.to_string(),
            quote: quote.to_string(),
            bid: left_bid * right_bid,
            ask: left_ask * right_ask,
            date: right_date.max(left_date),
            source: match with_source {
                true => Some((left_src, right_src)),
                false => None,
            },
        })
    }
}
