use std::io::{self, BufRead};

fn sort012(arr: &mut [i32]) {
    let mut low = 0;
    let mut mid = 0;

    let mut high = if arr.is_empty() { 0 } else { arr.len() - 1 };

    while mid <= high {
        match arr[mid] {
            0 => {
                arr.swap(low, mid);
                low += 1;
                mid += 1;
            }
            1 => {
                mid += 1;
            }
            2 => {
                arr.swap(mid, high);

                if high == 0 {
                    break;
                }
                high -= 1;
            }
            _ => {
                mid += 1;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines
        .next()
        .expect("Expected number of test cases")
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid number");

    for _ in 0..t {
        let line = lines.next().expect("Expected test case input").unwrap();

        let mut nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid integer"))
            .collect();

        sort012(&mut nums);

        for num in &nums {
            print!("{} ", num);
        }
        println!();

        println!("~");
    }
}
