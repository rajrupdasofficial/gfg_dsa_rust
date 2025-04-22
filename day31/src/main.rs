use std::io::{self, BufRead};

fn peak_element(arr: &[i32]) -> usize {
    let mut lo = 0;
    let mut hi = arr.len() - 1;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] > arr[mid + 1] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read a line of input representing the array elements
        let line = lines.next().unwrap().unwrap();
        let arr: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

        let n = arr.len();
        if n == 0 {
            println!("false");
            println!("~");
            continue;
        }

        let idx = peak_element(&arr);

        let f = if idx >= n {
            false
        } else if n == 1 && idx == 0 {
            true
        } else if idx == 0 && arr[0] > arr[1] {
            true
        } else if idx == n - 1 && arr[n - 1] > arr[n - 2] {
            true
        } else if idx > 0 && idx < n - 1 && arr[idx] > arr[idx + 1] && arr[idx] > arr[idx - 1] {
            true
        } else {
            false
        };

        println!("{}", if f { "true" } else { "false" });
        println!("~");
    }
}
