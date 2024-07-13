use std::collections::HashMap;

use crate::{
    CrossCalculationsError, CrossCalculationsSourceInstrument, CrossPair, CrossPairSameSideType,
    CrossPairType, InstrumentId,
};

use super::cross_pair_diff_side::{CrossPairDiffSideInnerType, CrossPairDiffSideType};

#[derive(Debug, Clone)]
pub struct CrossCalculationsCrossPairsMatrix {
    pub pairs: HashMap<String, HashMap<String, CrossPair>>,
}

impl CrossCalculationsCrossPairsMatrix {
    pub fn new(
        requested_crosses: &[(&str, &str)],
        instruments: &[&impl CrossCalculationsSourceInstrument],
    ) -> Result<Self, CrossCalculationsError> {
        let mut result = HashMap::new();

        for (base, quote) in requested_crosses {
            let cross = find_cross_pair(base, quote, instruments)?;

            let base_map = result.entry(cross.base.clone()).or_insert(HashMap::new());
            base_map.entry(cross.quote.clone()).or_insert(cross);
        }

        Ok(Self { pairs: result })
    }

    pub fn get_target_cross(&self, base: &str, quote: &str) -> Option<&CrossPair> {
        self.pairs.get(base)?.get(quote)
    }
}

fn find_cross_pair(
    base: &str,
    quote: &str,
    instruments: &[&impl CrossCalculationsSourceInstrument],
) -> Result<CrossPair, CrossCalculationsError> {
    let base_contains_instruments = instruments
        .iter()
        .filter(|x| x.get_base() == base || x.get_quote() == base)
        .collect::<Vec<_>>();

    let quote_contains_instruments = instruments
        .iter()
        .filter(|x| x.get_base() == quote || x.get_quote() == quote)
        .collect::<Vec<_>>();

    for base_pair in &base_contains_instruments {
        for quote_pair in quote_contains_instruments.iter() {
            let to_check = [base_pair.get_base(), base_pair.get_quote()];
            if to_check.contains(&quote_pair.get_base())
                || to_check.contains(&quote_pair.get_quote())
            {
                let (left, right) =
                    match base_pair.get_base() == base || base_pair.get_quote() == base {
                        true => (base_pair, quote_pair),
                        false => (quote_pair, base_pair),
                    };

                let _type: CrossPairType = match base_pair.get_base() == quote_pair.get_base()
                    || base_pair.get_quote() == quote_pair.get_quote()
                {
                    true => CrossPairType::CrossPairSameSideType(CrossPairSameSideType {
                        left: InstrumentId(left.get_id().to_string()),
                        right: InstrumentId(right.get_id().to_string()),
                    }),
                    false => CrossPairType::DiffSide(CrossPairDiffSideType {
                        left: CrossPairDiffSideInnerType::Direct(InstrumentId(
                            left.get_id().to_string(),
                        )),
                        right: if left.get_quote() == right.get_base() {
                            CrossPairDiffSideInnerType::Direct(InstrumentId(
                                right.get_id().to_string(),
                            ))
                        } else {
                            CrossPairDiffSideInnerType::Reversed(InstrumentId(
                                right.get_id().to_string(),
                            ))
                        },
                    }),
                };

                return Ok(CrossPair {
                    base: base.to_string(),
                    quote: quote.to_string(),
                    _type,
                });
            }
        }
    }

    Err(CrossCalculationsError::FailedToGenerateCross(format!(
        "Failed to find cross for {} - {}",
        base, quote
    )))
}
