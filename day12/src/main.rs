use std::io::{self, BufRead};

fn circular_subarray_sum(arr: Vec<i32>) -> i32 {
    let mut total_sum = 0;
    let mut max_sum = i32::MIN;
    let mut min_sum = i32::MAX;
    let mut curr_max = 0;
    let mut curr_min = 0;
    let mut all_negative = true;

    for num in arr {
        total_sum += num;
        curr_max = curr_max.max(num).max(curr_max + num);
        max_sum = max_sum.max(curr_max);
        curr_min = curr_min.min(num).min(curr_min + num);
        min_sum = min_sum.min(curr_min);
        if num > 0 {
            all_negative = false;
        }
    }

    if all_negative {
        max_sum
    } else {
        max_sum.max(total_sum - min_sum)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let arr: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let res = circular_subarray_sum(arr);
        println!("{}", res);
        println!("~");
    }
}
