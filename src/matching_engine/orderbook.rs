use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum BidOrAsk {
    Ask,
    Bid
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Price {
    fractional: u64,
    integral: u64,
    scalar: u64
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 10000;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        let integral = price as u64;
        Price {
            integral,
            fractional,
            scalar
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Order {
    bid_or_ask: BidOrAsk, 
    size: f64
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order{bid_or_ask, size}
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new()
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new()
        }
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);
        match order.bid_or_ask {
            BidOrAsk::Ask => {
                match self.asks.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.asks.insert(price, limit);
                    }
                }
            }
            BidOrAsk::Bid => {
                match self.bids.get_mut(&price) {
                    Some(limit) => limit.add_order(order),
                    None => {
                        let mut limit = Limit::new(price);
                        limit.add_order(order);
                        self.bids.insert(price, limit);
                    }
                }
            }
        }
    }
}