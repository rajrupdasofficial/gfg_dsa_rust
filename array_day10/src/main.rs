use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn max_subarray_sum(arr: Vec<i32>) -> i64 {
        let mut maxh: i64 = 0;
        let mut maxf: i64 = i64::MIN;

        for &num in &arr {
            maxh = num as i64 + maxh.max(0);
            maxf = maxf.max(maxh);
        }

        maxf
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let input = lines.next().unwrap().unwrap();
        let arr: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let result = Solution::max_subarray_sum(arr);
        println!("{}", result);
        println!("~");
    }
}
