use serde::{Deserialize, Serialize};

use crate::CrossPairSameSideType;

use super::cross_pair_diff_side::CrossPairDiffSideType;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstrumentId(pub String);

impl InstrumentId {
    pub fn get_source(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPair {
    pub base: String,
    pub quote: String,
    pub _type: CrossPairType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossPairType {
    CrossPairSameSideType(CrossPairSameSideType),
    DiffSide(CrossPairDiffSideType),
}

pub trait CrossCalculationsSourceInstrument {
    fn get_id(&self) -> &str;
    fn get_base(&self) -> &str;
    fn get_quote(&self) -> &str;
}
