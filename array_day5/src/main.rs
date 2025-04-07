use std::io::{self, BufRead};

fn reverse(arr: &mut [i32], start: usize, end: usize) {
    let mut i = start;
    let mut j = end;
    while i < j {
        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn next_permutation(arr: &mut Vec<i32>) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    // Step 1: Find the first index 'i' from the right such that arr[i] < arr[i+1]
    let mut i = n.wrapping_sub(2);
    while i < n && arr[i] >= arr[i + 1] {
        if i == 0 {
            break;
        }
        i -= 1;
    }

    // If we found such an index and it satisfies arr[i] < arr[i+1]
    if i < n && arr[i] < arr[i + 1] {
        // Step 2: Find the largest index 'j' to the right of 'i' such that arr[j] > arr[i]
        let mut j = n - 1;
        while arr[j] <= arr[i] {
            j -= 1;
        }
        arr.swap(i, j);
        // Step 3: Reverse the subarray from i+1 to end
        reverse(arr, i + 1, n - 1);
    } else {
        // If no such index exists, reverse the entire array to get the lowest order.
        reverse(arr, 0, n - 1);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line which represents the number of test cases.
    let t_line = lines.next().unwrap().unwrap();
    let t: usize = t_line.trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        // Parse the numbers in the current test case
        let mut nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        next_permutation(&mut nums);

        // Print the permuted array
        let output = nums
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", output);
        println!("~");
    }
}
