use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn search(arr: &Vec<i32>, key: i32) -> i32 {
        for (i, &val) in arr.iter().enumerate() {
            if val == key {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read the line with the array elements
        let input_line = lines.next().unwrap().unwrap();
        let arr: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        // Read the key to search
        let key_line = lines.next().unwrap().unwrap();
        let key: i32 = key_line.trim().parse().unwrap();

        let result = Solution::search(&arr, key);
        println!("{}", result);
        println!("~");
    }
}
