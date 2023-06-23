use std::{collections::HashMap, fmt::Display};

use rust_decimal::Decimal;

use super::orderbook::{Order, OrderBook};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl Display for TradingPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}_{})", self.base, self.quote)
    }
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("Opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: Decimal,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);

                println!("placing limit order at price level {:?}", price);

                Ok(())
            }
            None => Err(format!(
                "the orderbook for given trading pair ({}) doesnt exist",
                pair.to_string()
            )),
        }
    }
}
