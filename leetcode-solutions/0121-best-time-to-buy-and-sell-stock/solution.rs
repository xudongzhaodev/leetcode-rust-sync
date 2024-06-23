impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit:i32 = 0;
        let mut min_price:i32 = i32::MAX;

        for &price in prices.iter() {
            if price < min_price {
                min_price = price;
            } else {
                let profit = price - min_price;
                if profit > max_profit {
                    max_profit = profit;
                }
            }
        }
        max_profit
    }
}
