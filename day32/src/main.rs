use std::io::{self, BufRead};

fn kth_element(a: &[i32], b: &[i32], k: usize) -> i32 {
    let (n, m) = (a.len(), b.len());

    // Ensure a is the smaller array
    if n > m {
        return kth_element(b, a, k);
    }

    let mut low = if k > m { k - m } else { 0 };
    let mut high = if k < n { k } else { n };

    while low <= high {
        let cut1 = (low + high) / 2;
        let cut2 = k - cut1;

        let l1 = if cut1 == 0 { i32::MIN } else { a[cut1 - 1] };
        let l2 = if cut2 == 0 { i32::MIN } else { b[cut2 - 1] };
        let r1 = if cut1 == n { i32::MAX } else { a[cut1] };
        let r2 = if cut2 == m { i32::MAX } else { b[cut2] };

        if l1 <= r2 && l2 <= r1 {
            return if l1 > l2 { l1 } else { l2 };
        } else if l1 > r2 {
            high = cut1.saturating_sub(1);
        } else {
            low = cut1 + 1;
        }
    }

    -1
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read k
        let k: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

        // Read first array
        let a_line = lines.next().unwrap().unwrap();
        let a: Vec<i32> = a_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

        // Read second array
        let b_line = lines.next().unwrap().unwrap();
        let b: Vec<i32> = b_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

        let result = kth_element(&a, &b, k);
        println!("{}", result);
        println!("~");
    }
}
