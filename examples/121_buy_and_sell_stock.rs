//! You want to maximize your profit by choosing a single day to buy one stock and choosing a different day
//! in the future to sell that stock.
//! Return the maximum profit you can achieve from this transaction.
//! If you cannot achieve any profit, return 0.

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut left = 0; // Buy -> Always track the minimum price
        let mut right = 0; // Sell
        let mut profit = 0;

        for right in 1..prices.len() {
            if prices[right] > prices[left] {
                let current_profit = prices[right] - prices[left];
                profit = current_profit.max(profit);
            } else {
                left = right;
            }
        }
        profit
    }
}

fn main() {
    let prices = vec![2, 4, 1, 2, 5];
    dbg!(Solution::max_profit(prices));
}
