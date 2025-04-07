use std::io::{self, BufRead};

fn add_binary(s1: &str, s2: &str) -> String {
    let mut i = s1.len() - 1;
    let mut j = s2.len() - 1;
    let mut carry = 0;
    let mut result = String::new();

    while i > 0 || j > 0 || carry > 0 {
        let mut sum = carry;
        if i > 0 {
            sum += (s1.as_bytes()[i] - b'0') as i32;
            i -= 1;
        }
        if j > 0 {
            sum += (s2.as_bytes()[j] - b'0') as i32;
            j -= 1;
        }
        result.push((sum % 2 + b'0') as char);
        carry = sum / 2;
    }

    result = result.chars().rev().collect();

    let first_non_zero = result.find(|c| c != '0');
    match first_non_zero {
        Some(index) => result[index..].to_string(),
        None => "0".to_string(),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let s1 = parts.next().unwrap();
        let s2 = parts.next().unwrap();

        let result = add_binary(s1, s2);
        println!("{}", result);
        println!("~");
    }
}
