use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn push_zeros_to_end(arr: &mut Vec<i32>) {
        let mut non_zero_index = 0;

        for i in 0..arr.len() {
            if arr[i] != 0 {
                arr.swap(non_zero_index, i);
                non_zero_index += 1;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Declare `reader` as mutable

    let mut input = String::new();
    reader.read_line(&mut input).unwrap(); // Use mutable borrow
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear(); // Clear the buffer for the next line
        reader.read_line(&mut input).unwrap(); // Use mutable borrow

        let mut arr: Vec<i32> = input
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        Solution::push_zeros_to_end(&mut arr);

        println!(
            "{}",
            arr.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
