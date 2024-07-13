use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{CrossCalculationTestBidAsk, CrossCalculationsTestInstrument};

pub fn get_test_data() -> Vec<CrossCalculationTestBidAsk> {
    vec![
        CrossCalculationTestBidAsk {
            id: "EURUSD".to_string(),
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            bid: 1.1,
            ask: 1.2,
            date: DateTimeAsMicroseconds::from(123456_i64),
        },
        CrossCalculationTestBidAsk {
            id: "USDJPY".to_string(),
            base: "USD".to_string(),
            quote: "JPY".to_string(),
            bid: 110.0,
            ask: 111.0,
            date: DateTimeAsMicroseconds::from(123457_i64),
        },
        CrossCalculationTestBidAsk {
            id: "GBPUSD".to_string(),
            base: "GBP".to_string(),
            quote: "USD".to_string(),
            bid: 1.3,
            ask: 1.4,
            date: DateTimeAsMicroseconds::from(123458_i64),
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
            date: DateTimeAsMicroseconds::from(123456_i64),
        },
        CrossCalculationTestBidAsk {
            id: "ETHUSD".to_string(),
            base: "ETH".to_string(),
            quote: "USD".to_string(),
            bid: 500.0,
            ask: 500.0,
            date: DateTimeAsMicroseconds::from(123457_i64),
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
            date: DateTimeAsMicroseconds::from(123456_i64),
        },
        CrossCalculationTestBidAsk {
            id: "USDETH".to_string(),
            base: "USD".to_string(),
            quote: "ETH".to_string(),
            bid: 500.0,
            ask: 500.0,
            date: DateTimeAsMicroseconds::from(123457_i64),
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