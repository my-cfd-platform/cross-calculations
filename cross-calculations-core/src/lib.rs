mod dto;
mod cross_matrix;
mod get_cross_rate;

pub use dto::*;
pub use cross_matrix::*;
pub use get_cross_rate::*;


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossCalculationsError {
    FailedToGenerateCross(String),
    CrossPairNotFoundInMatrix(String),
    FailedToFindAssetForCrossCalculate(String),
    FailedToCalculateCross(String),
    FailedToFindSourceBidAsk(String),
}