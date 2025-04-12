use std::io::{self, BufRead};

fn compute_lps_array(pat: &[u8]) -> Vec<usize> {
    let m = pat.len();
    let mut lps = vec![0; m];
    let mut len = 0;
    let mut i = 1;

    while i < m {
        if pat[i] == pat[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    lps
}

fn min_char(s: &str) -> usize {
    let s = s.trim();
    let n = s.len();

    let rev_str: String = s.chars().rev().collect();

    let combined = format!("{}${}", s, rev_str);
    let lps = compute_lps_array(combined.as_bytes());
    n - lps.last().unwrap()
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
        // Read each test string.
        let s = lines.next().expect("Expected a test case").unwrap();

        let result = min_char(&s);
        println!("{}", result);
        // Display the result separator.
        println!("~");
    }
}
