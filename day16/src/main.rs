use std::io::{self, BufRead};

fn are_anagrams(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut freq1 = [0; 26];
    let mut freq2 = [0; 26];

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        freq1[(c1 as u8 - b'a') as usize] += 1;
        freq2[(c2 as u8 - b'a') as usize] += 1;
    }

    freq1 == freq2
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let s1 = lines.next().unwrap().unwrap();
        let s2 = lines.next().unwrap().unwrap();

        // Check if they are anagrams and print the result
        if are_anagrams(&s1, &s2) {
            println!("true");
        } else {
            println!("false");
        }

        println!("~");
    }
}
