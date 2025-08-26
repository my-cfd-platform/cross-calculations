use chrono::{DateTime, Utc};

use crate::InstrumentId;

pub trait CrossCalculationsBidAsk{
    fn get_id(&self) -> &str;
    fn get_bid(&self) -> f64;
    fn get_ask(&self) -> f64;
    fn get_date(&self) -> DateTime<Utc>;
}

pub trait CrossCalculationsPriceSource {
    fn get_bid_ask(&self, id: &InstrumentId) -> Option<impl CrossCalculationsBidAsk>;
}