use std::io::{self, BufRead};

/// Checks if `s2` is a rotation of `s1`.
fn are_rotations(s1: &str, s2: &str) -> bool {
    // If lengths are different, s2 cannot be a rotation of s1.
    if s1.len() != s2.len() {
        return false;
    }
    // Create a temporary string by concatenating s1 with itself.
    let temp = format!("{}{}", s1, s1);
    // Check if s2 is a substring of the concatenated string.
    temp.contains(s2)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases.
    let t: usize = lines
        .next()
        .expect("Expected number of test cases")
        .unwrap()
        .trim()
        .parse()
        .expect("Invalid number");

    for _ in 0..t {
        // Read the two strings for the current test case.
        let s1 = lines
            .next()
            .expect("Expected first string")
            .unwrap()
            .trim()
            .to_string();
        let s2 = lines
            .next()
            .expect("Expected second string")
            .unwrap()
            .trim()
            .to_string();

        // Check for rotations and output the result.
        if are_rotations(&s1, &s2) {
            println!("true");
        } else {
            println!("false");
        }

        // Display the result separator.
        println!("~");
    }
}
