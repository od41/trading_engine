mod matching_engine;
use matching_engine::orderbook::{Order, BidOrAsk, Orderbook};
use matching_engine::engine::{TradingPair, MatchingEngine};

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 3.45);
    let sell_order_from_bob = Order::new(BidOrAsk::Ask, 34.45);

    let mut orderbook = Orderbook::new();
    orderbook.add_order(41.4, sell_order_from_bob);
    orderbook.add_order(4.4, buy_order_from_bob.clone());
    orderbook.add_order(75.4, buy_order_from_bob);
    orderbook.add_order(4.4, buy_order_from_alice);

    // println!("{:?}", orderbook);
    let mut engine = MatchingEngine::new();
    let trade_pair = TradingPair::new("BTC".to_string(), "USDT".to_string());
    let alt_trade_pair = TradingPair::new("ETH".to_string(), "EURT".to_string());

    engine.add_new_pair(trade_pair.clone());

    let my_buy_order = Order::new(BidOrAsk::Bid, 4.0);

    engine.place_limit_order(alt_trade_pair, 30000.0, my_buy_order).unwrap_err();

}
