#![allow(dead_code)]
use std::collections::HashMap;
use rust_decimal::*;

#[derive(Debug, Copy, Clone)]
pub enum BidOrAsk {
    Ask,
    Bid,
}

#[derive(Debug, Copy, Clone)]
pub struct Order {
    bid_or_ask: BidOrAsk,
    size: f64,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

#[derive(Debug)]
struct Limit {
    price: Decimal,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Decimal) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    pub fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                // market_order size can't be filled by limit_order
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }

                // market_order can be filled by limit_order
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn total_volume(&self) -> f64 {
        self.orders
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap_or(0.0)
    }
}

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Decimal, Limit>,
    bids: HashMap<Decimal, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, price: Decimal, order: Order) {
        // let price = Decimal::new(price);
        match order.bid_or_ask {
            BidOrAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        match market_order.bid_or_ask {
            BidOrAsk::Ask => {
                // if you're filling a bid, you're matching 
                // the Bid (or buy) to an Ask (or sell)
                for limit_order in self.bid_limits() {
                    limit_order.fill_order(market_order);

                    if market_order.is_filled() {
                        break;
                    }
                }
            }
            BidOrAsk::Bid => {
                // if you're filling a bid, you're matching 
                // the Bid (or buy) to an Ask (or sell)
                for limit_order in self.ask_limits() {
                    limit_order.fill_order(market_order);

                    if market_order.is_filled() {
                        break;
                    }
                }
            }
        }
    }

    // TODO: return sorted vectors
    fn ask_limits(&mut self) -> Vec<&mut Limit> {
        self.asks.values_mut().collect::<Vec<&mut Limit>>()
    }

    // TODO: return sorted vectors
    fn bid_limits(&mut self) -> Vec<&mut Limit> {
        self.bids.values_mut().collect::<Vec<&mut Limit>>()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn total_volume() {
        let price = dec!(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);

        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        assert_eq!(limit.total_volume(), 200.0);
    }

    #[test]
    fn limit_order_single_fill() {
        let price = dec!(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);

        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 90.0);
        limit.fill_order(&mut market_sell_order);

        assert!(market_sell_order.is_filled());
        assert_eq!(limit.orders.get(0).unwrap().size, 10.0);
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = dec!(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order_a = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidOrAsk::Bid, 100.0);

        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 170.0);
        limit.fill_order(&mut market_sell_order);

        assert!(market_sell_order.is_filled());
        assert_eq!(limit.orders.get(0).unwrap().size, 0.0);
        assert_eq!(limit.orders.get(1).unwrap().size, 30.0);
    }
}
