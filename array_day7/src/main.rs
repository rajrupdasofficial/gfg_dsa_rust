use std::io;

struct Solution;

impl Solution {
    fn maximum_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let n = prices.len();

        for i in 1..n {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }

        profit
    }
}

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t: usize = t.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let prices: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        let res = Solution::maximum_profit(prices);
        println!("{}", res);
        println!("~");
    }
}
