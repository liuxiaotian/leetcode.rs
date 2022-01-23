use std::collections::BTreeMap;
use std::collections::HashMap;

/// Solution for [2034. Stock Price Fluctuation](https://leetcode-cn.com/problems/stock-price-fluctuation/)
pub struct StockPrice {
    max_timestamp: i32,
    time_price: HashMap<i32, i32>,
    prices: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    fn new() -> Self {
        StockPrice {
            max_timestamp: 0,
            time_price: HashMap::new(),
            prices: BTreeMap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.max_timestamp = self.max_timestamp.max(timestamp);
        if let Some(last_timestamp) = self.time_price.insert(timestamp, price) {
            *self.prices.entry(last_timestamp).or_insert(0) -= 1;
            if self.prices[&last_timestamp] <= 0 {
                self.prices.remove(&last_timestamp);
            }
        }
        *self.prices.entry(price).or_insert(0) += 1;
    }

    fn current(&self) -> i32 {
        self.time_price[&self.max_timestamp]
    }

    fn maximum(&self) -> i32 {
        *self.prices.iter().next_back().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.prices.iter().next().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut stock_price = StockPrice::new();
        stock_price.update(1, 10);
        stock_price.update(2, 5);
        assert_eq!(stock_price.current(), 5);
        assert_eq!(stock_price.maximum(), 10);
        stock_price.update(1, 3);
        assert_eq!(stock_price.maximum(), 5);
        stock_price.update(4, 2);
        assert_eq!(stock_price.minimum(), 2);
    }
}
