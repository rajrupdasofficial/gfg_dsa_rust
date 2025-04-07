use std::io;

struct Solution;

impl Solution {
    fn max_product(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 0 {
            return 0;
        }

        let mut max_product = arr[0];
        let mut max_val = arr[0];
        let mut min_val = arr[0];

        for i in 1..n {
            if arr[i] < 0 {
                let temp = max_val;
                max_val = min_val;
                min_val = temp;
            }

            max_val = if arr[i] > max_val * arr[i] {
                arr[i]
            } else {
                max_val * arr[i]
            };

            min_val = if arr[i] < min_val * arr[i] {
                arr[i]
            } else {
                min_val * arr[i]
            };

            max_product = if max_product > max_val {
                max_product
            } else {
                max_val
            };
        }

        max_product
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
        let arr: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        let ans = Solution::max_product(arr);
        println!("{}", ans);
        println!("~");
    }
}
