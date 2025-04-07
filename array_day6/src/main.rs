use std::io::{self, BufRead};

fn find_majority(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    let mut count1 = 0;
    let mut count2 = 0;
    let mut candidate1 = None;
    let mut candidate2 = None;

    for &num in arr {
        if Some(num) == candidate1 {
            count1 += 1;
        } else if Some(num) == candidate2 {
            count2 += 1;
        } else if count1 == 0 {
            candidate1 = Some(num);
            count1 = 1;
        } else if count2 == 0 {
            candidate2 = Some(num);
            count2 = 1;
        } else {
            count1 -= 1;
            count2 -= 1;
        }
    }

    let mut result = Vec::new();
    count1 = 0;
    count2 = 0;

    for &num in arr {
        if Some(num) == candidate1 {
            count1 += 1;
        } else if Some(num) == candidate2 {
            count2 += 1;
        }
    }

    if count1 > n / 3 {
        result.push(candidate1.unwrap());
    }
    if count2 > n / 3 {
        result.push(candidate2.unwrap());
    }

    result.sort();
    result
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();

    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        // Read the line containing numbers
        let line = lines.next().unwrap().unwrap();

        // Parse integers from the line
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Find majority elements
        let result = find_majority(&nums);

        // Print results
        if result.is_empty() {
            println!("[]");
            println!("~");
        } else {
            println!("{:?}", result);
            println!("~");
        }
    }
}
