use std::io::{self, BufRead};

fn reverse(arr: &mut Vec<i32>, start: usize, end: usize) {
    let mut start = start;
    let mut end = end;
    while start < end {
        arr.swap(start, end);
        start += 1;
        end -= 1;
    }
}

fn rotate_arr(arr: &mut Vec<i32>, d: usize) {
    let n = arr.len();
    let d = d % n;
    if d == 0 {
        return;
    }
    reverse(arr, 0, n - 1);
    reverse(arr, 0, n - d - 1);
    reverse(arr, n - d, n - 1);
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read the array
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Read number of rotations
        let d: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        let mut arr = nums.clone(); // Create a mutable copy of the array
        rotate_arr(&mut arr, d); // Rotate the array

        // Print the rotated array
        println!(
            "{}",
            arr.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
