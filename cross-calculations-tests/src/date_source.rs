use chrono::Utc;

use crate::{CrossCalculationTestBidAsk, CrossCalculationsTestInstrument};

pub fn get_test_data() -> Vec<CrossCalculationTestBidAsk> {
    vec![
        CrossCalculationTestBidAsk {
            id: "EURUSD".to_string(),
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            bid: 1.1,
            ask: 1.2,
            date: Utc::now(),
        },
        CrossCalculationTestBidAsk {
            id: "USDJPY".to_string(),
            base: "USD".to_string(),
            quote: "JPY".to_string(),
            bid: 110.0,
            ask: 111.0,
            date: Utc::now(),
        },
        CrossCalculationTestBidAsk {
            id: "GBPUSD".to_string(),
            base: "GBP".to_string(),
            quote: "USD".to_string(),
            bid: 1.3,
            ask: 1.4,
            date: Utc::now(),
        },
    ]
}

pub fn get_test_instruments() -> Vec<CrossCalculationsTestInstrument> {
    vec![
        CrossCalculationsTestInstrument {
            id: "EURUSD".to_string(),
            base: "EUR".to_string(),
            quote: "USD".to_string(),
        },
        CrossCalculationsTestInstrument {
            id: "USDJPY".to_string(),
            base: "USD".to_string(),
            quote: "JPY".to_string(),
        },
        CrossCalculationsTestInstrument {
            id: "GBPUSD".to_string(),
            base: "GBP".to_string(),
            quote: "USD".to_string(),
        },
    ]
}


pub fn get_test_data_2() -> Vec<CrossCalculationTestBidAsk> {
    vec![
        CrossCalculationTestBidAsk {
            id: "BTCUSD".to_string(),
            base: "BTC".to_string(),
            quote: "USD".to_string(),
            bid: 100.0,
            ask: 100.0,
            date: Utc::now(),
        },
        CrossCalculationTestBidAsk {
            id: "ETHUSD".to_string(),
            base: "ETH".to_string(),
            quote: "USD".to_string(),
            bid: 500.0,
            ask: 500.0,
            date: Utc::now(),
        },
    ]
}


pub fn get_test_instruments2() -> Vec<CrossCalculationsTestInstrument> {
    vec![
        CrossCalculationsTestInstrument {
            id: "BTCUSD".to_string(),
            base: "BTC".to_string(),
            quote: "USD".to_string(),
        },
        CrossCalculationsTestInstrument {
            id: "ETHUSD".to_string(),
            base: "ETH".to_string(),
            quote: "USD".to_string(),
        },
    ]
}



pub fn get_test_data_3() -> Vec<CrossCalculationTestBidAsk> {
    vec![
        CrossCalculationTestBidAsk {
            id: "BTCUSD".to_string(),
            base: "BTC".to_string(),
            quote: "USD".to_string(),
            bid: 100.0,
            ask: 100.0,
            date: Utc::now(),
        },
        CrossCalculationTestBidAsk {
            id: "USDETH".to_string(),
            base: "USD".to_string(),
            quote: "ETH".to_string(),
            bid: 500.0,
            ask: 500.0,
            date: Utc::now(),
        },
    ]
}


pub fn get_test_instruments3() -> Vec<CrossCalculationsTestInstrument> {
    vec![
        CrossCalculationsTestInstrument {
            id: "BTCUSD".to_string(),
            base: "BTC".to_string(),
            quote: "USD".to_string(),
        },
        CrossCalculationsTestInstrument {
            id: "USDETH".to_string(),
            base: "USD".to_string(),
            quote: "ETH".to_string(),
        },
    ]
}


pub fn get_test_data_4() -> Vec<CrossCalculationTestBidAsk> {
    vec![
        CrossCalculationTestBidAsk {
            id: "EURTRY".to_string(),
            base: "EUR".to_string(),
            quote: "TRY".to_string(),
            bid: 35.96553,
            ask: 36.05,
            date: Utc::now(),
        },
        CrossCalculationTestBidAsk {
            id: "EURGBP".to_string(),
            base: "EUR".to_string(),
            quote: "GBP".to_string(),
            bid: 0.83962,
            ask: 0.83992,
            date: Utc::now(),
        },
    ]
}


pub fn get_test_instruments4() -> Vec<CrossCalculationsTestInstrument> {
    vec![
        CrossCalculationsTestInstrument {
            id: "EURTRY".to_string(),
            base: "EUR".to_string(),
            quote: "TRY".to_string(),
        },
        CrossCalculationsTestInstrument {
            id: "EURGBP".to_string(),
            base: "EUR".to_string(),
            quote: "GBP".to_string(),
        },
    ]
}