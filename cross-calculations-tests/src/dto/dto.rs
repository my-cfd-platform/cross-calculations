use std::collections::HashMap;

use chrono::{DateTime, Utc};
use cross_calculations_core::{
    CrossCalculationsBidAsk, CrossCalculationsPriceSource, CrossCalculationsSourceInstrument,
};

pub struct CrossCalculationTestBidAsk {
    pub id: String,
    pub bid: f64,
    pub ask: f64,
    pub date: DateTime<Utc>,
    pub base: String,
    pub quote: String,
}

impl CrossCalculationsBidAsk for CrossCalculationTestBidAsk {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_bid(&self) -> f64 {
        self.bid
    }

    fn get_ask(&self) -> f64 {
        self.ask
    }

    fn get_date(&self) -> DateTime<Utc> {
        self.date
    }
}

pub struct CrossCalculationsCache {
    pub cache: HashMap<String, CrossCalculationTestBidAsk>,
}

impl CrossCalculationsCache {
    pub fn new(src: Vec<CrossCalculationTestBidAsk>) -> Self {
        let mut result = HashMap::new();

        for item in src {
            result.insert(item.id.clone(), item);
        }

        Self { cache: result }
    }
}

impl CrossCalculationsPriceSource for CrossCalculationsCache {
    fn get_bid_ask(
        &self,
        id: &cross_calculations_core::InstrumentId,
    ) -> Option<&impl CrossCalculationsBidAsk> {
        self.cache.get(id.get_source())
    }
}

pub struct CrossCalculationsTestInstrument {
    pub id: String,
    pub base: String,
    pub quote: String,
}

impl CrossCalculationsSourceInstrument for CrossCalculationsTestInstrument {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_base(&self) -> &str {
        &self.base
    }

    fn get_quote(&self) -> &str {
        &self.quote
    }
}
