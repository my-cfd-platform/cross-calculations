use cross_calculations_core::{get_cross_rate, CrossCalculationsCrossPairsMatrix};

use crate::*;

#[test]
fn test_single_pair_cross_2() {
    let bid_ask = CrossCalculationsCache::new(get_test_data_2());
    let instruments = get_test_instruments2();
    let crosses = [("BTC", "ETH"), ("ETH", "BTC")];

    let matrix = CrossCalculationsCrossPairsMatrix::new(&crosses, &instruments.iter().collect::<Vec<_>>()).unwrap();

    let first_cross = get_cross_rate("BTC", "ETH", &matrix, &bid_ask, true).unwrap();
    let second_cross = get_cross_rate("ETH", "BTC", &matrix, &bid_ask, true).unwrap();

    assert_eq!(format!("{:.1}", first_cross.bid), "0.2");
    assert_eq!(format!("{:.1}", first_cross.ask), "0.2");

    assert_eq!(format!("{:.1}", second_cross.bid), "5.0");
    assert_eq!(format!("{:.1}", second_cross.ask), "5.0");
}

#[test]
fn test_single_pair_cross_3() {
    let bid_ask = CrossCalculationsCache::new(get_test_data_3());
    let instruments = get_test_instruments3();
    let crosses = [("BTC", "ETH"), ("ETH", "BTC")];

    let matrix = CrossCalculationsCrossPairsMatrix::new(&crosses, &instruments.iter().collect::<Vec<_>>()).unwrap();

    let first_cross = get_cross_rate("BTC", "ETH", &matrix, &bid_ask, true).unwrap();
    let second_cross = get_cross_rate("ETH", "BTC", &matrix, &bid_ask, true).unwrap();

    assert_eq!(format!("{:.1}", first_cross.bid), "50000.0");
    assert_eq!(format!("{:.1}", first_cross.ask), "50000.0");

    assert_eq!(format!("{:.1}", second_cross.bid), "5.0");
    assert_eq!(format!("{:.1}", second_cross.ask), "5.0");
}
