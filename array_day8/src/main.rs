struct Solution;

impl Solution {
    fn maximum_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut buy_price = prices[0];
        let mut max_profit = 0;

        for &price in prices.iter().skip(1) {
            if price > buy_price {
                max_profit = max_profit.max(price - buy_price);
            } else {
                buy_price = price;
            }
        }

        max_profit
    }
}

fn main() {
    let mut t = String::new();
    std::io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line");
    let t: usize = t.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let prices: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        let ans = Solution::maximum_profit(prices);
        println!("{}", ans);
    }
}
