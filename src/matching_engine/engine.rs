use std::collections::HashMap;

use super::orderbook::{Order, Orderbook};
use rust_decimal::Decimal;

/*
    BTC_USDT
    BTC => Base
    USDT => Quote
*/
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair {base, quote}
    }

    pub fn to_string(self) -> String {
        format!("{}_{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, Orderbook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_pair(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), Orderbook::new());
        println!("opening new orderbook: {:?}", pair.to_string());
    }

    pub fn place_limit_order(&mut self, pair: TradingPair, price: Decimal, order: Order) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_order(price, order);
                Ok(())
            }
            None => Err(format!("orderbook for trading pair {} doesn't exist", pair.to_string()))
        }
    }
}