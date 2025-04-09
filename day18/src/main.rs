use std::io::{self, BufRead};

fn compute_lps(pat: &[u8]) -> Vec<usize> {
    let m = pat.len();
    let mut lps = vec![0; m];
    let mut len = 0;
    let mut i = 1;

    while i < m {
        if pat[i] == pat[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }

    lps
}

fn kmp_search(pat: &str, txt: &str) -> Vec<usize> {
    let pat_bytes = pat.as_bytes();
    let txt_bytes = txt.as_bytes();
    let m = pat_bytes.len();
    let n = txt_bytes.len();

    if m == 0 || n == 0 || m > n {
        return vec![];
    }

    let lps = compute_lps(pat_bytes);
    let mut result = Vec::new();

    let mut i = 0;
    let mut j = 0;

    while i < n {
        if pat_bytes[j] == txt_bytes[i] {
            i += 1;
            j += 1;
        }

        if j == m {
            result.push(i - j);
            j = lps[j - 1];
        } else if i < n && pat_bytes[j] != txt_bytes[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter_map(Result::ok);

    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let txt = lines.next().unwrap().trim().to_string();
        let pat = lines.next().unwrap().trim().to_string();

        let result = kmp_search(&pat, &txt);

        if result.is_empty() {
            println!("[]");
        } else {
            for index in result {
                print!("{} ", index);
            }
            println!();
        }
    }
}
