use std::io::{self, BufRead};

fn reverse_array(arr: &mut Vec<i32>) {
    let mut start = 0;
    let mut end = arr.len() - 1;

    while start < end {
        arr.swap(start, end);
        start += 1;
        end -= 1;
    }
}
fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        reader.read_line(&mut line).unwrap();

        // Split the input line into integers
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Reverse the array using the defined function
        let mut brr = nums.clone(); // Clone nums into brr
        reverse_array(&mut brr);

        // Output the reversed array
        for (i, &num) in brr.iter().enumerate() {
            if i == brr.len() - 1 {
                println!("{}", num);
            } else {
                print!("{} ", num);
            }
        }

        println!("~");
    }
}
