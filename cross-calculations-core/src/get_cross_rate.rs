use chrono::{DateTime, Utc};

use crate::{
    CrossCalculationsCrossPairsMatrix, CrossCalculationsError,
    CrossCalculationsPriceSource, InstrumentId,
};

#[derive(Debug, Clone)]
pub struct CrossCalculationsCrossRate {
    pub base: String,
    pub quote: String,
    pub bid: f64,
    pub ask: f64,
    pub date: DateTime<Utc>,
    pub source: Option<(InstrumentId, InstrumentId)>,
}

pub fn get_cross_rate(
    base: &str,
    quote: &str,
    matrix: &CrossCalculationsCrossPairsMatrix,
    price_src: &impl CrossCalculationsPriceSource,
    with_source: bool,
) -> Result<CrossCalculationsCrossRate, CrossCalculationsError> {
    let target_pair = matrix.get_target_cross(base, quote).ok_or(
        CrossCalculationsError::CrossPairNotFoundInMatrix(format!("{}{}", base, quote)),
    )?;

    match &target_pair._type {
        crate::CrossPairType::CrossPairSameSideType(src) => {
            src.calculate_cross(base, quote, with_source, price_src)
        }
        crate::CrossPairType::DiffSide(src) => {
            src.calculate_cross(base, quote, with_source, price_src)
        }
    }
}
