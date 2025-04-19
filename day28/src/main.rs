use std::io::{self, BufRead};

fn count_freq(arr: &[i32], target: i32) -> usize {
    arr.iter().filter(|&&x| x == target).count()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: usize = lines
        .next()
        .expect("Expected number of test cases")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse number of test cases");

    for _ in 0..t {
        // Read a line of integers
        let line = lines
            .next()
            .expect("Expected array line")
            .expect("Failed to read line");
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Parse error"))
            .collect();

        // Read the target integer
        let x_line = lines
            .next()
            .expect("Expected target integer")
            .expect("Failed to read line");
        let x: i32 = x_line
            .trim()
            .parse()
            .expect("Failed to parse target integer");

        let ans = count_freq(&nums, x);
        println!("{}", ans);
        println!("~");
    }
}
