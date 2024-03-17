use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum BidOrAsk {
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
    fn new(price: f64) -> Price {
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
struct Order {
    bid_or_ask: BidOrAsk, 
    size: f64
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order{bid_or_ask, size}
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new()
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>
}

impl Orderbook {
    fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new()
        }
    }

    fn add_order(&mut self, price: Price, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Ask => {}
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

fn main() {
    let buy_order_from_alice = Order:: new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 3.45);

    let mut orderbook = Orderbook::new();
    orderbook.add_order(Price::new(4.4), buy_order_from_bob.clone());
    orderbook.add_order(Price::new(4.4), buy_order_from_bob);
    orderbook.add_order(Price::new(4.4), buy_order_from_alice);

    println!("{:?}", orderbook);
}
