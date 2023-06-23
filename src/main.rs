use rust_decimal_macros::dec;

use crate::matching_engine::{
    engine::{self, MatchingEngine, TradingPair},
    orderbook::{BidOrAsk, Order, OrderBook},
};

mod matching_engine;

fn main() {
    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    let eth_pair = TradingPair::new("ETH".to_string(), "USD".to_string());

    engine
        .place_limit_order(pair, dec!(10.000), buy_order)
        .unwrap();
}
