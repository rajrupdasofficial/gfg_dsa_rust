use std::cmp::min;
use std::io;

struct Solution;

impl Solution {
    fn get_min_diff(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut modified_heights: Vec<(i32, usize)> = Vec::new();
        let mut frequency = vec![0; n];

        for i in 0..n {
            if arr[i] - k >= 0 {
                modified_heights.push((arr[i] - k, i));
            }
            modified_heights.push((arr[i] + k, i));
        }

        modified_heights.sort_unstable();

        let mut left = 0;
        let mut right = 0;
        let mut covered = 0;
        let mut min_difference = i32::MAX;

        while right < modified_heights.len() {
            if frequency[modified_heights[right].1] == 0 {
                covered += 1;
            }
            frequency[modified_heights[right].1] += 1;

            while covered == n {
                min_difference = min(
                    min_difference,
                    modified_heights[right].0 - modified_heights[left].0,
                );

                frequency[modified_heights[left].1] -= 1;
                if frequency[modified_heights[left].1] == 0 {
                    covered -= 1;
                }
                left += 1;
            }
            right += 1;
        }

        min_difference
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
        let k: i32 = input.trim().parse().expect("Please type a number!");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let arr: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        let ans = Solution::get_min_diff(arr, k);
        println!("{}", ans);
        println!("~");
    }
}
