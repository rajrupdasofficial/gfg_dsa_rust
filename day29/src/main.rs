use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn find_min(arr: &[i32]) -> i32 {
        let mut lo = 0;
        let mut hi = arr.len() - 1;

        while lo < hi {
            if arr[lo] < arr[hi] {
                return arr[lo];
            }
            let mid = lo + (hi - lo) / 2;
            if arr[mid] > arr[hi] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        arr[lo]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read a line of integers
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

        let result = Solution::find_min(&nums);
        println!("{}", result);
        println!("~");
    }
}
